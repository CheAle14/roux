use crate::{
    api::{inbox::InboxData, ThingFullname},
    client::AuthedClient,
    util::RouxError,
};

use super::{FromClientAndData, Listing};

/// A message in the inbox.
pub struct Message<T> {
    client: T,
    data: InboxData,
}

impl<T> Message<T> {
    /// ID
    pub fn id(&self) -> &String {
        &self.data.id
    }
    /// Subject
    pub fn subject(&self) -> &String {
        &self.data.subject
    }
    /// Was comment
    pub fn was_comment(&self) -> &bool {
        &self.data.was_comment
    }
    /// Author
    pub fn author(&self) -> &Option<String> {
        &self.data.author
    }
    /// Parent ID
    pub fn parent_id(&self) -> &Option<ThingFullname> {
        &self.data.parent_id
    }
    /// Sub name
    pub fn subreddit_name_prefixed(&self) -> &Option<String> {
        &self.data.subreddit_name_prefixed
    }
    /// New
    pub fn is_new(&self) -> &bool {
        &self.data.new
    }
    /// ???
    pub fn r#type(&self) -> &String {
        &self.data.r#type
    }
    /// Body
    pub fn body(&self) -> &String {
        &self.data.body
    }
    /// Dest
    pub fn dest(&self) -> &String {
        &self.data.dest
    }
    /// Body HTML
    pub fn body_html(&self) -> &String {
        &self.data.body_html
    }
    /// Name
    pub fn name(&self) -> &ThingFullname {
        &self.data.name
    }
    /// Created
    pub fn created(&self) -> &f64 {
        &self.data.created
    }
    /// Created (UTC)
    pub fn created_utc(&self) -> &f64 {
        &self.data.created_utc
    }
    /// Context
    pub fn context(&self) -> &String {
        &self.data.context
    }
    /// The first message in this reply chain.
    pub fn first_message_name(&self) -> &Option<ThingFullname> {
        &self.data.first_message_name
    }
}

impl Message<AuthedClient> {
    /// Mark this message as read.
    #[maybe_async::maybe_async]
    pub async fn mark_read(&self) -> Result<(), RouxError> {
        self.client.mark_read(self.name()).await?;
        Ok(())
    }

    /// Mark this message as unread.
    #[maybe_async::maybe_async]
    pub async fn mark_unread(&self) -> Result<(), RouxError> {
        self.client.mark_unread(self.name()).await?;
        Ok(())
    }

    /// Reply to this message.
    #[maybe_async::maybe_async]
    pub async fn reply(&self, text: &str) -> Result<Message<AuthedClient>, RouxError> {
        self.client.reply(text, self.name()).await
    }
}

impl<T> FromClientAndData<T, InboxData> for Message<T> {
    fn new(client: T, data: InboxData) -> Self {
        Self { client, data }
    }
}

pub type Inbox<T> = Listing<Message<T>>;
