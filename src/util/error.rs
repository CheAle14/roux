use std::error;
use std::fmt;
use std::time::Duration;

use serde_json;

use crate::client;

/// Error type that occurs when an API request fails for some reason.
#[derive(Debug)]
pub enum RouxError {
    /// Occurs when the API has returned a non-success error code.
    Status(client::req::Response),
    /// Occurs if the HTTP response from Reddit was corrupt and
    /// reqwest could not parse it.
    Network(reqwest::Error),
    /// A network error, with the response.
    FullNetwork(client::req::Response, reqwest::Error),
    /// Occurs if the request triggered a ratelimit
    Ratelimited {
        /// The duration to retry the request at, according to the response `Retry-After` header, or none
        /// if that header did not exist on the response.
        retry_after: Option<Duration>,
    },
    /// An error returned from Reddit's API.
    /// TODO actually figure out its structure when we get one..
    RedditError {
        /// The (presumably JSON) reddit API error
        body: String,
    },
    /// Occurs if serde could not Deserialize the response.
    Parse(serde_json::Error),
    /// Occurs if there is a grant error.
    Auth(String),
    /// Occurs if [`Reddit::create_client`] is called before [`Reddit::username`] and [`Reddit::password`].
    CredentialsNotSet,
    /// Occurs if endpoint requires OAuth
    OAuthClientRequired,
}

impl From<reqwest::Error> for RouxError {
    fn from(e: reqwest::Error) -> Self {
        Self::Network(e)
    }
}

impl From<serde_json::Error> for RouxError {
    fn from(e: serde_json::Error) -> Self {
        RouxError::Parse(e)
    }
}

impl fmt::Display for RouxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            RouxError::Status(err) => write!(f, "Status error: {}", err.status()),
            RouxError::Network(err) => err.fmt(f),
            RouxError::Parse(err) => err.fmt(f),
            RouxError::Auth(err) => write!(f, "Auth error: {}", err),
            RouxError::CredentialsNotSet => write!(
                f,
                "Must set username and password before calling create_client"
            ),
            RouxError::OAuthClientRequired => {
                write!(f, "Endpoint requires authentication with OAuth")
            }
            RouxError::FullNetwork(_, err) => err.fmt(f),
            RouxError::Ratelimited { retry_after } => {
                write!(f, "Ratelimited until {retry_after:?}")
            }
            RouxError::RedditError { body } => write!(f, "API error: {body}"),
        }
    }
}

impl error::Error for RouxError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            RouxError::Status(_) => None,
            RouxError::Auth(_) => None,
            RouxError::Network(err) => Some(err),
            RouxError::Parse(err) => Some(err),
            RouxError::CredentialsNotSet => None,
            RouxError::OAuthClientRequired => None,
            RouxError::FullNetwork(_, err) => Some(err),
            RouxError::Ratelimited { .. } => None,
            RouxError::RedditError { .. } => None,
        }
    }
}
