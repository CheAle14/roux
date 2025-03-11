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
    pub comments: BasicListing<ArticleCommentData>,
}

type Encoded = (serde::de::IgnoredAny, BasicListing<ArticleCommentData>);

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
