//! # Live Threads
//!
//! API data models for [live threads]
//!
//! [live threads]: https://www.reddit.com/r/live/wiki/index

use serde::Deserialize;

/// An update within a live thread.
#[derive(Debug, Deserialize)]
pub struct LiveUpdateData {
    /// The markdown body of the update.
    pub body: String,
    /// The fullname of the update. Note that this does *not* follow the usual tX_base36 pattern.
    pub name: String,
    /// The name of the author of the update (without the `/u/`).
    pub author: String,
    /// When the update was posted.
    pub created: f64,
    /// When the update was posted, in UTC.
    pub created_utc: f64,
    /// The body of the update as HTML.
    pub body_html: String,
    /// Whether this update has been stricken (marked incorrect, but not deleted).
    pub stricken: bool,
    /// The identifier for this update.
    pub id: String,
}

/// A live-updating thread.
#[derive(Debug, Deserialize)]
pub struct LiveThreadData {
    /// The total views? Always seems to be null.
    pub total_views: Option<i32>,
    /// The description for this live thread in markdown.
    pub description: String,
    /// The description for this live thread as HTML.
    pub description_html: String,
    /// When this live thread was created.
    pub created: f64,
    /// The title of this live thread.
    pub title: String,
    /// When this live thread was created, in UTC.
    pub created_utc: f64,
    /// ??
    pub button_cta: String,
    /// The websocket URL to listen for updates live. Expires and may need to be re-fetched.
    /// Is only present if the live thread is actually live.
    pub websocket_url: Option<String>,
    /// The fullname for this live thread.
    #[serde(rename = "name")]
    pub fullname: String,
    /// Whether this live is an announcement?
    pub is_announcement: bool,
    /// The current state of this live thread.
    pub state: LiveThreadState,
    /// ??
    pub announcement_url: String,
    /// Whether the live thread is NSFW.
    pub nsfw: bool,
    /// The current viewier count, if the thread is live.
    pub viewer_count: Option<i32>,
    /// ??
    pub num_times_dismissable: i32,
    /// ??
    pub viewer_count_fuzzed: Option<bool>,
    /// The additional resources for this live thread as HTML.
    pub resources_html: String,
    /// The identifier for this live thread
    pub id: String,
    /// The additional resources for this live thread as markdown.
    pub resources: String,
    /// ??
    pub icon: String,
}

/// The state that a live thread may be in.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "lowercase")]
pub enum LiveThreadState {
    /// The thread is live and can be updated.
    Live,
    /// The thread has closed and will have no more updates.
    Complete,
}
