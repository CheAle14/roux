use std::sync::{Arc, RwLock};

use reqwest::header::HeaderValue;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::api::comment::APICreatedComments;
use crate::api::me::MeData;
use crate::api::response::{BasicListing, LazyThingCreatedData, MultipleBasicThingsData};
use crate::api::{APIInbox, APISaved, APISubmissions, Friend, ThingId};
use crate::builders::form::FormBuilder;
use crate::builders::submission::SubmissionSubmitBuilder;
use crate::client::{inner::ClientInner, req::*};
use crate::models::inbox::Inbox;
use crate::models::submission::Submissions;
use crate::models::{
    CreatedComment, CreatedCommentWithLinkInfo, FromClientAndData, Listing, Message, Saved,
};
use crate::util::{FeedOption, RouxError};
use crate::Config;

use super::endpoint::EndpointBuilder;
use super::inner::ExecuteError;
use super::traits::RedditClient;

type ListSaved = Listing<Saved<AuthedClient>>;

pub(crate) struct AuthClientInner {
    base: ClientInner,
    access_token: RwLock<HeaderValue>,
}

fn form_auth_header(access_token: &str) -> HeaderValue {
    HeaderValue::from_str(&format!("Bearer {access_token}")).unwrap()
}

impl AuthClientInner {
    pub(crate) fn new(config: Config, access_token: String) -> Result<Self, RouxError> {
        let base = ClientInner::new(config)?;
        let header = form_auth_header(&access_token);
        Ok(Self {
            base,
            access_token: RwLock::new(header),
        })
    }

    pub(crate) fn request(
        &self,
        method: reqwest::Method,
        endpoint: &EndpointBuilder,
    ) -> RequestBuilder {
        let builder = self.base.request(method, endpoint);
        let token = self.access_token.read().unwrap();
        let value: &HeaderValue = &token;
        builder.header(reqwest::header::AUTHORIZATION, value)
    }
}

/// A logged in OAuth client to make privileged requests to Reddit's API.
///
/// Obtain through [`crate::client::OAuthClient::login`]
#[derive(Clone)]
pub struct AuthedClient(Arc<AuthClientInner>);

impl AuthedClient {
    pub(crate) fn new(config: Config, access_token: String) -> Result<Self, RouxError> {
        let inner = AuthClientInner::new(config, access_token)?;
        Ok(Self(Arc::new(inner)))
    }

    /// Get me
    #[maybe_async::maybe_async]
    pub async fn me(&self) -> Result<MeData, RouxError> {
        self.get_json("api/v1/me").await
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

        let parsed: LazyThingCreatedData = self.post_with_response(endpoint, &req).await?;

        let mut submissions = self.get_submissions(&[&parsed.name]).await?;

        Ok(submissions.children.pop().unwrap())
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
            .post_with_response_raw(format!("r/{}/api/friend", sub).as_str(), &form)
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
            .post_with_response_raw(format!("r/{}/api/unfriend", sub).as_str(), &form)
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

        self.post("api/compose", &form).await
    }

    /// Get user's received messages (includes both read and unread).
    #[maybe_async::maybe_async]
    pub async fn inbox(&self) -> Result<Inbox<Self>, RouxError> {
        let api: APIInbox = self.get_json("message/inbox").await?;
        let conv = Listing::new(api, self.clone());
        Ok(conv)
    }

    #[maybe_async::maybe_async]
    async fn _saved(&self, ty: &str, options: Option<FeedOption>) -> Result<ListSaved, RouxError> {
        let mut url = EndpointBuilder::new(format!(
            "user/{}/{ty}",
            self.0.base.config.username.as_ref().unwrap()
        ));

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        let response: APISaved = self.get_json(url).await?;
        let conv = Listing::new(response, self.clone());

        Ok(conv)
    }

    /// Get comments you have sent
    #[maybe_async::maybe_async]
    pub async fn comments(
        &self,
        options: Option<FeedOption>,
    ) -> Result<Listing<CreatedCommentWithLinkInfo<Self>>, RouxError> {
        let mut url = EndpointBuilder::new(format!(
            "user/{}/comments",
            self.0.base.config.username.as_ref().unwrap()
        ));

        if let Some(options) = options {
            options.build_url(&mut url);
        }

        let response: APICreatedComments = self.get_json(url).await?;
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
        let api: APIInbox = self.get_json("message/unread").await?;
        let conv = Listing::new(api, self.clone());
        Ok(conv)
    }

    /// Mark message as read
    #[maybe_async::maybe_async]
    pub async fn mark_read(&self, ids: &ThingId) -> Result<super::req::Response, RouxError> {
        let form = FormBuilder::new().with("id", ids.full());
        self.post("api/read_message", &form).await
    }

    /// Mark message as unread
    #[maybe_async::maybe_async]
    pub async fn mark_unread(&self, ids: &ThingId) -> Result<super::req::Response, RouxError> {
        let form = FormBuilder::new().with("id", ids.full());
        self.post("api/unread_message", &form).await
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

        let response: MultipleBasicThingsData<Data> =
            self.post_with_response("api/comment", &form).await?;

        Ok(T::new(self.clone(), response.assume_single()))
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
        self.post("api/editusertext", &form).await
    }

    /// Get submissions by id
    #[maybe_async::maybe_async]
    pub async fn get_submissions(&self, ids: &[&ThingId]) -> Result<Submissions<Self>, RouxError> {
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

    /// Logout
    #[maybe_async::maybe_async]
    pub async fn logout(self) -> Result<(), RouxError> {
        let url = EndpointBuilder::new("https://www.reddit.com/api/v1/revoke_token");

        let read = self.0.access_token.read().unwrap();
        let form = [("access_token", read.to_str().unwrap())];

        let response = self
            .request(reqwest::Method::POST, &url)
            .basic_auth(
                &self.0.base.config.client_id,
                Some(&self.0.base.config.client_secret),
            )
            .form(&form)
            .send()
            .await?;

        if response.status() == 204 {
            Ok(())
        } else {
            Err(RouxError::status(response))
        }
    }

    pub(crate) fn request(
        &self,
        method: reqwest::Method,
        endpoint: &EndpointBuilder,
    ) -> RequestBuilder {
        self.0.request(method, endpoint)
    }

    #[maybe_async::maybe_async]
    pub(crate) async fn execute<F>(&self, builder: F) -> Result<Response, RouxError>
    where
        F: Fn() -> RequestBuilder,
    {
        let mut has_retried = false;
        loop {
            match self.0.base.execute(&builder).await {
                Ok(response) => return Ok(response),
                Err(ExecuteError::AuthorizationRequired) => {
                    if has_retried {
                        return Err(RouxError::credentials_not_set());
                    }
                    has_retried = true;
                    let mut write = self.0.access_token.write().unwrap();
                    let token = self.0.base.attempt_login().await?;
                    *write = form_auth_header(&token);
                }
                Err(other_error) => return Err(other_error.into()),
            }
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
        let endpoint = endpoint.into();

        let request = || self.request(Method::GET, &endpoint);

        self.execute(request).await
    }

    #[inline(always)]
    #[maybe_async::maybe_async]
    async fn post<T: Serialize>(
        &self,
        endpoint: impl Into<EndpointBuilder>,
        form: &T,
    ) -> Result<super::req::Response, RouxError> {
        let endpoint = endpoint.into();

        let request = || self.request(Method::POST, &endpoint).form(form);

        self.execute(request).await
    }
}
