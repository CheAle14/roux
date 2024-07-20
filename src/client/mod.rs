pub(crate) mod req {
    #[cfg(feature = "blocking")]
    pub use reqwest::blocking::{Client, ClientBuilder, Request, RequestBuilder, Response};
    #[cfg(feature = "blocking")]
    pub use std::sync::Mutex;

    #[cfg(not(feature = "blocking"))]
    pub use reqwest::{Client, ClientBuilder, Request, RequestBuilder, Response};
    #[cfg(not(feature = "blocking"))]
    pub use tokio::sync::Mutex;
}

mod auth;
pub(crate) mod endpoint;
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
