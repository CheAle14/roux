use serde::{Deserialize, Serialize};

use super::common::CommonCommentData;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedCommentData {
    #[serde(flatten)]
    pub common: CommonCommentData,
    pub rte_mode: String,
}
