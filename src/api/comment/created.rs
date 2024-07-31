use serde::{Deserialize, Serialize};

use super::common::CommonCommentData;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedCommentData {
    #[serde(flatten)]
    pub common: CommonCommentData,
    pub rte_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedCommentWithLinkInfoData {
    #[serde(flatten)]
    pub common: CommonCommentData,
    pub rte_mode: String,
    // TODO: de-duplicate this and LatestCommentData.
    pub link_author: String,
    pub link_permalink: String,
    pub link_title: String,
    pub link_url: String,
    pub num_comments: i32,
    pub over_18: bool,
    pub quarantine: bool,
}
