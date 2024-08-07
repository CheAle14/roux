use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::api::response::PostResponse;
use crate::api::ThingId;
use crate::models::comment::ArticleComments;
use crate::models::Listing;
use crate::util::url::build_subreddit;
use crate::util::RouxError;

use super::endpoint::EndpointBuilder;

use super::req::Response;
use super::subreddits::{Subreddit, Subreddits};
use super::user::User;

/// A generic client to send and build requests.
///
/// This allows the models to share common methods between Unauthed, OAuth or Authed,
/// as well as to specialize for Authed requests.
#[maybe_async::maybe_async(AFIT)]
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
    async fn post<T: Serialize>(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &T,
    ) -> Result<Response, RouxError>;

    /// Post the data, parsing the response as a [`PostResponse<T>`](crate::api::response::PostResponse).  
    /// If any errors are present, they are raised as [`RouxError::RedditError`](crate::util::error::RouxError).  
    /// Otherwise, the data is unwrapped and returned.
    async fn post_with_response<TReq: Serialize, TResp: DeserializeOwned>(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &TReq,
    ) -> Result<TResp, RouxError> {
        let response: PostResponse<TResp> = self.post_with_response_raw(endpoint, form).await?;

        if response.json.errors.len() > 0 {
            Err(RouxError::reddit_error(response.json.errors))
        } else {
            Ok(response.json.data.unwrap())
        }
    }

    /// Post the data, parsing the response as `TResp` directly.
    async fn post_with_response_raw<TReq: Serialize, TResp: DeserializeOwned>(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &TReq,
    ) -> Result<TResp, RouxError> {
        let response = self.post(endpoint, form).await?;
        let response: TResp = response.json().await?;
        Ok(response)
    }

    /// Creates a user helper, which can be used to make further requests using this underlying client
    fn user(&self, name: &str) -> User<Self>
    where
        Self: Sized + Clone,
    {
        User::new(name, self.clone())
    }

    /// Creates a subreddit helper
    fn subreddit(&self, name: &str) -> Subreddit<Self>
    where
        Self: Sized + Clone,
    {
        Subreddit::new(name, self.clone())
    }

    /// Creates a subreddits helper, which can be used to search for a subreddit.
    fn subreddits(&self) -> Subreddits<Self>
    where
        Self: Sized + Clone,
    {
        Subreddits(self.clone())
    }

    /// Get comments from article.
    #[maybe_async::maybe_async]
    async fn article_comments(
        &self,
        subreddit_name: &str,
        article: &ThingId,
        depth: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ArticleComments<Self>, RouxError>
    where
        Self: Sized + Clone,
    {
        let mut endpoint =
            build_subreddit(subreddit_name).join(format!("comments/{}", article.id()));

        if let Some(depth) = depth {
            endpoint.with_query("depth", depth.to_string());
        }

        if let Some(limit) = limit {
            endpoint.with_query("limit", limit.to_string());
        }

        let comments: crate::api::comment::ArticleCommentsResponse =
            self.get_json(endpoint).await?;

        let conv = Listing::new(comments.comments, self.clone());

        Ok(conv)
    }
}
