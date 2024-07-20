//! # Subreddit Comment Responses

pub mod article;
pub mod common;
pub mod created;
pub mod latest;

use super::response::BasicListing;

pub use article::ArticleCommentsResponse;
/// list of latest comment data
pub type APILatestComments = BasicListing<latest::LatestCommentData>;
/// list of article comment data
pub type APIArticleComments = BasicListing<article::ArticleCommentData>;
