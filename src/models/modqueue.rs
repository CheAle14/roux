//! Models related to the modqueue.

use crate::{
    api::{subreddit::ModQueueItem, ThingFullname},
    models::{FromClientAndData, LatestComment, Listing, Submission},
};

/// Returned by `/about/modqueue`, a list of things that need moderator review.
pub type Modqueue<T> = Listing<QueueThing<T>>;

/// A thing which needs moderator review
pub enum QueueThing<T> {
    /// A submission
    Submission(Submission<T>),
    /// A comment
    Comment(LatestComment<T>),
}

impl<T> QueueThing<T> {
    /// The thing's fullname
    pub fn name(&self) -> &ThingFullname {
        match self {
            QueueThing::Submission(d) => d.name(),
            QueueThing::Comment(d) => d.name(),
        }
    }

    /// The thing's author
    pub fn author(&self) -> &str {
        match self {
            QueueThing::Submission(d) => d.author(),
            QueueThing::Comment(d) => d.author(),
        }
    }
}

impl<T> FromClientAndData<T, ModQueueItem> for QueueThing<T> {
    fn new(client: T, data: ModQueueItem) -> Self {
        match data {
            ModQueueItem::Submission(d) => Self::Submission(Submission::new(client, d)),
            ModQueueItem::Comment(d) => Self::Comment(LatestComment::new(client, d)),
        }
    }
}
