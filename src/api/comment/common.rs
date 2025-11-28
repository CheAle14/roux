use serde::{
    de::{Expected, Visitor},
    Deserialize, Serialize,
};
use serde_json::Value;

use crate::api::{Distinguished, ThingFullname};

/// Data that is shared between the latest and article comments.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    pub author_flair_type: Option<String>,
    pub author_flair_template_id: Option<String>,
    pub author_fullname: Option<ThingFullname>,
    pub author_is_blocked: bool,
    pub author_patreon_flair: Option<bool>,
    pub author_premium: Option<bool>,
    pub awarders: Vec<Value>,
    pub banned_at_utc: Option<f64>,
    pub banned_by: Option<Value>,
    #[serde(deserialize_with = "crate::util::serde::unescape_html")]
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
    pub distinguished: Distinguished,
    pub downs: i32,
    pub edited: Edited,
    pub gilded: i32,
    pub gildings: Value,
    pub id: String,
    pub ignore_reports: Option<bool>,
    pub is_submitter: bool,
    pub likes: Option<Value>,
    pub link_id: ThingFullname,
    pub locked: bool,
    pub mod_note: Option<Value>,
    pub mod_reason_by: Option<Value>,
    pub mod_reason_title: Option<Value>,
    pub mod_reports: Vec<[String; 2]>,
    pub name: ThingFullname,
    pub no_follow: bool,
    pub num_reports: Option<i32>,
    pub parent_id: ThingFullname,
    pub permalink: String,
    pub removal_reason: Option<Value>,
    pub removed: Option<bool>,
    pub report_reasons: Option<Vec<Value>>,
    pub saved: bool,
    pub score: i32,
    pub score_hidden: bool,
    pub send_replies: bool,
    pub spam: Option<bool>,
    pub stickied: bool,
    pub subreddit: String,
    pub subreddit_id: ThingFullname,
    pub subreddit_name_prefixed: String,
    pub subreddit_type: String,
    pub top_awarded_type: Option<Value>,
    pub total_awards_received: i32,
    pub treatment_tags: Vec<Value>,
    pub unrepliable_reason: Option<Value>,
    pub ups: i32,
    pub user_reports: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Edited {
    EditedAt(f64),
    NotEdited,
}

impl<'de> Deserialize<'de> for Edited {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EditedVistor;

        impl<'de> Visitor<'de> for EditedVistor {
            type Value = Edited;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "boolean 'false' or a float timestamp")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v == false {
                    Ok(Edited::NotEdited)
                } else {
                    Err(E::invalid_value(
                        serde::de::Unexpected::Bool(v),
                        &"boolean 'false'" as &dyn Expected,
                    ))
                }
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Edited::EditedAt(v))
            }
        }

        deserializer.deserialize_any(EditedVistor)
    }
}

impl Serialize for Edited {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Edited::EditedAt(f) => serializer.serialize_f64(*f),
            Edited::NotEdited => serializer.serialize_bool(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::comment::common::Edited;

    #[test]
    fn serde_edited() {
        assert_eq!(
            serde_json::from_str::<Edited>("false").unwrap(),
            Edited::NotEdited
        );
        assert_eq!(
            serde_json::from_str::<Edited>("123.0").unwrap(),
            Edited::EditedAt(123.0)
        );
        assert!(serde_json::from_str::<Edited>("true").is_err());
    }
}
