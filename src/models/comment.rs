use crate::api::comment::CommentData;

/// A Comment on a [`crate::models::Submission`]
pub struct Comment<T> {
    client: T,
    data: CommentData,
}

impl<T> Comment<T> {
    /// Creates a new comment
    pub fn new(client: T, data: CommentData) -> Self {
        Self { client, data }
    }
}
