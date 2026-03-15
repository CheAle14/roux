use serde::{Deserialize, Serialize};

use crate::api::{
    comment::latest::LatestCommentData, response::OuterBasicListing, submission::SubmissionData,
};

/// The raw response for modqueue
pub type ModQueueItems = OuterBasicListing<ModQueueItem>;

/// The raw modqueue items
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum ModQueueItem {
    /// A comment
    #[serde(rename = "t1")]
    Comment(LatestCommentData),
    /// A submission
    #[serde(rename = "t3")]
    Submission(SubmissionData),
}
