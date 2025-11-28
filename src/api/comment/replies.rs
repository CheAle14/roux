use serde::{
    de::{
        value::{MapAccessDeserializer, SeqAccessDeserializer},
        Visitor,
    },
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::api::{comment::article::ArticleCommentOrMoreComments, response::OuterBasicListing};

/// The article an article has, or empty if it has none.
#[derive(Debug, PartialEq)]
pub enum ArticleReplies {
    /// It has replies.
    Replies(OuterBasicListing<ArticleCommentOrMoreComments>),
    /// It does not.
    Empty,
}

impl Serialize for ArticleReplies {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ArticleReplies::Replies(listing) => listing.serialize(serializer),
            ArticleReplies::Empty => "".serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ArticleReplies {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReplyVisitor;

        impl<'de> Visitor<'de> for ReplyVisitor {
            type Value = ArticleReplies;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("basic listing of article replies, or an empty string")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let deser = MapAccessDeserializer::new(map);

                let listing =
                    <OuterBasicListing<ArticleCommentOrMoreComments> as Deserialize>::deserialize(
                        deser,
                    )?;

                Ok(ArticleReplies::Replies(listing))
            }

            fn visit_string<E>(self, _v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ArticleReplies::Empty)
            }

            fn visit_str<E>(self, _v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ArticleReplies::Empty)
            }

            fn visit_borrowed_str<E>(self, _v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ArticleReplies::Empty)
            }
        }

        deserializer.deserialize_any(ReplyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::api::{
        comment::{
            article::{ArticleCommentOrMoreComments, MoreCommentData},
            replies::ArticleReplies,
        },
        response::{BasicThing, Listing},
        ThingFullname,
    };

    #[derive(Deserialize)]
    struct Response {
        replies: ArticleReplies,
    }

    #[test]
    fn deserialize_empty_replies() {
        static NO_REPLIES: &str = r#"{
            "replies": ""
        }"#;

        let response: Response = serde_json::from_str(NO_REPLIES).unwrap();
        assert_eq!(response.replies, ArticleReplies::Empty);
    }

    #[test]
    fn maybe_replies_deserializes() {
        static HAS_REPLIES: &str = r#"{
            "replies": {
                "kind": "Listing",
                "data": {
                    "children": [{
                        "kind": "more",
                        "data": {
                            "id": "abc123",
                            "name": "t1_abc123",
                            "parent_id": "t3_xyz123",
                            "count": 123,
                            "depth": 0
                        }
                    }]
                }
            }
        }"#;

        let response: Response = serde_json::from_str(HAS_REPLIES).unwrap();
        assert_eq!(
            response.replies,
            ArticleReplies::Replies(BasicThing {
                kind: Some(String::from("Listing")),
                data: Listing {
                    modhash: None,
                    dist: None,
                    after: None,
                    before: None,
                    children: vec![ArticleCommentOrMoreComments::More(MoreCommentData {
                        id: String::from("abc123"),
                        name: ThingFullname::try_from("t1_abc123").unwrap(),
                        parent_id: ThingFullname::try_from("t3_xyz123").unwrap(),
                        count: 123,
                        depth: 0
                    })]
                }
            })
        );
    }
}
