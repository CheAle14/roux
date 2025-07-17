use serde::{Deserialize, Deserializer};

pub fn unescape_html<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let text = String::deserialize(deserializer)?;

    match html_escape::decode_html_entities(&text) {
        std::borrow::Cow::Borrowed(_) => Ok(text),
        std::borrow::Cow::Owned(decoded) => Ok(decoded),
    }
}
