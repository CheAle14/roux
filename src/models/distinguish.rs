use serde::{Deserialize, Serialize};

/// The manner in which a comment or submission has been distinguished.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Distinguish {
    /// It has not been distinguished.
    #[serde(rename = "no")]
    None,
    /// It has been distinguished as a moderator, requires mod permissions in the subreddit to set.
    #[serde(rename = "yes")]
    Moderator,
    /// It has been distinguished as an Admin, requires admin account.
    #[serde(rename = "admin")]
    Admin,
    /// A special per-user distinguish type.
    #[serde(rename = "special")]
    Special,
}
