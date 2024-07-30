use serde::{Deserialize, Deserializer};

/// String function for serde defaults.
pub fn default_string() -> String {
    "".to_string()
}

pub fn null_to_empty<'de, D, T>(de: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let key = Option::<Vec<T>>::deserialize(de)?;
    let v = match key {
        Some(v) => v,
        None => Vec::new(),
    };
    Ok(v)
}
