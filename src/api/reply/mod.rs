//! # Subreddit Comment Responses
use serde::{Deserialize, Serialize};

use crate::api::comment::article::ArticleCommentData;
use crate::api::response::BasicListing;

/// Doc
pub type Replies = BasicListing<ArticleCommentData>;

/// Replies can be more comments or an empty string
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeReplies {
    /// Reply
    Reply(Replies),
    /// String
    Str(String),
}
