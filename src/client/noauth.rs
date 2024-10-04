use std::future::Future;

use crate::{
    builders::form::FormBuilder,
    util::{maybe_async_handler, RouxError},
};

use super::{endpoint::EndpointBuilder, req::*, traits::RedditClient};
use reqwest::{header, Method};
use serde::Serialize;

/// An unauthenticated client that uses a generic user agent to interact with Reddit's API.
#[derive(Clone)]
pub struct UnauthedClient {
    inner: Client,
}

impl UnauthedClient {
    /// Create a new unauthenticated client.
    pub fn new() -> Result<Self, RouxError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("roux/rust"),
        );

        let inner = ClientBuilder::new().default_headers(headers).build()?;

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
    async fn post<T: Serialize>(
        &self,
        endpoint: impl Into<super::endpoint::EndpointBuilder>,
        form: &T,
    ) -> Result<super::req::Response, RouxError> {
        let endpoint: EndpointBuilder = endpoint.into();
        let endpoint = endpoint.build("https://www.reddit.com");
        let resp = self.inner.post(endpoint).form(form).send().await?;
        Ok(resp)
    }

    maybe_async_handler!(fn execute_with_retries(&self, builder, handler) RouxError {
        let req = builder().build()?;
        let response = self.inner.execute(req).await?;
        Ok(handler(response).await?)
    });

    fn make_req(&self, method: Method, endpoint: &EndpointBuilder) -> RequestBuilder {
        let endpoint = endpoint.build("https://www.reddit.com");
        self.inner.request(method, &endpoint)
    }
}
