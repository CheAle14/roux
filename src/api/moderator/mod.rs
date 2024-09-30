//! # Subreddit Moderator Responses
use serde::Deserialize;

use crate::api::response::{BasicThing, Listing};

/// ModeratorsData
#[derive(Debug, Deserialize)]
pub struct ModeratorData {
    /// The ID of the moderator
    pub id: String,
    /// The name of the moderator
    pub name: String,
    /// Author flair text
    pub author_flair_text: Option<String>,
    /// Mod permissions
    pub mod_permissions: Option<Vec<String>>,
}

/// Moderators
pub type Moderators = BasicThing<Listing<ModeratorData>>;
