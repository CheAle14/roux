use serde::{ser::SerializeStruct, Serialize};

/// Payload for a text-only post
#[derive(Debug, Clone, Serialize)]
pub struct PayloadSelfText {
    kind: &'static str,
    text: String,
}

/// Payload for a rich text post
#[derive(Debug, Clone, Serialize)]
pub struct PayloadRichText {
    kind: &'static str,
    rich_text: String,
}

/// Payload for a link post
#[derive(Debug, Clone, Serialize)]
pub struct PayloadLink {
    kind: &'static str,
    url: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    resubmit: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

/// A builder to gather the data to submit a post
#[derive(Debug, Clone, Serialize)]
pub struct SubmissionSubmitBuilder<Kind> {
    title: String,
    #[serde(rename = "sendreplies")]
    send_replies: bool,
    nsfw: bool,
    spoiler: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    flair_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flair_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discussion_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    draft_id: Option<String>,
    /// The submission kind
    #[serde(flatten)]
    pub kind: Kind,
    api_type: &'static str,
    validate_on_submit: bool,
}

impl<Kind> SubmissionSubmitBuilder<Kind> {
    fn new(title: impl Into<String>, kind: Kind) -> Self {
        Self {
            title: title.into(),
            kind,
            send_replies: true,
            nsfw: false,
            spoiler: false,
            flair_id: None,
            flair_text: None,
            collection_id: None,
            discussion_type: None,
            draft_id: None,
            api_type: "json",
            validate_on_submit: false,
        }
    }
}

impl SubmissionSubmitBuilder<PayloadSelfText> {
    /// Creates a submission builder for a self text post
    pub fn text(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self::new(
            title,
            PayloadSelfText {
                kind: "self",
                text: body.into(),
            },
        )
    }
}

impl SubmissionSubmitBuilder<PayloadRichText> {
    /// Creates a submission builder for a rich text JSON post.
    pub fn rich_text_json(title: impl Into<String>, json: impl Into<String>) -> Self {
        Self::new(
            title,
            PayloadRichText {
                kind: "self",
                rich_text: json.into(),
            },
        )
    }
}

impl SubmissionSubmitBuilder<PayloadLink> {
    /// Creates a submission builder for a link post.
    pub fn link(title: impl Into<String>, url: impl Into<String>) -> Self {
        Self::new(
            title,
            PayloadLink {
                kind: "link",
                url: url.into(),
                resubmit: false,
                text: None,
            },
        )
    }

    /// If `resubmit` is true, the post will be made even if there are other posts for this URL in the subreddit.
    pub fn with_resubmit(mut self, resubmit: bool) -> Self {
        self.kind.resubmit = resubmit;
        self
    }

    /// Adds a markdown body for this link post
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.kind.text = Some(text.into());
        self
    }
}

impl<Kind> SubmissionSubmitBuilder<Kind> {
    /// Whether comments to the post should be sent to your inbox as messages.
    /// Defaults to `true`
    pub fn with_send_replies(mut self, send_replies: bool) -> Self {
        self.send_replies = send_replies;
        self
    }

    /// Whether the post is marked as a spoiler.
    /// Defaults to `false`
    pub fn with_spoiler(mut self, spoiler: bool) -> Self {
        self.spoiler = spoiler;
        self
    }

    /// Whether the post is marked as NSFW.
    /// Defaults to `false`
    pub fn with_nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;
        self
    }

    /// Specifies the flair template ID used for the submission.
    pub fn with_flair_id(mut self, flair_id: impl Into<String>) -> Self {
        self.flair_id = Some(flair_id.into());
        self
    }

    /// Specifies the flair text used for the submission.
    pub fn with_flair_text(mut self, flair_text: impl Into<String>) -> Self {
        self.flair_text = Some(flair_text.into());
        self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_self_text_serialize() {
        let builder = super::SubmissionSubmitBuilder::text("Hello world", "**Some body** here")
            .with_send_replies(false);

        let value = serde_json::to_string(&builder).unwrap();
        assert_eq!(
            value,
            r#"{"title":"Hello world","sendreplies":false,"nsfw":false,"spoiler":false,"kind":"self","text":"**Some body** here","api_type":"json","validate_on_submit":false}"#,
        );
    }
    #[test]
    pub fn test_url_serialize() {
        let builder = super::SubmissionSubmitBuilder::link("Another test", "https://example.com")
            .with_send_replies(false)
            .with_spoiler(true)
            .with_nsfw(true);

        let value = serde_json::to_string(&builder).unwrap();
        assert_eq!(
            value,
            r#"{"title":"Another test","sendreplies":false,"nsfw":true,"spoiler":true,"kind":"link","url":"https://example.com","api_type":"json","validate_on_submit":false}"#,
        );
    }
    #[test]
    pub fn test_url_resubmit_serialize() {
        let builder = super::SubmissionSubmitBuilder::link("Another test", "https://example.com")
            .with_resubmit(true)
            .with_send_replies(false)
            .with_nsfw(true);

        let value = serde_json::to_string(&builder).unwrap();
        assert_eq!(
            value,
            r#"{"title":"Another test","sendreplies":false,"nsfw":true,"spoiler":false,"kind":"link","url":"https://example.com","resubmit":true,"api_type":"json","validate_on_submit":false}"#,
        );
    }
    #[test]
    pub fn test_url_text_serialize() {
        let builder = super::SubmissionSubmitBuilder::link("Another test", "https://example.com")
            .with_resubmit(true)
            .with_send_replies(false)
            .with_text("hello world")
            .with_nsfw(true);

        let value = serde_json::to_string(&builder).unwrap();
        assert_eq!(
            value,
            r#"{"title":"Another test","sendreplies":false,"nsfw":true,"spoiler":false,"kind":"link","url":"https://example.com","resubmit":true,"text":"hello world","api_type":"json","validate_on_submit":false}"#,
        );
    }
}
