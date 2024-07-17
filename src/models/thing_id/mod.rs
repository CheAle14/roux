//! # Thing Id
//! A thing id represents a complete identifier to a particular object, encoding both its kind and a base-36 identifier.

use serde::{
    de::{Error, Visitor},
    Deserialize, Serialize,
};

/// A wrapper for a thing id, which is a kind and base-36 identifier. The possible kinds include:
/// - t1_ - Comment
/// - t2_ - Account
/// - t3_ - Link
/// - t4_ - Message
/// - t5_ - Subreddit
/// - t6_ - Award
/// - t8_ - PromoCampaign
#[derive(Debug, Clone, Serialize)]
pub struct ThingId(String);

impl<'a> TryFrom<&'a str> for ThingId {
    type Error = ();

    fn try_from(thing_id: &'a str) -> Result<Self, Self::Error> {
        Self::validate(thing_id)?;

        Ok(Self(thing_id.to_owned()))
    }
}

impl TryFrom<String> for ThingId {
    type Error = ();

    fn try_from(thing_id: String) -> Result<Self, Self::Error> {
        Self::validate(&thing_id)?;

        Ok(Self(thing_id))
    }
}

impl ThingId {
    fn validate(thing_id: &str) -> Result<(), ()> {
        let (kind, _) = thing_id.split_once('_').ok_or(())?;
        if kind.len() != 2 || !kind.starts_with("t") {
            Err(())
        } else {
            Ok(())
        }
    }

    /// Returns the kind and id separately
    #[inline(always)]
    pub fn split(&self) -> (&str, &str) {
        self.0.split_once('_').expect("validated at input")
    }

    /// Returns just the kind, e.g. `t1`
    #[inline(always)]
    pub fn kind(&self) -> &str {
        self.split().0
    }

    /// Returns just the id, e.g. `1e5leyy`
    #[inline(always)]
    pub fn id(&self) -> &str {
        self.split().1
    }

    /// Returns the full thing id
    #[inline(always)]
    pub fn full(&self) -> &str {
        &self.0
    }
}

struct ThingIdVisitor;

impl<'de> Visitor<'de> for ThingIdVisitor {
    type Value = ThingId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string in the format `t1_1e5leyy`")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        ThingId::try_from(v).map_err(|()| E::custom("invalid thing id"))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        ThingId::try_from(v).map_err(|()| E::custom("invalid thing id"))
    }
}

impl<'de> Deserialize<'de> for ThingId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = ThingIdVisitor;
        deserializer.deserialize_string(visitor)
    }
}
