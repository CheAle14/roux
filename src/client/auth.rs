use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::api::comment::created::CreatedCommentData;
use crate::api::me::MeData;
use crate::api::response::{LazyThingCreatedData, MultipleBasicThingsData, PostResponse};
use crate::api::saved::SavedData;
use crate::api::{APIInbox, APISaved, APISubmissions, Friend, ThingId};
use crate::builders::form::FormBuilder;
use crate::builders::submission::SubmissionSubmitBuilder;
use crate::models::inbox::Inbox;
use crate::models::{
    CreatedComment, FromClientAndData, LatestComment, Listing, Message, Saved, Submission,
};
use crate::util::{FeedOption, RouxError};

use super::endpoint::EndpointBuilder;
use super::traits::RedditClient;

type ListSaved = Listing<Saved<AuthedClient>>;

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
    ) -> Result<crate::models::Submission<Self>, RouxError> {
        #[derive(Serialize)]
        struct SubmitRequest<'a> {
            sr: &'a str,
            #[serde(flatten)]
            data: &'a SubmissionSubmitBuilder,
            api_type: &'static str,
        }

        let req = SubmitRequest {
            sr: subreddit_name,
            data: submission,
            api_type: "json",
        };

        let endpoint = EndpointBuilder::new("api/submit");

        let req = || self.0.request(reqwest::Method::POST, &endpoint).form(&req);

        let parsed: crate::api::response::PostResponse<LazyThingCreatedData> =
            self.0.execute(req).await?.json().await?;

        let mut submissions = self
            .get_submissions(&[&parsed.json.data.unwrap().name])
            .await?;
        let rtn = submissions.data.children.pop().unwrap();

        Ok(crate::models::Submission::new(self.clone(), rtn.data))
    }

    /// Adds a friend to a subreddit with the specified type
    #[maybe_async::maybe_async]
    pub async fn add_subreddit_friend(
        &self,
        username: &str,
        typ: &str,
        sub: &str,
    ) -> Result<bool, RouxError> {
        let form = FormBuilder::new().with("name", username).with("type", typ);
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
        let form = FormBuilder::new().with("name", username).with("type", typ);
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
        let form = FormBuilder::new()
            .with("subject", subject)
            .with("text", body)
            .with("to", username);

        self.0.post("api/compose", &form).await
    }

    /// Get user's received messages (includes both read and unread).
    #[maybe_async::maybe_async]
    pub async fn inbox(&self) -> Result<Inbox<Self>, RouxError> {
        let api: APIInbox = self.0.get_json("message/inbox").await?;
        let conv = Listing::new(api, self.clone());
        Ok(conv)
    }

    #[maybe_async::maybe_async]
    async fn _saved(&self, ty: &str, options: Option<FeedOption>) -> Result<ListSaved, RouxError> {
        let mut url = EndpointBuilder::new(format!(
            "user/{}/{ty}",
            self.0.config().username.as_ref().unwrap()
        ));

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        let response: APISaved = self.0.get_json(url).await?;
        let conv = Listing::new(response, self.clone());

        Ok(conv)
    }

    /// Get saved
    #[maybe_async::maybe_async]
    pub async fn saved(&self, options: Option<FeedOption>) -> Result<ListSaved, RouxError> {
        self._saved("saved", options).await
    }

    /// Get upvoted
    #[maybe_async::maybe_async]
    pub async fn upvoted(&self, options: Option<FeedOption>) -> Result<ListSaved, RouxError> {
        self._saved("upvoted", options).await
    }

    /// Get downvoted
    #[maybe_async::maybe_async]
    pub async fn downvoted(&self, options: Option<FeedOption>) -> Result<ListSaved, RouxError> {
        self._saved("downvoted", options).await
    }

    /// Get users unread messages
    #[maybe_async::maybe_async]
    pub async fn unread(&self) -> Result<Inbox<Self>, RouxError> {
        let api: APIInbox = self.0.get_json("message/unread").await?;
        let conv = Listing::new(api, self.clone());
        Ok(conv)
    }

    /// Mark message as read
    #[maybe_async::maybe_async]
    pub async fn mark_read(&self, ids: &ThingId) -> Result<super::req::Response, RouxError> {
        let form = FormBuilder::new().with("id", ids.full());
        self.0.post("api/read_message", &form).await
    }

    /// Mark message as unread
    #[maybe_async::maybe_async]
    pub async fn mark_unread(&self, ids: &ThingId) -> Result<super::req::Response, RouxError> {
        let form = FormBuilder::new().with("id", ids.full());
        self.0.post("api/unread_message", &form).await
    }

    /// Comment
    #[maybe_async::maybe_async]
    async fn _comment<Data: DeserializeOwned, T: FromClientAndData<Self, Data>>(
        &self,
        text: &str,
        parent: &ThingId,
    ) -> Result<T, RouxError> {
        let form = FormBuilder::new()
            .with("text", text)
            .with("parent", parent.full());

        let response: PostResponse<MultipleBasicThingsData<Data>> =
            self.0.post_with_response("api/comment", &form).await?;

        Ok(T::new(
            self.clone(),
            response.json.data.unwrap().assume_single(),
        ))
    }

    /// Adds a comment under a submission or replies to a comment in a submission.
    #[maybe_async::maybe_async]
    pub async fn comment(
        &self,
        text: &str,
        parent: &ThingId,
    ) -> Result<CreatedComment<Self>, RouxError> {
        self._comment(text, parent).await
    }

    /// Adds a reply to an inbox message.
    #[maybe_async::maybe_async]
    pub async fn reply(&self, text: &str, parent: &ThingId) -> Result<Message<Self>, RouxError> {
        self._comment(text, parent).await
    }

    /// Edit a 'thing'
    #[maybe_async::maybe_async]
    pub async fn edit(
        &self,
        text: &str,
        parent: &ThingId,
    ) -> Result<super::req::Response, RouxError> {
        let form = FormBuilder::new()
            .with("text", text)
            .with("thing_id", parent.full());
        self.0.post("api/editusertext", &form).await
    }

    /// Get submissions by id
    #[maybe_async::maybe_async]
    pub async fn get_submissions(&self, ids: &[&ThingId]) -> Result<APISubmissions, RouxError> {
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

    /// Logout
    #[maybe_async::maybe_async]
    pub async fn logout(self) -> Result<(), RouxError> {
        let url = EndpointBuilder::new("https://www.reddit.com/api/v1/revoke_token");

        let form = [("access_token", self.0.config().access_token.to_owned())];

        let response = self
            .0
            .request(reqwest::Method::POST, &url)
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
    async fn post(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &FormBuilder<'_>,
    ) -> Result<super::req::Response, RouxError> {
        self.0.post(endpoint, form).await
    }
}
