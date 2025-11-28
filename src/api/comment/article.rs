use serde::{Deserialize, Serialize};

use crate::api::{
    comment::replies::ArticleReplies,
    response::{BasicListing, OuterBasicListing},
    submission::SubmissionData,
    ThingFullname,
};

use super::common::CommonCommentData;

/// A comment to a submission, or a reply thereof.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticleCommentData {
    /// Shared data with other comment-like items
    #[serde(flatten)]
    pub common: CommonCommentData,
    /// How deep this comment is from the top-level
    pub depth: i32,
    /// The comment's replies
    pub replies: ArticleReplies,
}

#[derive(Debug)]
pub struct ArticleCommentsResponse {
    pub comments: OuterBasicListing<ArticleCommentOrMoreComments>,
}

type Encoded = (
    serde::de::IgnoredAny,
    OuterBasicListing<ArticleCommentOrMoreComments>,
);

impl<'de> Deserialize<'de> for ArticleCommentsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (_, comments) = Encoded::deserialize(deserializer)?;
        Ok(ArticleCommentsResponse { comments })
    }
}

#[derive(Debug)]
pub struct ArticleCommentsResponseWithoutComments {
    pub submission: SubmissionData,
}

type EncodedNoComments = (BasicListing<SubmissionData>, serde::de::IgnoredAny);

impl<'de> Deserialize<'de> for ArticleCommentsResponseWithoutComments {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (submission, _) = EncodedNoComments::deserialize(deserializer)?;
        Ok(ArticleCommentsResponseWithoutComments {
            submission: submission.data.children.into_iter().next().unwrap().data,
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MoreCommentData {
    pub id: String,
    pub name: ThingFullname,
    pub parent_id: ThingFullname,
    pub count: i32,
    pub depth: i32,
}

/// Represents an article comment, or a more comments marker
#[derive(Debug, PartialEq, Serialize, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum ArticleCommentOrMoreComments {
    /// A comment or reply
    #[serde(rename = "t1")]
    Comment(ArticleCommentData),
    /// More comments to load
    #[serde(rename = "more")]
    More(MoreCommentData),
}
