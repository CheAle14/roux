#[cfg(feature = "blocking")]
pub(crate) mod req {
    pub use reqwest::blocking::{Client, ClientBuilder, Request, RequestBuilder, Response};
    pub use std::sync::Mutex;
    pub use std::thread::sleep;
}

#[cfg(not(feature = "blocking"))]
pub(crate) mod req {
    pub use reqwest::{Client, ClientBuilder, Request, RequestBuilder, Response};
    pub use tokio::sync::Mutex;
    pub use tokio::time::sleep;
}

mod auth;
pub(crate) mod endpoint;
pub(crate) mod inner;
mod noauth;
mod oauth;
mod ratelimit;
mod subreddits;
mod traits;
mod user;

pub use auth::*;
pub use noauth::*;
pub use oauth::*;
pub use subreddits::*;
pub use traits::RedditClient;
pub use user::*;
