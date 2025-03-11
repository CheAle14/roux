//! # Subreddit Comment Responses

pub mod article;
pub mod common;
pub mod created;
pub mod latest;

use super::response::BasicListing;

pub use article::{ArticleCommentsResponse, ArticleCommentsResponseWithoutComments};
/// list of latest comment data
pub type APILatestComments = BasicListing<latest::LatestCommentData>;
/// list of article comment data
pub type APIArticleComments = BasicListing<article::ArticleCommentData>;
/// list of created comment data
pub type APICreatedComments = BasicListing<created::CreatedCommentWithLinkInfoData>;
