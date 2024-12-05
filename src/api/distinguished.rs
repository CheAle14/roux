use serde::{de::Visitor, Deserialize, Serialize};

/// The ways that a thing could be distinguished
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Distinguished {
    /// It is not distinguished
    None,
    /// It is distinguished as a moderator [M]
    Moderator,
    /// It is distinguished as an admin [A]
    Admin,
    /// It is distinguished in a special way, e.g. admin emeritus
    Special,
}

impl Serialize for Distinguished {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Distinguished::None => serializer.serialize_none(),
            Distinguished::Moderator => serializer.serialize_str("moderator"),
            Distinguished::Admin => serializer.serialize_str("admin"),
            Distinguished::Special => serializer.serialize_str("special"),
        }
    }
}

impl<'de> Deserialize<'de> for Distinguished {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DistinguishedVisitor;

        impl<'de> Visitor<'de> for DistinguishedVisitor {
            type Value = Distinguished;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "null or 'moderator', 'admin' or 'special'")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Distinguished::None)
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "moderator" => Ok(Distinguished::Moderator),
                    "admin" => Ok(Distinguished::Admin),
                    "special" => Ok(Distinguished::Special),
                    _ => Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(v),
                        &"moderator, admin or special",
                    )),
                }
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Distinguished::None)
            }
        }

        deserializer.deserialize_any(DistinguishedVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::api::Distinguished;

    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        pub distinguished: Distinguished,
    }

    #[test]
    pub fn test_null_is_none() {
        const JSON: &str = r#"{"distinguished":null}"#;

        let value: TestStruct = serde_json::from_str(JSON).unwrap();
        assert_eq!(value.distinguished, Distinguished::None);

        let back = serde_json::to_string(&value).unwrap();

        assert_eq!(back, JSON);
    }

    #[test]
    pub fn test_moderator() {
        const JSON: &str = r#"{"distinguished":"moderator"}"#;

        let value: TestStruct = serde_json::from_str(JSON).unwrap();
        assert_eq!(value.distinguished, Distinguished::Moderator);

        let back = serde_json::to_string(&value).unwrap();

        assert_eq!(back, JSON);
    }

    #[test]
    pub fn test_admin() {
        const JSON: &str = r#"{"distinguished":"admin"}"#;

        let value: TestStruct = serde_json::from_str(JSON).unwrap();
        assert_eq!(value.distinguished, Distinguished::Admin);

        let back = serde_json::to_string(&value).unwrap();

        assert_eq!(back, JSON);
    }

    #[test]
    pub fn test_special() {
        const JSON: &str = r#"{"distinguished":"special"}"#;

        let value: TestStruct = serde_json::from_str(JSON).unwrap();
        assert_eq!(value.distinguished, Distinguished::Special);

        let back = serde_json::to_string(&value).unwrap();

        assert_eq!(back, JSON);
    }
}
