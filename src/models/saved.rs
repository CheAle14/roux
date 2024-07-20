/// Represents an item that has been saved to your account.
pub enum Saved<T> {
    /// A saved submission.
    Submission(super::Submission<T>),
    /// A saved comment.
    Comment(super::LatestComment<T>),
}
