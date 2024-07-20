use crate::{builders::form::FormBuilder, util::RouxError};

use super::{endpoint::EndpointBuilder, req, traits::RedditClient};
use reqwest::header;

/// An unauthenticated client that uses a generic user agent to interact with Reddit's API.
#[derive(Clone)]
pub struct UnauthedClient {
    inner: req::Client,
}

impl UnauthedClient {
    /// Create a new unauthenticated client.
    pub fn new() -> Result<Self, RouxError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("roux/rust"),
        );

        let inner = req::ClientBuilder::new().default_headers(headers).build()?;

        Ok(Self { inner })
    }
}

impl RedditClient for UnauthedClient {
    #[maybe_async::maybe_async]
    async fn get(
        &self,
        endpoint: impl Into<super::endpoint::EndpointBuilder>,
    ) -> Result<super::req::Response, RouxError> {
        let endpoint: EndpointBuilder = endpoint.into();
        let endpoint = endpoint.build("https://www.reddit.com");
        println!("GET {endpoint}");
        let response = self.inner.get(endpoint).send().await?;
        if response.error_for_status_ref().is_err() {
            let status = response.status();
            let body = response.text().await?;
            panic!("{:?}: {body}", status)
        } else {
            Ok(response)
        }
    }

    #[maybe_async::maybe_async]
    async fn post(
        &self,
        endpoint: impl Into<super::endpoint::EndpointBuilder>,
        form: &FormBuilder<'_>,
    ) -> Result<super::req::Response, RouxError> {
        let endpoint: EndpointBuilder = endpoint.into();
        let endpoint = endpoint.build("https://www.reddit.com");
        let resp = self.inner.post(endpoint).form(form).send().await?;
        Ok(resp)
    }
}
