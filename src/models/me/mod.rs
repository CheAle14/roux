//! # Me
//! Me module.

pub mod response;

extern crate reqwest;
extern crate serde_json;

use serde::Serialize;

use crate::client::{Client, Response};
use crate::config::Config;
use crate::models::me::response::MeData;
use crate::models::{Friend, Inbox, Saved};
use crate::submission::SubmissionData;
use crate::util::{url, FeedOption, RouxError};
use crate::{builders::submission::SubmissionSubmitBuilder, Submissions, ThingId};

/// Me
#[derive(Debug, Clone)]
pub struct Me {
    /// Config
    pub config: Config,
    /// Client
    pub client: Client,
}

impl Me {
    /// Create a new `me`
    pub fn new(config: &Config, client: &Client) -> Me {
        Me {
            config: config.to_owned(),
            client: client.to_owned(),
        }
    }

    #[maybe_async::maybe_async]
    async fn get(&self, url: &str) -> Result<Response, RouxError> {
        let get_url = url::build_oauth(url);

        match self.client.get(&get_url[..]).send().await {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        }
    }

    #[maybe_async::maybe_async]
    async fn post<T: Serialize>(&self, url: &str, form: T) -> Result<Response, RouxError> {
        let post_url = url::build_oauth(url).to_owned();

        match self.client.post(&post_url[..]).form(&form).send().await {
            Ok(response) => Ok(response),
            Err(e) => Err(e.into()),
        }
    }

    /// Get me
    #[maybe_async::maybe_async]
    pub async fn me(&self) -> Result<MeData, RouxError> {
        match self.get("api/v1/me").await {
            Ok(res) => Ok(res.json::<MeData>().await?),
            Err(e) => Err(e),
        }
    }

    /// Submits a new post to the subreddit from the builder
    ///
    /// Note that `subreddit_name` is the display name of the subreddit without the `/r/` prefix, NOT the "full name" (e.g. `t5_abcde`)
    #[maybe_async::maybe_async]
    pub async fn submit(
        &self,
        subreddit_name: &str,
        submission: &SubmissionSubmitBuilder,
    ) -> Result<SubmissionData, RouxError> {
        #[derive(Serialize)]
        struct SubmitRequest<'a> {
            sr: &'a str,
            #[serde(flatten)]
            data: &'a SubmissionSubmitBuilder,
        }

        let req = SubmitRequest {
            sr: subreddit_name,
            data: submission,
        };

        let parsed: crate::response::PostResponse = self
            .post("api/submit", &req)
            .await?
            .error_for_status()?
            .json()
            .await?;

        let mut submissions = self
            .get_submissions(&[&parsed.json.data.unwrap().name])
            .await?;
        let rtn = submissions.data.children.pop().unwrap();
        Ok(rtn.data)
    }

    /// Adds a friend to a subreddit with the specified type
    #[maybe_async::maybe_async]
    pub async fn add_subreddit_friend(
        &self,
        username: &str,
        typ: &str,
        sub: &str,
    ) -> Result<bool, RouxError> {
        let form = [("name", username), ("type", typ)];
        Ok(self
            .post(format!("r/{}/api/friend", sub).as_str(), form)
            .await?
            .json::<Friend>()
            .await?
            .success)
    }

    /// Removes a friend to a subreddit with the specified type
    #[maybe_async::maybe_async]
    pub async fn remove_subreddit_friend(
        &self,
        username: &str,
        typ: &str,
        sub: &str,
    ) -> Result<bool, RouxError> {
        let form = [("name", username), ("type", typ)];
        Ok(self
            .post(format!("r/{}/api/unfriend", sub).as_str(), form)
            .await?
            .json::<Friend>()
            .await?
            .success)
    }

    /// Compose message
    #[maybe_async::maybe_async]
    pub async fn compose_message(
        &self,
        username: &str,
        subject: &str,
        body: &str,
    ) -> Result<Response, RouxError> {
        let form = [
            ("api_type", "json"),
            ("subject", subject),
            ("text", body),
            ("to", username),
        ];

        self.post("api/compose", &form).await
    }

    /// Get user's submitted posts.
    #[maybe_async::maybe_async]
    pub async fn inbox(&self) -> Result<Inbox, RouxError> {
        Ok(self.get("message/inbox").await?.json::<Inbox>().await?)
    }

    /// Get saved
    #[maybe_async::maybe_async]
    pub async fn saved(&self, options: Option<FeedOption>) -> Result<Saved, RouxError> {
        let mut url = format!(
            "user/{}/saved/.json",
            self.config.username.to_owned().unwrap()
        );

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        Ok(self.get(&url).await?.json::<Saved>().await?)
    }

    /// Get upvoted
    #[maybe_async::maybe_async]
    pub async fn upvoted(&self, options: Option<FeedOption>) -> Result<Saved, RouxError> {
        let mut url = format!(
            "user/{}/upvoted/.json",
            self.config.username.to_owned().unwrap()
        );

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        Ok(self.get(&url).await?.json::<Saved>().await?)
    }

    /// Get downvoted
    #[maybe_async::maybe_async]
    pub async fn downvoted(&self, options: Option<FeedOption>) -> Result<Saved, RouxError> {
        let mut url = format!(
            "user/{}/downvoted/.json",
            self.config.username.to_owned().unwrap()
        );

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        Ok(self.get(&url).await?.json::<Saved>().await?)
    }

    /// Get users unread messages
    #[maybe_async::maybe_async]
    pub async fn unread(&self) -> Result<Inbox, RouxError> {
        Ok(self.get("message/unread").await?.json::<Inbox>().await?)
    }

    /// Mark messages as read
    #[maybe_async::maybe_async]
    pub async fn mark_read(&self, ids: &ThingId) -> Result<Response, RouxError> {
        let form = [("id", ids.full())];
        self.post("api/read_message", &form).await
    }

    /// Mark messages as unread
    #[maybe_async::maybe_async]
    pub async fn mark_unread(&self, ids: &str) -> Result<Response, RouxError> {
        let form = [("id", ids)];
        self.post("api/unread_message", &form).await
    }

    /// Comment
    #[maybe_async::maybe_async]
    pub async fn comment(&self, text: &str, parent: &ThingId) -> Result<Response, RouxError> {
        let form = [("text", text), ("parent", parent.full())];
        self.post("api/comment", &form).await
    }

    /// Edit a 'thing'
    #[maybe_async::maybe_async]
    pub async fn edit(&self, text: &str, parent: &ThingId) -> Result<Response, RouxError> {
        let form = [("text", text), ("thing_id", parent.full())];
        self.post("api/editusertext", &form).await
    }

    /// Get submissions by id
    #[maybe_async::maybe_async]
    pub async fn get_submissions(&self, ids: &[&ThingId]) -> Result<Submissions, RouxError> {
        let mut ids = ids.iter().map(|id| id.full());
        let mut url = format!("/by_id/");
        url.push_str(ids.next().unwrap());
        for next in ids {
            url.push(',');
            url.push_str(next);
        }

        Ok(self.get(&url).await?.json::<Submissions>().await?)
    }

    /// Report
    #[maybe_async::maybe_async]
    pub async fn report(&self, id: &ThingId, reason: &str) -> Result<Response, RouxError> {
        let form = [("id", id.full()), ("reason", reason)];
        self.post("api/report", &form).await
    }

    /// Logout
    #[maybe_async::maybe_async]
    pub async fn logout(self) -> Result<(), RouxError> {
        let url = "https://www.reddit.com/api/v1/revoke_token";

        let form = [("access_token", self.config.access_token.to_owned())];

        let response = self
            .client
            .post(url)
            .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
            .form(&form)
            .send()
            .await?;

        if response.status() == 204 {
            Ok(())
        } else {
            Err(RouxError::Status(response))
        }
    }
}
