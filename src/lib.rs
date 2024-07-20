#![deny(missing_docs)]

//! # roux.rs
//! This crate provides simple access to the Reddit API.
//!
//! ## Using OAuth
//! To create an OAuth client with the reddit API, use the `Reddit` class.
//! ```no_run
//! use roux::Reddit;
//! # #[cfg(not(feature = "blocking"))]
//! # use tokio;
//!
//! # #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! # #[maybe_async::maybe_async]
//! # async fn main() {
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .username("USERNAME")
//!     .password("PASSWORD")
//!     .login()
//!     .await;
//! let me = client.unwrap();
//! # }
//! ```
//!
//! It is important that you pick a good user agent. The ideal format is
//! `platform:program:version (by /u/yourname)`, e.g. `macos:roux:v0.3.0 (by /u/beanpup_py)`.
//!
//! This will authticate you as the user given in the username function.
//!
//!
//! ## Usage
//! Using the OAuth client, you can:
//!
//! ### Submit A Text Post
//! ```no_run
//! use roux::Reddit;
//! # #[cfg(not(feature = "blocking"))]
//! # use tokio;
//!
//! # #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! # #[maybe_async::maybe_async]
//! # async fn main() {
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .username("USERNAME")
//!     .password("PASSWORD")
//!     .login()
//!     .await;
//! let me = client.unwrap();
//!
//! me.submit_text("TEXT_TITLE", "TEXT_BODY", "SUBREDDIT");
//! # }
//! ```
//!
//! ### Submit A Link Post
//! ```no_run
//! use roux::Reddit;
//! # #[cfg(not(feature = "blocking"))]
//! # use tokio;
//!
//! # #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! # #[maybe_async::maybe_async]
//! # async fn main() {
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .username("USERNAME")
//!     .password("PASSWORD")
//!     .login()
//!     .await;
//! let me = client.unwrap();
//!
//! # me.submit_link("LINK_TITLE", "LINK", "SUBREDDIT");
//! # }
//! ```

mod config;
pub use config::Config;

/// The clients and some models that store them.
pub mod client;

/// Models that can be interacted with (e.g. reported, edited) using a stored client
pub mod models;

/// The data structures as returned by Reddit's API
pub mod api;

/// Builders to help construct requests to the API
pub mod builders;

/// Utils for requests.
pub mod util;
use util::{url, RouxError};
