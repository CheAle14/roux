use crate::api::saved::SavedData;

use super::{FromClientAndData, LatestComment, Submission};

/// Represents an item that has been saved to your account.
pub enum Saved<T> {
    /// A saved submission.
    Submission(super::Submission<T>),
    /// A saved comment.
    Comment(super::LatestComment<T>),
}

impl<T> FromClientAndData<T, SavedData> for Saved<T> {
    fn new(client: T, data: SavedData) -> Self {
        match data {
            SavedData::Comment(comment) => Saved::Comment(LatestComment::new(client, comment)),
            SavedData::Submission(post) => Saved::Submission(Submission::new(client, post)),
        }
    }
}
