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
use reqwest::StatusCode;

use crate::api::comment::latest::LatestCommentData;
use crate::api::subreddit::{
    FlairSelection, ModActionData, ModActionType, ModLogListing, SubredditData,
    SubredditRemovalReasons, SubredditResponse, SubredditsData,
};

use crate::builders::form::FormBuilder;
use crate::builders::submission::SubmissionSubmitBuilder;
use crate::models::comment::{ArticleComments, LatestComments};
use crate::models::submission::Submissions;
use crate::models::{FromClientAndData, Listing, Submission, SubmissionStickySlot};
use crate::util::error::RouxErrorKind;
use crate::util::ser_enumstr::get_enum_name;
use crate::util::url::build_subreddit;
use crate::util::{FeedOption, RouxError};

use crate::api::response::BasicListing as APIListing;
use crate::api::{Moderators, ThingFullname};

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
    /// The reddit client used.
    pub client: T,
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
        let endpoint = self.endpoint("about");
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
        article: &ThingFullname,
        depth: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ArticleComments<T>, RouxError> {
        self.client
            .article_comments(&self.name, article, depth, limit)
            .await
    }

    /// Fetches the stickied post on the subreddit, if there is one.
    #[maybe_async::maybe_async]
    pub async fn sticky(
        &self,
        slot: SubmissionStickySlot,
    ) -> Result<Option<Submission<T>>, RouxError> {
        let mut url = self.endpoint("about/sticky");

        match slot {
            SubmissionStickySlot::Top => (),
            SubmissionStickySlot::Bottom => {
                url.with_query("num", "2");
            }
        }

        let response = match self.client.get(url).await {
            Ok(response) => response,
            Err(error) => match error.kind {
                RouxErrorKind::FullNetwork(response, _)
                    if response.status() == StatusCode::NOT_FOUND =>
                {
                    return Ok(None)
                }
                _ => return Err(error),
            },
        };

        let data: crate::api::comment::ArticleCommentsResponseWithoutComments =
            response.json().await?;

        let post = Submission::new(self.client.clone(), data.submission);

        Ok(Some(post))
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

    /// List possible flair options in this subreddit
    #[maybe_async::maybe_async]
    pub async fn list_flairs(&self, selecting: FlairSelector) -> Result<FlairSelection, RouxError> {
        let mut form = FormBuilder::new();
        match selecting {
            FlairSelector::Link(link) => form.add("link", link.into_inner()),
            FlairSelector::NewLink => form.add("is_newlink", "true"),
            FlairSelector::User(name) => form.add("name", name),
        };

        let url = self.endpoint("api/flairselector");

        let got = self.client.post_with_response_raw(url, &form).await?;
        Ok(got)
    }

    /// Accepts an invite to become a moderator for this subreddit. Must have been invited by a current moderator.
    #[maybe_async::maybe_async]
    pub async fn accept_moderator_invite(&self) -> Result<(), RouxError> {
        let form = FormBuilder::new();

        let url = self.endpoint("api/accept_moderator_invite");
        self.client.post(url, &form).await?;
        Ok(())
    }

    /// Returns a list of removal reasons for this subreddit.
    #[maybe_async::maybe_async]
    pub async fn list_removal_reasons(&self) -> Result<SubredditRemovalReasons, RouxError> {
        let url = EndpointBuilder::new(format!("api/v1/{name}/removal_reasons", name = self.name));
        self.client.get_json(url).await
    }

    /// Returns a list of mod actions taken
    #[maybe_async::maybe_async]
    pub async fn list_mod_log(
        &self,
        after: Option<String>,
        limit: Option<u16>,
        moderators: Option<String>,
        action: Option<ModActionType>,
    ) -> Result<Vec<ModActionData>, RouxError> {
        let mut endpoint = self.endpoint("about/log");

        if let Some(after) = after {
            endpoint.with_query("after", after);
        }

        if let Some(limit) = limit {
            endpoint.with_query("limit", limit.min(500).to_string());
        }

        if let Some(mods) = moderators {
            endpoint.with_query("mod", mods);
        }

        if let Some(action) = action {
            let name = get_enum_name(&action);
            endpoint.with_query("type", name);
        }

        let result: ModLogListing = self.client.get_json(endpoint).await?;

        Ok(result.data.children.into_iter().map(|d| d.data).collect())
    }
}

/// For use in [`Subreddit::list_flairs`](crate::client::subreddits::Subreddit::list_flairs)
pub enum FlairSelector {
    /// List potential flairs for an existing link
    Link(ThingFullname),
    /// List potential flairs for a new submission
    NewLink,
    /// List user flairs for a particular user.
    User(String),
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
