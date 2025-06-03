use serde::Deserialize;

use crate::api::FlairId;

/// Represents the possible flair selections, and the current flair selected.
#[derive(Debug, Deserialize)]
pub struct FlairSelection {
    /// The potential flairs
    pub choices: Vec<FlairChoice>,
    /// The current flair
    pub current: FlairCurrentChoice,
}

/// The current flair choice.
///
/// template_id and text may be None if no flair is currently selected.
#[derive(Debug, Deserialize)]
pub struct FlairCurrentChoice {
    /// CSS class
    pub flair_css_class: String,
    /// Position
    pub flair_position: String,
    /// Template ID
    pub flair_template_id: Option<FlairId>,
    /// Text
    pub flair_text: Option<String>,
}

/// A potential flair choice.
#[derive(Debug, Deserialize)]
pub struct FlairChoice {
    /// CSS class
    pub flair_css_class: String,
    /// Position
    pub flair_position: String,
    /// Template ID
    pub flair_template_id: FlairId,
    /// Text
    pub flair_text: String,
    /// Whether the text can be edited
    pub flair_text_editable: bool,
}
