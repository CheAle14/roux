//! # User
//! A read-only module to read data from for a specific user.
//!
//! # Usage
//! ```no_run
//! use roux::User;
//! use roux::util::FeedOption;
//! # #[cfg(not(feature = "blocking"))]
//! # use tokio;
//!
//! # #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! # #[maybe_async::maybe_async]
//! # async fn main() {
//! let user = User::new("kasuporo");
//! // Now you are able to:
//!
//! // Get overview
//! let overview = user.overview(None).await;
//!
//! // Get submitted posts.
//! let submitted = user.submitted(None).await;
//!
//! // Get comments.
//! let comments = user.comments(None).await;
//! # }
//! ```

extern crate serde_json;

use crate::util::{FeedOption, RouxError};

use crate::api::{About, Comments, Overview, Submissions};

use super::endpoint::EndpointBuilder;
use super::traits::RedditClient;

/// User.
pub struct User<T> {
    /// User's name.
    pub user: String,
    client: T,
}

impl<T: RedditClient> User<T> {
    /// Create a new `User` instance.
    pub fn new(user: &str, client: T) -> User<T> {
        User {
            user: user.to_owned(),
            client,
        }
    }

    /// Get user's overview.
    #[maybe_async::maybe_async]
    pub async fn overview(&self, options: Option<FeedOption>) -> Result<Overview, RouxError> {
        let mut endpoint = EndpointBuilder::from(format!("user/{}/overview", self.user));

        if let Some(options) = options {
            options.build_url(&mut endpoint);
        }

        self.client.get_json(endpoint).await
    }

    /// Get user's submitted posts.
    #[maybe_async::maybe_async]
    pub async fn submitted(&self, options: Option<FeedOption>) -> Result<Submissions, RouxError> {
        let mut url = EndpointBuilder::from(format!("user/{}/submitted", self.user));

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        self.client.get_json(url).await
    }

    /// Get user's submitted comments.
    #[maybe_async::maybe_async]
    pub async fn comments(&self, options: Option<FeedOption>) -> Result<Comments, RouxError> {
        let mut url = EndpointBuilder::from(format!("user/{}/comments", self.user));

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        self.client.get_json(url).await
    }

    /// Get user's about page
    #[maybe_async::maybe_async]
    pub async fn about(&self, options: Option<FeedOption>) -> Result<About, RouxError> {
        let mut url = EndpointBuilder::from(format!("{}/about", self.user));

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        self.client.get_json(url).await
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use crate::{
        client::{noauth::UnauthedClient, traits::RedditClient},
        util::FeedOption,
    };

    #[maybe_async::async_impl]
    #[tokio::test]
    async fn test_no_auth() {
        let client = UnauthedClient::new().unwrap();
        let user = client.user("beneater");

        // Test overview
        let overview = user.overview(None).await.unwrap();

        // Test submitted
        let submitted = user.submitted(None).await.unwrap();

        // Test comments
        let comments = user.comments(None).await.unwrap();

        // Test about
        let about = user.about(None).await.unwrap();

        // Test feed options
        let after = comments.data.after.unwrap();
        let after_options = FeedOption::new().after(&after);
        let next_comments = user.comments(Some(after_options)).await.unwrap();
    }
}
