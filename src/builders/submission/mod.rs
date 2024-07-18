use serde::{ser::SerializeStruct, Deserialize, Serialize};

/// The type of submission, one of self text, rich text or link.
#[derive(Debug)]
pub enum SubmissionSubmitKind {
    /// A text post
    SelfText {
        /// Markdown formatted text for the post
        text: String,
    },
    /// A rich text post
    RichText {
        /// JSON formatted text for the post
        rich_text: String,
    },
    /// A link post
    Link {
        /// The URL for this link post
        url: String,
        /// Whether previous posts for this link should be ignored
        resubmit: bool,
    },
}

impl Serialize for SubmissionSubmitKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SubmissionSubmitKind::SelfText { text } => {
                let mut start = serializer.serialize_struct("", 2)?;
                start.serialize_field("kind", "self")?;
                start.serialize_field("text", text)?;
                start.end()
            }
            SubmissionSubmitKind::RichText { rich_text } => {
                let mut start = serializer.serialize_struct("", 2)?;
                start.serialize_field("kind", "self")?;
                start.serialize_field("richtext_json", rich_text)?;
                start.end()
            }
            SubmissionSubmitKind::Link { url, resubmit } => {
                let mut start = if *resubmit {
                    let mut start = serializer.serialize_struct("", 3)?;
                    start.serialize_field("resubmit", &true)?;
                    start
                } else {
                    serializer.serialize_struct("", 2)?
                };

                start.serialize_field("kind", "link")?;
                start.serialize_field("url", url)?;
                start.end()
            }
        }
    }
}

/// A builder to gather the data to submit a post
#[derive(Debug, Serialize)]
pub struct SubmissionSubmitBuilder {
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
    pub kind: SubmissionSubmitKind,
    api_type: &'static str,
    validate_on_submit: bool,
}

impl SubmissionSubmitBuilder {
    fn new(title: impl Into<String>, kind: SubmissionSubmitKind) -> Self {
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

    /// Creates a submission builder for a self text post
    pub fn text(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self::new(title, SubmissionSubmitKind::SelfText { text: body.into() })
    }

    /// Creates a submission builder for a link post.
    ///
    /// If `resubmit` is true, the post will be made even if there are other posts for this URL in the subreddit.
    pub fn link(title: impl Into<String>, url: impl Into<String>, resubmit: bool) -> Self {
        Self::new(
            title,
            SubmissionSubmitKind::Link {
                resubmit,
                url: url.into(),
            },
        )
    }

    /// Creates a submission builder for a rich text JSON post.
    pub fn rich_text_json(title: impl Into<String>, json: impl Into<String>) -> Self {
        Self::new(
            title,
            SubmissionSubmitKind::RichText {
                rich_text: json.into(),
            },
        )
    }

    /// Whether comments to the post should be sent to your inbox as messages.
    /// Defaults to `true`
    pub fn with_send_replies(mut self, send_replies: bool) -> Self {
        self.send_replies = send_replies;
        self
    }

    /// Whether the post is marked as NSFW.
    /// Defaults to `false`
    pub fn with_nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = nsfw;
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
            r#"{"title":"Hello world","sendreplies":false,"kind":"self","text":"**Some body** here"}"#,
        );
    }
    #[test]
    pub fn test_url_serialize() {
        let builder =
            super::SubmissionSubmitBuilder::link("Another test", "https://example.com", false)
                .with_send_replies(false)
                .with_nsfw(true);

        let value = serde_json::to_string(&builder).unwrap();
        assert_eq!(
            value,
            r#"{"title":"Another test","sendreplies":false,"nsfw":true,"kind":"link","url":"https://example.com"}"#,
        );
    }
    #[test]
    pub fn test_url_resubmit_serialize() {
        let builder =
            super::SubmissionSubmitBuilder::link("Another test", "https://example.com", true)
                .with_send_replies(false)
                .with_nsfw(true);

        let value = serde_json::to_string(&builder).unwrap();
        assert_eq!(
            value,
            r#"{"title":"Another test","sendreplies":false,"nsfw":true,"resubmit":true,"kind":"link","url":"https://example.com"}"#,
        );
    }
}
