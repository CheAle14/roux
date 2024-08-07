use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::{MaybeReplies, ThingId};

/// Data that is shared between the latest and article comments.
#[derive(Debug, Serialize, Deserialize)]
pub struct CommonCommentData {
    pub all_awardings: Vec<Value>,
    pub approved: Option<bool>,
    pub approved_at_utc: Option<f64>,
    pub approved_by: Option<String>,
    pub archived: bool,
    pub associated_award: Option<Value>,
    pub author: String,
    pub author_flair_background_color: Option<Value>,
    pub author_flair_css_class: Option<Value>,
    pub author_flair_richtext: Option<Value>,
    pub author_flair_text: Option<String>,
    pub author_flair_text_color: Option<Value>,
    pub author_flair_type: String,
    pub author_fullname: ThingId,
    pub author_is_blocked: bool,
    pub author_patreon_flair: bool,
    pub author_premium: bool,
    pub awarders: Vec<Value>,
    pub banned_at_utc: Option<f64>,
    pub banned_by: Option<Value>,
    pub body: String,
    pub body_html: String,
    pub can_gild: bool,
    pub can_mod_post: bool,
    pub collapsed: bool,
    pub collapsed_because_crowd_control: Option<Value>,
    pub collapsed_reason: Option<Value>,
    pub collapsed_reason_code: Option<Value>,
    pub comment_type: Option<Value>,
    pub controversiality: i32,
    pub created: f64,
    pub created_utc: f64,
    pub distinguished: Option<String>,
    pub downs: i32,
    pub edited: bool,
    pub gilded: i32,
    pub gildings: Value,
    pub id: String,
    pub ignore_reports: Option<bool>,
    pub is_submitter: bool,
    pub likes: Option<Value>,
    pub link_id: ThingId,
    pub locked: bool,
    pub mod_note: Option<Value>,
    pub mod_reason_by: Option<Value>,
    pub mod_reason_title: Option<Value>,
    pub mod_reports: Vec<[String; 2]>,
    pub name: ThingId,
    pub no_follow: bool,
    pub num_reports: Option<i32>,
    pub parent_id: ThingId,
    pub permalink: String,
    pub removal_reason: Option<Value>,
    pub removed: Option<bool>,
    pub replies: MaybeReplies,
    pub report_reasons: Option<Vec<Value>>,
    pub saved: bool,
    pub score: i32,
    pub score_hidden: bool,
    pub send_replies: bool,
    pub spam: Option<bool>,
    pub stickied: bool,
    pub subreddit: String,
    pub subreddit_id: ThingId,
    pub subreddit_name_prefixed: String,
    pub subreddit_type: String,
    pub top_awarded_type: Option<Value>,
    pub total_awards_received: i32,
    pub treatment_tags: Vec<Value>,
    pub unrepliable_reason: Option<Value>,
    pub ups: i32,
    pub user_reports: Vec<Value>,
}
