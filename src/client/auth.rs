use serde::Serialize;

use crate::models::me::MeData;
use crate::models::{Friend, Inbox, Saved};
use crate::submission::SubmissionData;
use crate::util::{FeedOption, RouxError};
use crate::{builders::submission::SubmissionSubmitBuilder, Submissions, ThingId};

use super::endpoint::EndpointBuilder;
use super::traits::RedditClient;

/// A logged in OAuth client to make privileged requests to Reddit's API.
///
/// Obtain through [`crate::client::OAuthClient::login`]
#[derive(Clone)]
pub struct AuthedClient(pub(crate) super::OAuthClient);

impl AuthedClient {
    /// Get me
    #[maybe_async::maybe_async]
    pub async fn me(&self) -> Result<MeData, RouxError> {
        self.0.get_json("api/v1/me").await
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

        let parsed: crate::response::PostResponse =
            self.0.post_with_response("api/submit", &req).await?;

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
        let resp: Friend = self
            .0
            .post_with_response(format!("r/{}/api/friend", sub).as_str(), &form)
            .await?;

        Ok(resp.success)
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
        let resp: Friend = self
            .0
            .post_with_response(format!("r/{}/api/unfriend", sub).as_str(), &form)
            .await?;
        Ok(resp.success)
    }

    /// Compose message
    #[maybe_async::maybe_async]
    pub async fn compose_message(
        &self,
        username: &str,
        subject: &str,
        body: &str,
    ) -> Result<super::req::Response, RouxError> {
        let form = [
            ("api_type", "json"),
            ("subject", subject),
            ("text", body),
            ("to", username),
        ];

        self.0.post("api/compose", &form).await
    }

    /// Get user's submitted posts.
    #[maybe_async::maybe_async]
    pub async fn inbox(&self) -> Result<Inbox, RouxError> {
        Ok(self.0.get("message/inbox").await?.json::<Inbox>().await?)
    }

    /// Get saved
    #[maybe_async::maybe_async]
    pub async fn saved(&self, options: Option<FeedOption>) -> Result<Saved, RouxError> {
        let mut url = EndpointBuilder::new(format!(
            "user/{}/saved",
            self.0.config().username.as_ref().unwrap()
        ));

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        self.0.get_json(url).await
    }

    /// Get upvoted
    #[maybe_async::maybe_async]
    pub async fn upvoted(&self, options: Option<FeedOption>) -> Result<Saved, RouxError> {
        let mut url = EndpointBuilder::new(format!(
            "user/{}/upvoted",
            self.0.config().username.as_ref().unwrap()
        ));

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        self.0.get_json(url).await
    }

    /// Get downvoted
    #[maybe_async::maybe_async]
    pub async fn downvoted(&self, options: Option<FeedOption>) -> Result<Saved, RouxError> {
        let mut url = EndpointBuilder::new(format!(
            "user/{}/downvoted",
            self.0.config().username.as_ref().unwrap()
        ));

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        self.0.get_json(url).await
    }

    /// Get users unread messages
    #[maybe_async::maybe_async]
    pub async fn unread(&self) -> Result<Inbox, RouxError> {
        self.0.get_json("message/unread").await
    }

    /// Mark messages as read
    #[maybe_async::maybe_async]
    pub async fn mark_read(&self, ids: &ThingId) -> Result<super::req::Response, RouxError> {
        let form = [("id", ids.full())];
        self.0.post("api/read_message", &form).await
    }

    /// Mark messages as unread
    #[maybe_async::maybe_async]
    pub async fn mark_unread(&self, ids: &str) -> Result<super::req::Response, RouxError> {
        let form = [("id", ids)];
        self.0.post("api/unread_message", &form).await
    }

    /// Comment
    #[maybe_async::maybe_async]
    pub async fn comment(
        &self,
        text: &str,
        parent: &ThingId,
    ) -> Result<super::req::Response, RouxError> {
        let form = [("text", text), ("parent", parent.full())];
        self.0.post("api/comment", &form).await
    }

    /// Edit a 'thing'
    #[maybe_async::maybe_async]
    pub async fn edit(
        &self,
        text: &str,
        parent: &ThingId,
    ) -> Result<super::req::Response, RouxError> {
        let form = [("text", text), ("thing_id", parent.full())];
        self.0.post("api/editusertext", &form).await
    }

    /// Get submissions by id
    #[maybe_async::maybe_async]
    pub async fn get_submissions(&self, ids: &[&ThingId]) -> Result<Submissions, RouxError> {
        let mut ids = ids.iter().map(|id| id.full());
        let mut url = format!("by_id/");
        url.push_str(ids.next().unwrap());
        for next in ids {
            url.push(',');
            url.push_str(next);
        }

        let url = EndpointBuilder::new(url);

        self.0.get_json(url).await
    }

    /// Report
    #[maybe_async::maybe_async]
    pub async fn report(
        &self,
        id: &ThingId,
        reason: &str,
    ) -> Result<super::req::Response, RouxError> {
        let form = [("id", id.full()), ("reason", reason)];
        self.0.post("api/report", &form).await
    }

    /// Logout
    #[maybe_async::maybe_async]
    pub async fn logout(self) -> Result<(), RouxError> {
        let url = "https://www.reddit.com/api/v1/revoke_token";

        let form = [("access_token", self.0.config().access_token.to_owned())];

        let response = self
            .0
            .request(reqwest::Method::POST, url)
            .basic_auth(
                &self.0.config().client_id,
                Some(&self.0.config().client_secret),
            )
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

impl RedditClient for AuthedClient {
    #[inline(always)]
    #[maybe_async::maybe_async]
    async fn get(
        &self,
        endpoint: impl Into<EndpointBuilder>,
    ) -> Result<super::req::Response, RouxError> {
        self.0.get(endpoint).await
    }

    #[inline(always)]
    #[maybe_async::maybe_async]
    async fn post<T: Serialize>(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &T,
    ) -> Result<super::req::Response, RouxError> {
        self.0.post(endpoint, form).await
    }
}
