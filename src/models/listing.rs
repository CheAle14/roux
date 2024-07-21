use crate::api::{response::BasicListing, response::Listing as APIListing, ThingId};

/// Represents a view of a list of some thing `T`.
pub struct Listing<T> {
    /// The full name of the thing which comes before this view.
    pub before: Option<ThingId>,
    /// The full name of the thing which comes after this view.
    pub after: Option<ThingId>,
    /// The items within this view
    pub children: Vec<T>,
    /// How many items were present
    pub dist: Option<i32>,
    /// A mod hash
    pub modhash: Option<String>,
}

impl<TModel> Listing<TModel> {
    pub(crate) fn new<TApi, F>(listing: BasicListing<TApi>, convertor: F) -> Self
    where
        F: Fn(TApi) -> TModel,
    {
        let APIListing {
            modhash,
            dist,
            after,
            before,
            children,
        } = listing.data;

        let children: Vec<_> = children
            .into_iter()
            .map(|basic| convertor(basic.data))
            .collect();

        Self {
            before,
            after,
            children,
            dist,
            modhash,
        }
    }
}

impl<T> IntoIterator for Listing<T> {
    type Item = T;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}
