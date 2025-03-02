use std::collections::HashMap;

use serde::Deserialize;

/// A subreddit's removal reasons, and the order they appear in.
#[derive(Debug, Deserialize)]
pub struct SubredditRemovalReasons {
    /// The removal reasons. Key is the reason's ID.
    pub data: HashMap<String, RemovalReason>,
    /// The order of the reasons, each entry is the reason's ID.
    pub order: Vec<String>,
}

/// A subreddit's removal reason.
#[derive(Debug, Deserialize)]
pub struct RemovalReason {
    /// The message for the removal reason.
    pub message: String,
    /// The unique ID for this removal reason.
    pub id: String,
    /// A title shown to moderators
    pub title: String,
}
