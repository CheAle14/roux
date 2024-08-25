// pub ([\w_]+): ([\w<>, ]+),
// pub fn $1(&self) -> &$2 { &self.data.$1 }

pub(crate) mod listing;
pub use listing::Listing;

pub(crate) mod submission;
pub use submission::Submission;

pub(crate) mod saved;
pub use saved::Saved;

pub(crate) mod comment;
pub use comment::{ArticleComment, CreatedComment, CreatedCommentWithLinkInfo, LatestComment};

pub(crate) mod inbox;
pub use inbox::Message;

mod distinguish;
pub use distinguish::Distinguish;

pub(crate) trait FromClientAndData<Client, Data> {
    fn new(client: Client, data: Data) -> Self;
}
