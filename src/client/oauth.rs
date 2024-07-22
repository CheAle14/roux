use std::error::Error;
use std::io;
use std::{sync::Arc, time::Duration};

use crate::client::traits::RedditClient;
use crate::{builders::form::FormBuilder, client::endpoint::EndpointBuilder};
use reqwest::{header, Method, StatusCode};
use serde::{Deserialize, Serialize};

use super::ratelimit::Ratelimit;
use super::{req::*, AuthedClient};
use crate::{config::Config, util::RouxError};

pub(crate) struct ClientInner {
    config: Config,
    base_url: &'static str,
    inner: Client,
    ratelimit: Mutex<Ratelimit>,
}

/// An OAuth client that is not yet authenticated with any particular user.
///
/// As with reqwest's own client, this uses an Arc internally so can be shared freely.
/// Indeed, using the [`crate::client::traits::RedditClient::subreddit`] and similar functions will Arc-clone this.
pub struct OAuthClient {
    inner: Arc<ClientInner>,
}

enum ExecuteError {
    RetryAfter(Duration),
    RetryExponential {
        max_retries: Option<u8>,
        last_error: reqwest::Error,
    },
    BadRequest(String),
    OtherResponseError(Response, reqwest::Error),
    Other(reqwest::Error),
}

impl From<reqwest::Error> for ExecuteError {
    fn from(value: reqwest::Error) -> Self {
        Self::Other(value)
    }
}

impl OAuthClient {
    /// Creates a new OAuthClient with the provided config.
    ///
    /// If no password is set, the base URL for all requests will be `www.reddit.com`, otherwise `oauth.reddit.com`.
    ///
    /// If provided, the user agent and access token will be used in their respective headers.
    pub fn new(config: Config) -> Result<Self, RouxError> {
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

        if let Some(access_token) = &config.access_token {
            headers.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
            );
        }

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(Self {
            inner: Arc::new(ClientInner {
                base_url,
                config,
                inner: client,
                ratelimit: Mutex::new(Ratelimit::new()),
            }),
        })
    }

    /// Attempts to login this client and produce an [`AuthedClient`].
    /// This will immediately error if the config does not have a username and password set.
    #[maybe_async::maybe_async]
    pub async fn login(self) -> Result<AuthedClient, RouxError> {
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
                .inner
                .config
                .username
                .as_ref()
                .ok_or(crate::util::RouxError::credentials_not_set())?,
            password: &self
                .inner
                .config
                .password
                .as_ref()
                .ok_or(crate::util::RouxError::credentials_not_set())?,
        };

        let mut endpoint = EndpointBuilder::new("api/v1/access_token");
        endpoint.with_dot_json = false;

        let request = || {
            self.request(Method::POST, &endpoint)
                .basic_auth(
                    &self.inner.config.client_id,
                    Some(&self.inner.config.client_secret),
                )
                .form(&login)
        };

        let response = self.execute(request).await?;

        if response.status() == 200 {
            let auth_data = response.json::<AuthResponse>().await?;

            let access_token = match auth_data {
                AuthResponse::AuthData { access_token } => access_token,
                AuthResponse::ErrorData { error } => {
                    return Err(crate::util::RouxError::auth(error))
                }
            };

            let mut config = self.inner.config.clone();
            config.access_token = Some(access_token);

            let client = Self::new(config)?;
            Ok(AuthedClient(client))
        } else {
            Err(crate::util::RouxError::status(response))
        }
    }

    pub(crate) fn request(&self, method: Method, endpoint: &EndpointBuilder) -> RequestBuilder {
        let url = endpoint.build(&self.inner.base_url);
        println!("[roux] {method:?} {url}");
        self.inner.inner.request(method, url)
    }

    #[cfg(feature = "blocking")]
    pub(crate) fn with_ratelimits(&self, request: Request) -> Result<Response, reqwest::Error> {
        let mut lock = self.inner.ratelimit.lock().unwrap();
        lock.delay();
        let response = self.inner.inner.execute(request)?;
        lock.update(response.headers());
        Ok(response)
    }
    #[cfg(not(feature = "blocking"))]
    pub(crate) async fn with_ratelimits(
        &self,
        request: Request,
    ) -> Result<Response, reqwest::Error> {
        let mut lock = self.inner.ratelimit.lock().await;
        lock.delay().await;
        let response = self.inner.inner.execute(request).await?;
        lock.update(response.headers());
        Ok(response)
    }

    #[maybe_async::maybe_async]
    async fn convert_error(
        &self,
        response: reqwest::Response,
        error: reqwest::Error,
    ) -> ExecuteError {
        let status = error.status().unwrap_or(StatusCode::BAD_REQUEST);
        println!("[roux] Response error: {status:?}");
        match status {
            StatusCode::TOO_MANY_REQUESTS => {
                if let Some(value) = response.headers().get("Retry-After") {
                    if let Ok(value) = value.to_str() {
                        if let Ok(value) = value.parse() {
                            return ExecuteError::RetryAfter(Duration::from_secs(value));
                        }
                    }
                }
                ExecuteError::RetryExponential {
                    max_retries: None,
                    last_error: error,
                }
            }
            StatusCode::BAD_REQUEST => match response.text().await {
                Ok(body) => ExecuteError::BadRequest(body),
                Err(e) => ExecuteError::Other(e),
            },
            StatusCode::INTERNAL_SERVER_ERROR => ExecuteError::RetryExponential {
                max_retries: Some(32),
                last_error: error,
            },
            _ => ExecuteError::OtherResponseError(response, error),
        }
    }

    #[maybe_async::maybe_async]
    async fn inner_execute(&self, request: Request) -> Result<Response, ExecuteError> {
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
                                return Err(ExecuteError::RetryExponential {
                                    max_retries: Some(16),
                                    last_error: error,
                                });
                            }
                            _ => (),
                        }
                    }
                }
                return Err(ExecuteError::Other(error));
            }
        }
    }

    #[maybe_async::maybe_async]
    pub(crate) async fn execute<F>(&self, builder: F) -> Result<Response, crate::util::RouxError>
    where
        F: Fn() -> RequestBuilder,
    {
        use super::req::sleep;

        let mut retries: u32 = 0;
        loop {
            let request = builder().build()?;
            match self.inner_execute(request).await {
                Ok(response) => return Ok(response),
                Err(ExecuteError::RetryAfter(duration)) => {
                    retries += 1;
                    println!("[roux] Retrying request after {duration:?} ({retries})");
                    sleep(duration).await;
                }
                Err(ExecuteError::RetryExponential {
                    max_retries,
                    last_error,
                }) => {
                    retries += 1;
                    if let Some(max_retries) = max_retries {
                        if retries > max_retries as u32 {
                            println!("[roux] Exceeded max retries for request, raising err.");
                            return Err(RouxError::network(last_error));
                        }
                    }
                    let secs = std::cmp::min(60, 2u64.pow(retries));
                    let duration = Duration::from_secs(secs);
                    println!(
                        "[roux] Exp retrying request after {duration:?} ({retries}/{max_retries:?})"
                    );
                    sleep(duration).await;
                }
                Err(ExecuteError::BadRequest(body)) => {
                    return Err(RouxError::reddit_error(body));
                }
                Err(ExecuteError::OtherResponseError(response, e)) => {
                    return Err(RouxError::full_network(response, e));
                }
                Err(ExecuteError::Other(e)) => {
                    return Err(RouxError::network(e));
                }
            }
        }
    }

    pub(crate) fn config(&self) -> &Config {
        &self.inner.config
    }
}

impl RedditClient for OAuthClient {
    #[maybe_async::maybe_async]
    async fn get(&self, endpoint: impl Into<EndpointBuilder>) -> Result<Response, RouxError> {
        let endpoint = endpoint.into();

        let builder = || self.request(Method::GET, &endpoint);

        self.execute(builder).await
    }

    #[maybe_async::maybe_async]
    async fn post(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &FormBuilder<'_>,
    ) -> Result<Response, RouxError> {
        let endpoint = endpoint.into();
        let r = || self.request(Method::POST, &endpoint).form(form);

        self.execute(r).await
    }
}

impl Clone for OAuthClient {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
