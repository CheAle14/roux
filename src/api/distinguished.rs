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
        }

        deserializer.deserialize_any(DistinguishedVisitor)
    }
}
