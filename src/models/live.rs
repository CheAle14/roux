//! Helper models for live thread related objects.

use crate::{api::live::LiveThreadData, client::AuthedClient, util::RouxError};

/// A live thread that can provide live-updating events.
pub struct LiveThread<T> {
    client: T,
    data: LiveThreadData,
}

impl<T> std::ops::Deref for LiveThread<T> {
    type Target = LiveThreadData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl LiveThread<AuthedClient> {
    /// Helper to get the human URL to this live thread.
    pub fn url(&self) -> String {
        format!("https://www.reddit.com/live/{}", &self.id)
    }

    /// Close this thread, meaning it will get no more updates.
    #[maybe_async::maybe_async]
    pub async fn close(&self) -> Result<(), RouxError> {
        self.client.close_live_thread(&self.id).await
    }

    /// Posts an update to this live thread.
    #[maybe_async::maybe_async]
    pub async fn update(&self, text: &str) -> Result<(), RouxError> {
        self.client.update_live_thread(&self.id, text).await
    }

    /// Invites a contributor to this live thread.
    #[maybe_async::maybe_async]
    pub async fn invite(&self, name: &str) -> Result<(), RouxError> {
        self.client
            .invite_live_thread_contributor(&self.id, name)
            .await
    }
}

impl<T> super::FromClientAndData<T, LiveThreadData> for LiveThread<T> {
    fn new(client: T, data: LiveThreadData) -> Self {
        Self { client, data }
    }
}
