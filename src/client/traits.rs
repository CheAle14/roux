use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::api::response::PostResponse;
use crate::api::{APISubmissions, ThingFullname};
use crate::models::comment::ArticleComments;
use crate::models::submission::Submissions;
use crate::models::{Listing, Submission};
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
        let response = self.get(endpoint).await?;

        Ok(parse_response_as_json(response).await?)
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
        let response = parse_response_as_json(response).await?;
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
        article: &ThingFullname,
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

    /// Get submissions by id
    #[maybe_async::maybe_async]
    async fn get_submissions(&self, ids: &[&ThingFullname]) -> Result<Submissions<Self>, RouxError>
    where
        Self: Sized + Clone,
    {
        let mut ids = ids.iter().map(|id| id.full());
        let mut url = format!("by_id/");
        url.push_str(ids.next().unwrap());
        for next in ids {
            url.push(',');
            url.push_str(next);
        }

        let url = EndpointBuilder::new(url);

        let json: APISubmissions = self.get_json(url).await?;
        let conv = Listing::new(json, self.clone());
        Ok(conv)
    }

    /// Gets a submission by its permalink
    #[maybe_async::maybe_async]
    async fn get_submission_by_link(&self, url: &str) -> Result<Submission<Self>, RouxError>
    where
        Self: Sized + Clone,
    {
        let thing_id = ThingFullname::from_submission_link(url)
            .ok_or_else(|| RouxError::credentials_not_set())?;

        let post = self.get_submissions(&[&thing_id]).await?;
        let post = post.into_iter().next().unwrap();
        Ok(post)
    }
}

#[cfg(feature = "log-json-on-error")]
#[maybe_async::maybe_async]
async fn parse_response_as_json<T: DeserializeOwned>(response: Response) -> Result<T, RouxError> {
    use std::sync::atomic::AtomicU64;
    static ERRORS: AtomicU64 = AtomicU64::new(0);

    let text = response.text().await?;

    match serde_json::from_str(&text) {
        Ok(v) => Ok(v),
        Err(e) => {
            let file = format!(
                "roux-json-error-{}.json",
                ERRORS.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            );
            std::fs::write(file, &text).unwrap();
            Err(RouxError::from(e))
        }
    }
}

#[cfg(not(feature = "log-json-on-error"))]
#[maybe_async::maybe_async]
async fn parse_response_as_json<T: DeserializeOwned>(response: Response) -> Result<T, RouxError> {
    Ok(response.json().await?)
}
