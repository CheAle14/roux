//! # Responses
//! Base responses

use serde::{Deserialize, Deserializer, Serialize};

use crate::api::ThingFullname;

/// Basic structure of a Reddit response.
/// See: <https://github.com/reddit-archive/reddit/wiki/JSON>
#[derive(Serialize, PartialEq, Deserialize, Debug)]
pub struct BasicThing<T> {
    /// An identifier that specifies the type of object that this is.
    pub kind: Option<String>,
    /// The data contained by this struct. This will vary depending on the type parameter
    /// because each endpoint returns different contents.
    pub data: T,
}

/// JSON list response.
#[derive(Serialize, PartialEq, Deserialize, Default, Debug)]
pub struct Listing<T> {
    /// Modhash
    pub modhash: Option<String>,
    /// The number of children in the listing.
    pub dist: Option<i32>,
    /// The fullname of the listing that follows after this page.
    pub after: Option<ThingFullname>,
    /// The fullname of the listing that follows before this page.
    pub before: Option<ThingFullname>,
    /// A list of `things` that this Listing wraps.
    pub children: Vec<T>,
}

/// JSON list response.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListingNotFullname<T> {
    /// Modhash
    pub modhash: Option<String>,
    /// The number of children in the listing.
    pub dist: Option<i32>,
    /// The arbitrary ID of the listing that follows after this page.
    pub after: Option<String>,
    /// The arbitrary of the listing that follows before this page.
    pub before: Option<String>,
    /// A list of `things` that this Listing wraps.
    pub children: Vec<T>,
}

/// Note that `api_type=json` must be passed as a form param to get this as the response.
/// Otherwise you get weird jQuery stuff.
#[derive(Deserialize, Debug)]
pub(crate) struct PostResponse<T> {
    pub json: PostResponseInner<T>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PostResponseInner<T> {
    #[serde(deserialize_with = "crate::util::defaults::null_to_empty")]
    pub errors: Vec<ApiError>,
    pub data: Option<T>,
}

/// Represents a particular error returned by Reddit.
///
/// An example may be:
///
/// ```json
///    [
///     "RATELIMIT",
///     "you are doing that too much. try again in 34 minutes.",
///     "vdelay"
///    ]
/// ```
#[derive(Debug, Deserialize)]
pub struct ApiError(pub [String; 3]);

/// A response for something that has been created, but without its actual data.
#[derive(Deserialize, Debug)]
pub(crate) struct LazyThingCreatedData {
    #[allow(unused)]
    pub id: String,
    pub name: ThingFullname,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MultipleBasicThingsData<T> {
    pub things: Vec<BasicThing<T>>,
}

impl<T> MultipleBasicThingsData<T> {
    pub fn assume_single(self) -> T {
        self.things.into_iter().next().unwrap().data
    }
}

/// A basic-thing listing, but whose inner type is not wrapped in a basic thing.
pub type OuterBasicListing<T> = BasicThing<Listing<T>>;

/// Often times a basic thing will have this structure.
pub type BasicListing<T> = BasicThing<Listing<BasicThing<T>>>;
