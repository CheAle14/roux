use std::error::Error;
use std::time::Duration;

use reqwest::{header, Method, StatusCode};
use serde::{Deserialize, Serialize};

use crate::client::ratelimit::Ratelimit;
use crate::client::req::*;
use crate::util::RouxError;
use crate::Config;

use super::endpoint::EndpointBuilder;

enum RetryableExecuteError {
    RetryAfter(Duration),
    RetryExponential {
        max_retries: Option<u8>,
        last_error: reqwest::Error,
    },
    Unauthorized,
    OtherResponseError(Response, reqwest::Error),
    Other(reqwest::Error),
}

impl From<reqwest::Error> for RetryableExecuteError {
    fn from(value: reqwest::Error) -> Self {
        Self::Other(value)
    }
}

pub(crate) enum ExecuteError {
    AuthorizationRequired,
    AuthError(String),
    ErrorOnly(reqwest::Error),
    ResponseAndError(Response, reqwest::Error),
}

impl From<reqwest::Error> for ExecuteError {
    fn from(value: reqwest::Error) -> Self {
        Self::ErrorOnly(value)
    }
}

impl From<ExecuteError> for RouxError {
    fn from(value: ExecuteError) -> Self {
        match value {
            ExecuteError::AuthorizationRequired => RouxError::credentials_not_set(),
            ExecuteError::ErrorOnly(error) => RouxError::network(error),
            ExecuteError::AuthError(error) => RouxError::auth(error),
            ExecuteError::ResponseAndError(response, error) => {
                RouxError::full_network(response, error)
            }
        }
    }
}

pub(crate) struct ClientInner {
    pub(crate) config: Config,
    base_url: &'static str,
    inner: Client,
    ratelimit: Mutex<Ratelimit>,
}

impl ClientInner {
    pub(crate) fn new(config: Config) -> Result<Self, RouxError> {
        let base_url = if config.password.is_some() {
            "https://oauth.reddit.com"
        } else {
            "https://www.reddit.com"
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&config.user_agent).unwrap(),
        );
        /*
        if let Some(access_token) = &config.access_token {
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
            );
        }
         */

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(Self {
            base_url,
            config,
            inner: client,
            ratelimit: Mutex::new(Ratelimit::new()),
        })
    }

    pub(crate) fn request(&self, method: Method, endpoint: &EndpointBuilder) -> RequestBuilder {
        let url = endpoint.build(&self.base_url);
        println!("[roux] {method:?} {url}");
        self.inner.request(method, url)
    }

    #[cfg(feature = "blocking")]
    pub(crate) fn with_ratelimits(&self, request: Request) -> Result<Response, reqwest::Error> {
        let mut lock = self.ratelimit.lock().unwrap();
        lock.delay();
        let response = self.inner.execute(request)?;
        lock.update(response.headers());
        Ok(response)
    }
    #[cfg(not(feature = "blocking"))]
    pub(crate) async fn with_ratelimits(
        &self,
        request: Request,
    ) -> Result<Response, reqwest::Error> {
        let mut lock = self.ratelimit.lock().await;
        lock.delay().await;
        let response = self.inner.execute(request).await?;
        lock.update(response.headers());
        Ok(response)
    }

    #[maybe_async::maybe_async]
    async fn convert_error(
        &self,
        response: super::req::Response,
        error: reqwest::Error,
    ) -> RetryableExecuteError {
        let status = error.status().unwrap_or(StatusCode::BAD_REQUEST);
        println!("[roux] Response error: {status:?}");
        match status {
            StatusCode::TOO_MANY_REQUESTS => {
                if let Some(value) = response.headers().get("Retry-After") {
                    if let Ok(value) = value.to_str() {
                        if let Ok(value) = value.parse() {
                            return RetryableExecuteError::RetryAfter(Duration::from_secs(value));
                        }
                    }
                }
                RetryableExecuteError::RetryExponential {
                    max_retries: None,
                    last_error: error,
                }
            }
            StatusCode::INTERNAL_SERVER_ERROR => RetryableExecuteError::RetryExponential {
                max_retries: Some(32),
                last_error: error,
            },
            StatusCode::UNAUTHORIZED => RetryableExecuteError::Unauthorized,
            _ => RetryableExecuteError::OtherResponseError(response, error),
        }
    }

    #[maybe_async::maybe_async]
    async fn inner_execute(&self, request: Request) -> Result<Response, RetryableExecuteError> {
        match self.with_ratelimits(request).await {
            Ok(response) => {
                // We did get a response from the server, but it may still be an error (e.g. bad request, etc)
                if let Err(e) = response.error_for_status_ref() {
                    let err = self.convert_error(response, e).await;
                    Err(err)
                } else {
                    Ok(response)
                }
            }
            Err(error) => {
                // We either did not get a response or it was malformed in some way.
                // Attempt to retry failures that could be intermittent, but fail eventually.
                if let Some(inner) = error.source() {
                    if let Some(err) = inner.downcast_ref::<std::io::Error>() {
                        match err.kind() {
                            std::io::ErrorKind::ConnectionAborted
                            | std::io::ErrorKind::ConnectionReset => {
                                return Err(RetryableExecuteError::RetryExponential {
                                    max_retries: Some(16),
                                    last_error: error,
                                });
                            }
                            _ => (),
                        }
                    }
                }
                return Err(RetryableExecuteError::Other(error));
            }
        }
    }

    #[maybe_async::maybe_async]
    pub(crate) async fn execute<F>(&self, builder: &F) -> Result<Response, ExecuteError>
    where
        F: Fn() -> RequestBuilder,
    {
        use super::req::sleep;

        let mut retries: u32 = 0;
        loop {
            let request = builder().build()?;
            match self.inner_execute(request).await {
                Ok(response) => return Ok(response),
                Err(RetryableExecuteError::RetryAfter(duration)) => {
                    retries += 1;
                    println!("[roux] Retrying request after {duration:?} ({retries})");
                    sleep(duration).await;
                }
                Err(RetryableExecuteError::RetryExponential {
                    max_retries,
                    last_error,
                }) => {
                    retries += 1;
                    if let Some(max_retries) = max_retries {
                        if retries > max_retries as u32 {
                            println!("[roux] Exceeded max retries for request, raising err.");
                            return Err(ExecuteError::ErrorOnly(last_error));
                        }
                    }
                    let secs = std::cmp::min(60, 2u64.pow(retries));
                    let duration = Duration::from_secs(secs);
                    println!(
                        "[roux] Exp retrying request after {duration:?} ({retries}/{max_retries:?})"
                    );
                    sleep(duration).await;
                }
                Err(RetryableExecuteError::OtherResponseError(response, e)) => {
                    return Err(ExecuteError::ResponseAndError(response, e));
                }
                Err(RetryableExecuteError::Other(e)) => {
                    return Err(ExecuteError::ErrorOnly(e));
                }
                Err(RetryableExecuteError::Unauthorized) => {
                    return Err(ExecuteError::AuthorizationRequired)
                }
            }
        }
    }

    #[maybe_async::maybe_async]
    pub(crate) async fn attempt_login(&self) -> Result<String, ExecuteError> {
        #[derive(Serialize)]
        struct LoginRequest<'a> {
            grant_type: &'a str,
            username: &'a str,
            password: &'a str,
        }

        #[derive(Deserialize, Debug)]
        #[serde(untagged)]
        enum AuthResponse {
            AuthData { access_token: String },
            ErrorData { error: String },
        }

        let login = LoginRequest {
            grant_type: "password",
            username: &self
                .config
                .username
                .as_ref()
                .ok_or(ExecuteError::AuthorizationRequired)?,
            password: &self
                .config
                .password
                .as_ref()
                .ok_or(ExecuteError::AuthorizationRequired)?,
        };

        let mut endpoint = EndpointBuilder::new("api/v1/access_token");
        endpoint.with_dot_json = false;

        let request = || {
            self.request(Method::POST, &endpoint)
                .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
                .form(&login)
        };

        let response = self.execute(&request).await?;
        let auth_data = response.json::<AuthResponse>().await?;

        let access_token = match auth_data {
            AuthResponse::AuthData { access_token } => access_token,
            AuthResponse::ErrorData { error } => return Err(ExecuteError::AuthError(error)),
        };

        Ok(access_token)
    }
}
