//! # Inbox Responses
use serde::Deserialize;

use crate::{api::response::BasicListing, api::ThingFullname};

/// InboxItem
#[derive(Debug, Deserialize)]
pub struct InboxData {
    /// ID
    pub id: String,
    /// Subject
    pub subject: String,
    /// Was comment
    pub was_comment: bool,
    /// Author
    pub author: Option<String>,
    /// Parent ID
    pub parent_id: Option<ThingFullname>,
    /// Sub name
    pub subreddit_name_prefixed: Option<String>,
    /// New
    pub new: bool,
    /// ???
    pub r#type: String,
    /// Body
    pub body: String,
    /// Dest
    pub dest: String,
    /// Body HTML
    pub body_html: String,
    /// Name
    pub name: ThingFullname,
    /// Created
    pub created: f64,
    /// Created (UTC)
    pub created_utc: f64,
    /// Context
    pub context: String,
    pub first_message_name: Option<ThingFullname>,
}

/// Inbox
pub type APIInbox = BasicListing<InboxData>;
