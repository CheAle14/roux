//! # Me Responses

use serde::Deserialize;

use crate::api::comment::CommentData;
use crate::api::response::BasicListing;
use crate::api::submission::SubmissionData;

/// A saved item can be a comment or post
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SavedData {
    /// Post
    Submission(SubmissionData),
    /// Comment
    Comment(CommentData),
}

/// Saved listing
pub type Saved = BasicListing<SavedData>;
