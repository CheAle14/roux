use std::{sync::Arc, time::Duration};

use crate::client::traits::RedditClient;
use crate::{builders::form::FormBuilder, client::endpoint::EndpointBuilder};
use reqwest::{header, Method, StatusCode};
use serde::{Deserialize, Serialize};

use super::{req::*, AuthedClient};
use crate::{config::Config, util::RouxError};

pub(crate) struct ClientInner {
    config: Config,
    base_url: &'static str,
    inner: Client,
}

/// An OAuth client that is not yet authenticated with any particular user.
///
/// As with reqwest's own client, this uses an Arc internally so can be shared freely.
/// Indeed, using the [`crate::client::traits::RedditClient::subreddit`] and similar functions will Arc-clone this.
pub struct OAuthClient {
    inner: Arc<ClientInner>,
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
                .to_owned()
                .ok_or(crate::util::RouxError::CredentialsNotSet)?,
            password: &self
                .inner
                .config
                .password
                .to_owned()
                .ok_or(crate::util::RouxError::CredentialsNotSet)?,
        };

        let mut endpoint = EndpointBuilder::new("api/v1/access_token");
        endpoint.with_dot_json = false;

        let request = self
            .request(Method::POST, endpoint)
            .basic_auth(
                &self.inner.config.client_id,
                Some(&self.inner.config.client_secret),
            )
            .form(&login)
            .build()?;

        let response = self.execute(request).await?;

        if response.status() == 200 {
            let auth_data = response.json::<AuthResponse>().await?;

            let access_token = match auth_data {
                AuthResponse::AuthData { access_token } => access_token,
                AuthResponse::ErrorData { error } => {
                    return Err(crate::util::RouxError::Auth(error))
                }
            };

            let mut config = self.inner.config.clone();
            config.access_token = Some(access_token);

            let client = Self::new(config)?;
            Ok(AuthedClient(client))
        } else {
            Err(crate::util::RouxError::Status(response))
        }
    }

    pub(crate) fn request(
        &self,
        method: Method,
        endpoint: impl Into<EndpointBuilder>,
    ) -> RequestBuilder {
        let endpoint = endpoint.into();
        let url = endpoint.build(&self.inner.base_url);
        println!("{method:?} {url}");
        self.inner.inner.request(method, url)
    }

    #[maybe_async::maybe_async]
    pub(crate) async fn execute(
        &self,
        request: Request,
    ) -> Result<Response, crate::util::RouxError> {
        let response = self.inner.inner.execute(request).await?;
        if let Err(e) = response.error_for_status_ref() {
            let status = e.status().unwrap_or(StatusCode::BAD_REQUEST);
            match status {
                StatusCode::TOO_MANY_REQUESTS => {
                    if let Some(value) = response.headers().get("Retry-After") {
                        if let Ok(value) = value.to_str() {
                            if let Ok(value) = value.parse() {
                                return Err(RouxError::Ratelimited {
                                    retry_after: Some(Duration::from_secs(value)),
                                });
                            }
                        }
                    }
                    Err(RouxError::Ratelimited { retry_after: None })
                }
                StatusCode::BAD_REQUEST => {
                    let body = response.text().await?;
                    Err(RouxError::RedditError { body })
                }
                _ => Err(crate::util::RouxError::FullNetwork(response, e)),
            }
        } else {
            Ok(response)
        }
    }

    pub(crate) fn config(&self) -> &Config {
        &self.inner.config
    }
}

impl RedditClient for OAuthClient {
    #[maybe_async::maybe_async]
    async fn get(&self, endpoint: impl Into<EndpointBuilder>) -> Result<Response, RouxError> {
        let r = self.request(Method::GET, endpoint).build()?;
        self.execute(r).await
    }

    #[maybe_async::maybe_async]
    async fn post(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &FormBuilder<'_>,
    ) -> Result<Response, RouxError> {
        let r = self.request(Method::POST, endpoint).form(form).build()?;

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
