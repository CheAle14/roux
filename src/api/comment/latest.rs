use serde::{Deserialize, Serialize};

use super::common::CommonCommentData;

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestCommentData {
    #[serde(flatten)]
    pub common: CommonCommentData,
    pub link_author: String,
    pub link_permalink: String,
    pub link_title: String,
    pub link_url: String,
    pub num_comments: i32,
    pub over_18: bool,
    pub quarantine: bool,
}
