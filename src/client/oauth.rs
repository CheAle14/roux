use std::sync::Arc;

use crate::client::traits::RedditClient;
use crate::{builders::form::FormBuilder, client::endpoint::EndpointBuilder};
use reqwest::Method;
use serde::Serialize;

use super::inner::ClientInner;
use super::{req::*, AuthedClient};
use crate::{config::Config, util::RouxError};

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
        let inner = ClientInner::new(config)?;
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    #[maybe_async::maybe_async]
    pub(crate) async fn execute<F>(&self, builder: F) -> Result<Response, RouxError>
    where
        F: Fn() -> RequestBuilder,
    {
        let response = self.inner.execute(&builder).await?;
        Ok(response)
    }

    /// Attempts to login this client and produce an [`AuthedClient`].
    /// This will immediately error if the config does not have a username and password set.
    #[maybe_async::maybe_async]
    pub async fn login(self) -> Result<AuthedClient, RouxError> {
        let token = self.inner.attempt_login().await?;
        AuthedClient::new(self.inner.config.clone(), token)
    }

    pub(crate) fn request(&self, method: Method, endpoint: &EndpointBuilder) -> RequestBuilder {
        self.inner.request(method, endpoint)
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
    async fn post<T: Serialize>(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &T,
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
