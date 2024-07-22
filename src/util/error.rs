use std::error;
use std::fmt;
use std::time::Duration;

use serde_json;

use crate::client;

/// Error type that occurs when an API request fails for some reason.
pub enum RouxErrorKind {
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

/// An error type with a backtrace, if that feature is enabled.
pub struct RouxError {
    /// The kind of error that occured.
    pub kind: RouxErrorKind,
    /// A backtrace of where this error occured
    pub backtrace: Box<std::backtrace::Backtrace>,
}

impl RouxError {
    pub(crate) fn new(kind: RouxErrorKind) -> Self {
        Self {
            kind,
            backtrace: Box::new(std::backtrace::Backtrace::capture()),
        }
    }

    pub(crate) fn credentials_not_set() -> Self {
        Self::new(RouxErrorKind::CredentialsNotSet)
    }

    pub(crate) fn auth(s: String) -> Self {
        Self::new(RouxErrorKind::Auth(s))
    }

    pub(crate) fn status(response: crate::client::req::Response) -> Self {
        Self::new(RouxErrorKind::Status(response))
    }

    pub(crate) fn full_network(
        response: crate::client::req::Response,
        error: reqwest::Error,
    ) -> Self {
        Self::new(RouxErrorKind::FullNetwork(response, error))
    }

    pub(crate) fn network(error: reqwest::Error) -> Self {
        Self::new(RouxErrorKind::Network(error))
    }

    pub(crate) fn reddit_error(body: String) -> Self {
        Self::new(RouxErrorKind::RedditError { body })
    }

    pub(crate) fn parse(error: serde_json::Error) -> Self {
        Self::new(RouxErrorKind::Parse(error))
    }
}

impl From<RouxErrorKind> for RouxError {
    fn from(value: RouxErrorKind) -> Self {
        Self::new(value)
    }
}

impl From<reqwest::Error> for RouxError {
    fn from(e: reqwest::Error) -> Self {
        Self::network(e)
    }
}

impl From<serde_json::Error> for RouxError {
    fn from(e: serde_json::Error) -> Self {
        Self::parse(e)
    }
}

impl fmt::Display for RouxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            RouxErrorKind::Status(err) => write!(f, "Status error: {}", err.status()),
            RouxErrorKind::Network(err) => err.fmt(f),
            RouxErrorKind::Parse(err) => err.fmt(f),
            RouxErrorKind::Auth(err) => write!(f, "Auth error: {}", err),
            RouxErrorKind::CredentialsNotSet => write!(
                f,
                "Must set username and password before calling create_client"
            ),
            RouxErrorKind::OAuthClientRequired => {
                write!(f, "Endpoint requires authentication with OAuth")
            }
            RouxErrorKind::FullNetwork(_, err) => err.fmt(f),
            RouxErrorKind::Ratelimited { retry_after } => {
                write!(f, "Ratelimited until {retry_after:?}")
            }
            RouxErrorKind::RedditError { body } => write!(f, "API error: {body}"),
        }?;

        write!(f, "\r\nBacktrace:\r\n{:}", self.backtrace)?;

        Ok(())
    }
}
impl std::fmt::Debug for RouxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl error::Error for RouxError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            RouxErrorKind::Status(_) => None,
            RouxErrorKind::Auth(_) => None,
            RouxErrorKind::Network(err) => Some(err),
            RouxErrorKind::Parse(err) => Some(err),
            RouxErrorKind::CredentialsNotSet => None,
            RouxErrorKind::OAuthClientRequired => None,
            RouxErrorKind::FullNetwork(_, err) => Some(err),
            RouxErrorKind::Ratelimited { .. } => None,
            RouxErrorKind::RedditError { .. } => None,
        }
    }

    // https://github.com/rust-lang/rust/issues/99301
    // fn provide<'a>(&'a self, request: &mut error::Request<'a>) {
    //     request
    //         .provide_ref(&self.backtrace)
    //         .provide_value(|| self.backtrace);
    // }
}
