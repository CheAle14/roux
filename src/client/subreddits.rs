//! # Subreddit
//! A read-only module to read data from a specific subreddit.
//!
//! # Basic Usage
//! ```no_run
//! use roux::Subreddit;
//! # #[cfg(not(feature = "blocking"))]
//! # use tokio;
//!
//! # #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! # #[maybe_async::maybe_async]
//! # async fn main() {
//! let subreddit = Subreddit::new("rust");
//! // Now you are able to:
//!
//! // Get moderators.
//! let moderators = subreddit.moderators().await;
//!
//! // Get hot posts with limit = 25.
//! let hot = subreddit.hot(25, None).await;
//!
//! // Get rising posts with limit = 30.
//! let rising = subreddit.rising(30, None).await;
//!
//! // Get top posts with limit = 10.
//! let top = subreddit.top(10, None).await;
//!
//! // Get latest comments.
//! // `depth` and `limit` are optional.
//! let latest_comments = subreddit.latest_comments(None, Some(25)).await;
//!
//! // Get comments from a submission.
//! let article_id = &hot.unwrap().data.children.first().unwrap().data.id.clone();
//! let article_comments = subreddit.article_comments(article_id, None, Some(25));
//! # }
//! ```
//!
//! # Usage with feed options
//!
//! ```no_run
//! use roux::Subreddit;
//! use roux::util::{FeedOption, TimePeriod};
//! # #[cfg(not(feature = "blocking"))]
//! # use tokio;
//!
//! # #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! # #[maybe_async::maybe_async]
//! # async fn main() {
//! let subreddit = Subreddit::new("astolfo");
//!
//! // Gets top 10 posts from this month
//! let options = FeedOption::new().period(TimePeriod::ThisMonth);
//! let top = subreddit.top(25, Some(options)).await;
//!
//! // Gets hot 10
//! let hot = subreddit.hot(25, None).await;
//!
//! // Get after param from `hot`
//! let after = hot.unwrap().data.after.unwrap();
//! let after_options = FeedOption::new().after(&after);
//!
//! // Gets next 25
//! let next_hot = subreddit.hot(25, Some(after_options)).await;
//! # }
//! ```
use crate::api::comment::latest::LatestCommentData;
use crate::api::subreddit::{SubredditData, SubredditResponse, SubredditsData};

use crate::builders::submission::SubmissionSubmitBuilder;
use crate::models::comment::{ArticleComments, LatestComments};
use crate::models::submission::Submissions;
use crate::models::{ArticleComment, LatestComment, Listing, Submission};
use crate::util::url::build_subreddit;
use crate::util::{FeedOption, RouxError};

use crate::api::response::BasicListing as APIListing;
use crate::api::{Moderators, ThingId};

use super::endpoint::EndpointBuilder;
use super::traits::RedditClient;
use super::AuthedClient;

/// Access subreddits API
pub struct Subreddits<T>(pub(crate) T);

impl<T: RedditClient> Subreddits<T> {
    /// Search subreddits
    #[maybe_async::maybe_async]
    pub async fn search(
        &self,
        name: &str,
        options: Option<FeedOption>,
    ) -> Result<SubredditsData, RouxError> {
        let mut url = EndpointBuilder::new("subreddits/search").query("q", name);

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        self.0.get_json(url).await
    }
}

/// Subreddit
pub struct Subreddit<T> {
    /// Name of subreddit.
    pub name: String,
    client: T,
}

impl<T: RedditClient + Clone> Subreddit<T> {
    /// Create a new `Subreddit` instance.
    pub fn new(name: impl Into<String>, client: T) -> Subreddit<T> {
        Subreddit {
            name: name.into(),
            client,
        }
    }

    pub(crate) fn endpoint(&self, endpoint: impl Into<EndpointBuilder>) -> EndpointBuilder {
        build_subreddit(&self.name).join(endpoint)
    }

    /// Get subreddit data.
    #[maybe_async::maybe_async]
    pub async fn about(&self) -> Result<SubredditData, RouxError> {
        let endpoint = self.endpoint("about/moderators");
        let resp: SubredditResponse = self.client.get_json(endpoint).await?;
        Ok(resp.data)
    }

    #[maybe_async::maybe_async]
    async fn get_feed(
        &self,
        ty: &str,
        options: Option<FeedOption>,
    ) -> Result<Submissions<T>, RouxError> {
        let mut endpoint = self.endpoint(format!("{ty}"));

        if let Some(options) = options {
            options.build_url(&mut endpoint);
        }

        let api: crate::api::APISubmissions = self.client.get_json(endpoint).await?;
        let listing = Listing::new(api, self.client.clone());

        Ok(listing)
    }

    /// Get hot posts.
    #[maybe_async::maybe_async]
    pub async fn hot(&self, options: Option<FeedOption>) -> Result<Submissions<T>, RouxError> {
        self.get_feed("hot", options).await
    }

    /// Get rising posts.
    #[maybe_async::maybe_async]
    pub async fn rising(&self, options: Option<FeedOption>) -> Result<Submissions<T>, RouxError> {
        self.get_feed("rising", options).await
    }

    /// Get top posts.
    #[maybe_async::maybe_async]
    pub async fn top(&self, options: Option<FeedOption>) -> Result<Submissions<T>, RouxError> {
        self.get_feed("top", options).await
    }

    /// Get latest posts.
    #[maybe_async::maybe_async]
    pub async fn latest(&self, options: Option<FeedOption>) -> Result<Submissions<T>, RouxError> {
        self.get_feed("new", options).await
    }

    /// Get latest comments.
    #[maybe_async::maybe_async]
    pub async fn latest_comments(
        &self,
        depth: Option<u32>,
        limit: Option<u32>,
    ) -> Result<LatestComments<T>, RouxError> {
        let mut endpoint = self.endpoint("comments");

        if let Some(depth) = depth {
            endpoint.with_query("depth", depth.to_string());
        }

        if let Some(limit) = limit {
            endpoint.with_query("limit", limit.to_string());
        }

        let api: APIListing<LatestCommentData> = self.client.get_json(endpoint).await?;

        let conv = Listing::new(api, self.client.clone());
        Ok(conv)
    }

    /// Get comments from article.
    #[maybe_async::maybe_async]
    pub async fn article_comments(
        &self,
        article: &ThingId,
        depth: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ArticleComments<T>, RouxError> {
        self.client
            .article_comments(&self.name, article, depth, limit)
            .await
    }
}

impl Subreddit<AuthedClient> {
    /// Get moderators (requires authentication)
    #[maybe_async::maybe_async]
    pub async fn moderators(&self) -> Result<Moderators, RouxError> {
        let endpoint = self.endpoint("about/moderators");
        self.client.get_json(endpoint).await
    }

    /// Submits a post to this subreddit
    #[maybe_async::maybe_async]
    pub async fn submit(
        &self,
        submission: &SubmissionSubmitBuilder,
    ) -> Result<Submission<AuthedClient>, RouxError> {
        self.client.submit(&self.name, submission).await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::noauth::UnauthedClient;
    use crate::client::traits::RedditClient;
    use crate::util::FeedOption;

    #[maybe_async::async_impl]
    #[tokio::test]
    async fn test_no_auth() {
        let client = UnauthedClient::new().unwrap();
        let subreddit = client.subreddit("astolfo");

        // Test feeds
        let hot = subreddit.hot(Some(FeedOption::new().limit(25))).await;
        assert!(hot.is_ok());

        let rising = subreddit.rising(Some(FeedOption::new().limit(25))).await;
        assert!(rising.is_ok());

        let top = subreddit.top(Some(FeedOption::new().limit(25))).await;
        assert!(top.is_ok());

        let latest_comments = subreddit.latest_comments(None, Some(25)).await;
        assert!(latest_comments.is_ok());

        let article_id = &hot.unwrap().children.first().unwrap().name().clone();
        let article_comments = subreddit.article_comments(article_id, None, Some(25)).await;
        assert!(article_comments.is_ok());

        // Test subreddit data.
        let data_res = subreddit.about().await;
        assert!(data_res.is_ok());

        let data = data_res.unwrap();
        assert!(data.title == Some(String::from("Rider of Black, Astolfo")));
        assert!(data.subscribers.is_some());
        assert!(data.subscribers.unwrap() > 1000);

        // Test subreddit search
        let subreddits_limit = 3u32;
        let subreddits = client
            .subreddits()
            .search("rust", Some(FeedOption::new().limit(subreddits_limit)))
            .await;
        assert!(subreddits.is_ok());
        assert!(subreddits.unwrap().data.children.len() == subreddits_limit as usize);
    }
}
