use std::borrow::Cow;

/// Configuration information for the OAuth or Authed clients.
#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) user_agent: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) base_url: Cow<'static, str>,
}

impl Config {
    /// Creates a new config using the provided values.
    pub fn new(user_agent: &str, client_id: &str, client_secret: &str) -> Config {
        Config {
            user_agent: user_agent.to_owned(),
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            username: None,
            password: None,
            base_url: Cow::Borrowed("https://oauth.reddit.com"),
        }
    }

    /// Sets the password.
    ///
    /// Once both password and username are set, use [`crate::client::OAuthClient::login`] to login.
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Sets the username.
    ///
    /// Once both password and username are set, use [`crate::client::OAuthClient::login`] to login.
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    /// Configures the base URL the client sends requests to.
    ///
    /// Defaults to https://oauth.reddit.com
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Cow::Owned(url.into());
        self
    }
}
