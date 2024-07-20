use serde::{Deserialize, Serialize};

use crate::api::{response::BasicListing, submission::SubmissionData};

use super::common::CommonCommentData;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleCommentData {
    #[serde(flatten)]
    pub common: CommonCommentData,
    pub depth: i32,
}

#[derive(Debug)]
pub struct ArticleCommentsResponse {
    pub submission: BasicListing<SubmissionData>,
    pub comments: BasicListing<ArticleCommentData>,
}

type Encoded = (
    BasicListing<SubmissionData>,
    BasicListing<ArticleCommentData>,
);

impl<'de> Deserialize<'de> for ArticleCommentsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (submission, comments) = Encoded::deserialize(deserializer)?;
        Ok(ArticleCommentsResponse {
            submission,
            comments,
        })
    }
}
