use serde::{
    de::{self, Visitor},
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Serialize,
};

use crate::util::ser_map::SerMap;

/// Moderator-related data for the submission.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmissionModerationData {
    /// Whether the post has been approved
    pub approved: bool,
    /// If it has been approved, when was that.
    pub approved_at_utc: Option<f64>,
    /// If it has been approved, the username of the moderator who did.
    pub approved_by: Option<String>,
    /// ??
    pub ban_note: Option<String>,
    /// When this post was banned?
    pub banned_at_utc: Option<f64>,
    /// Which moderator banned it?
    pub banned_by: Option<String>,
    /// Whether further reports to the submission will be disgarded
    pub ignore_reports: bool,
    /// ??
    pub mod_note: Option<String>,
    /// ??
    pub mod_reason_by: Option<String>,
    /// ??
    pub mod_reason_title: Option<String>,
    /// Reports made by subreddit moderators
    pub mod_reports: Vec<SubmissionModeratorReport>,
    /// The number of reports (can sometimes be negative?)
    pub num_reports: i32,
    /// The reason provided for removal, if any.
    pub removal_reason: Option<String>,
    /// Whether the post has been removed
    pub removed: bool,
    /// Who removed the post
    pub removed_by: Option<String>,
    /// Which type of user removed it (presumably an enum of moderator and admin)
    pub removed_by_category: Option<String>,
    /// Whether it was removed for spam
    pub spam: bool,
    /// Reports made by users.
    pub user_reports: Vec<SubmissionUserReport>,
}

pub(super) fn deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<SubmissionModerationData>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let mut map = serde_json::Map::deserialize(deserializer)?;

    match map.remove("can_mod_post") {
        None => Ok(None),
        Some(serde_json::Value::Bool(false)) => Ok(None),
        Some(serde_json::Value::Bool(true)) => {
            let rest = serde_json::Value::Object(map);
            SubmissionModerationData::deserialize(rest)
                .map(Option::Some)
                .map_err(de::Error::custom)
        }
        _ => Err(de::Error::missing_field("can_mod_post")),
    }
}

pub(super) fn serialize<S>(
    value: &Option<SubmissionModerationData>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    let mut map = serializer.serialize_map(None)?;
    match value {
        Some(data) => {
            map.serialize_entry("can_mod_post", &true)?;

            let ser = SerMap::<S>(map);
            data.serialize(ser)
        }
        None => {
            map.serialize_entry("can_mod_post", &false)?;
            map.end()
        }
    }
}

/// Reports by one of the subreddit's moderators.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmissionModeratorReport([String; 2]);

impl SubmissionModeratorReport {
    /// The username of the moderator.
    pub fn moderator(&self) -> &str {
        self.0[0].as_str()
    }

    /// The reason for the moderator's report.
    pub fn reason(&self) -> &str {
        self.0[1].as_str()
    }
}

/// One or more reports performed by anonymous users of the subreddit.
#[derive(Debug, PartialEq)]
pub struct SubmissionUserReport {
    /// The short name of the rule reported for (or a custom reason?)
    pub rule: String,
    /// The number of reports made
    pub count: i32,
    /// ???
    pub unknown1: bool,
    /// ???
    pub unknown2: bool,
}

impl<'de> Deserialize<'de> for SubmissionUserReport {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ReportVisitor;

        impl<'de> Visitor<'de> for ReportVisitor {
            type Value = SubmissionUserReport;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "sequence of string, number and two bools")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let rule = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let count = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let unknown1 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let unknown2 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(SubmissionUserReport {
                    rule,
                    count,
                    unknown1,
                    unknown2,
                })
            }
        }

        deserializer.deserialize_seq(ReportVisitor)
    }
}

impl Serialize for SubmissionUserReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(4))?;
        seq.serialize_element(&self.rule)?;
        seq.serialize_element(&self.count)?;
        seq.serialize_element(&self.unknown1)?;
        seq.serialize_element(&self.unknown2)?;
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_mod_data_de() {
        const DATA: &str = include_str!("is_mod.json");

        #[derive(Deserialize)]
        struct Container {
            #[serde(flatten, deserialize_with = "deserialize")]
            pub mod_data: Option<SubmissionModerationData>,
        }

        let container = serde_json::from_str::<Container>(DATA).unwrap();

        assert_eq!(
            container.mod_data,
            Some(SubmissionModerationData {
                approved: false,
                approved_at_utc: None,
                approved_by: None,
                ban_note: Some(String::from("remove not spam")),
                banned_at_utc: Some(1725096693.0),
                banned_by: Some(String::from("exampleUsername")),
                ignore_reports: false,
                mod_note: None,
                mod_reason_by: None,
                mod_reason_title: None,
                mod_reports: Vec::new(),
                num_reports: 0,
                removal_reason: None,
                removed: true,
                removed_by: Some(String::from("exampleUsername")),
                removed_by_category: Some(String::from("moderator")),
                spam: false,
                user_reports: Vec::new()
            })
        )
    }

    #[test]
    pub fn test_not_mod_data_de() {
        const DATA: &str = include_str!("not_mod.json");

        #[derive(Deserialize)]
        struct Container {
            #[serde(flatten, deserialize_with = "deserialize")]
            pub mod_data: Option<SubmissionModerationData>,
        }

        let container = serde_json::from_str::<Container>(DATA).unwrap();

        assert_eq!(container.mod_data, None);
    }

    #[test]
    pub fn test_user_reports_serde() {
        const DATA: &str = r#"["A rule",5,false,true]"#;

        let reports = serde_json::from_str::<SubmissionUserReport>(DATA).unwrap();

        assert_eq!(reports.rule, "A rule");
        assert_eq!(reports.count, 5);
        assert_eq!(reports.unknown1, false);
        assert_eq!(reports.unknown2, true);

        let ser = serde_json::to_string(&reports).unwrap();

        assert_eq!(ser, DATA);
    }
}
