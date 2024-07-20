use serde::de::DeserializeOwned;

use crate::builders::form::FormBuilder;
use crate::util::RouxError;

use super::endpoint::EndpointBuilder;

use super::req::Response;
use super::subreddits::{Subreddit, Subreddits};
use super::user::User;

/// A generic async client to send and build requests.
///
/// This allows the stateful models to be agnostic as to whether they are Unauthed, OAuth or Authed.
#[cfg(not(feature = "blocking"))]
pub trait RedditClient {
    /// Get the endpoint, returning the raw response or an error.
    async fn get(&self, endpoint: impl Into<EndpointBuilder>) -> Result<Response, RouxError>;

    /// Get the endpoint, parsing the response into the type.
    async fn get_json<T: DeserializeOwned>(
        &self,
        endpoint: impl Into<EndpointBuilder>,
    ) -> Result<T, RouxError> {
        Ok(self.get(endpoint).await?.json().await?)
    }

    /// Post the data to the endpoint.
    async fn post(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &FormBuilder<'_>,
    ) -> Result<Response, RouxError>;

    /// Post the data, parsing the response as JSON.
    async fn post_with_response<TResp: DeserializeOwned>(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &FormBuilder<'_>,
    ) -> Result<TResp, RouxError> {
        let response = self.post(endpoint, form).await?;
        Ok(response.json().await?)
    }

    /// Creates a stateful user, which can be used to make further requests using this underlying client
    fn user(&self, name: &str) -> User<Self>
    where
        Self: Sized + Clone,
    {
        User::new(name, self.clone())
    }

    /// Creates a stateful subreddit
    fn subreddit(&self, name: &str) -> Subreddit<Self>
    where
        Self: Sized + Clone,
    {
        Subreddit::new(name, self.clone())
    }

    /// Creates a stateful subreddits, which can be used to search for a subreddit.
    fn subreddits(&self) -> Subreddits<Self>
    where
        Self: Sized + Clone,
    {
        Subreddits(self.clone())
    }
}

/// A generic blocking client to send and build requests.
///
/// This allows the stateful models to be agnostic as to whether they are Unauthed, OAuth or Authed.
#[cfg(feature = "blocking")]
pub trait RedditClient {
    /// Get the endpoint, returning the raw response or an error.
    fn get(&self, endpoint: impl Into<EndpointBuilder>) -> Result<Response, RouxError>;

    /// Get the endpoint, parsing the response into the type.
    fn get_json<T: DeserializeOwned>(
        &self,
        endpoint: impl Into<EndpointBuilder>,
    ) -> Result<T, RouxError> {
        Ok(self.get(endpoint)?.json()?)
    }

    /// Post the data to the endpoint.
    fn post(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &FormBuilder<'_>,
    ) -> Result<Response, RouxError>;

    /// Post the data, parsing the response as JSON.
    fn post_with_response<TResp: DeserializeOwned>(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &FormBuilder<'_>,
    ) -> Result<TResp, RouxError> {
        let response = self.post(endpoint, form)?;
        Ok(response.json()?)
    }

    /// Creates a stateful user, which can be used to make further requests using this underlying client
    fn user(&self, name: &str) -> User<Self>
    where
        Self: Sized + Clone,
    {
        User::new(name, self.clone())
    }

    /// Creates a stateful subreddit
    fn subreddit(&self, name: &str) -> Subreddit<Self>
    where
        Self: Sized + Clone,
    {
        Subreddit::new(name, self.clone())
    }

    /// Creates a stateful subreddits, which can be used to search for a subreddit.
    fn subreddits(&self) -> Subreddits<Self>
    where
        Self: Sized + Clone,
    {
        Subreddits(self.clone())
    }
}
