use crate::{
    api::{
        comment::{
            article::{ArticleCommentData, ArticleCommentOrMoreComments, MoreCommentData},
            created::{CreatedCommentData, CreatedCommentWithLinkInfoData},
            latest::LatestCommentData,
            replies::ArticleReplies,
        },
        Distinguished, ThingFullname,
    },
    builders::form::FormBuilder,
    client::{AuthedClient, RedditClient},
    models::Distinguish,
    util::RouxError,
};
use serde_json::Value;

use super::Listing;

pub(crate) type LatestComments<T> = Listing<LatestComment<T>>;
pub(crate) type ArticleComments<T> = Listing<ArticleCommentOrMore<T>>;

macro_rules! impl_comment {
    ($name:ident, $data_name:ident, $docs:literal) => {
        #[doc = $docs]
        pub struct $name<T> {
            client: T,
            data: $data_name,
        }

        impl<T> $name<T> {
            /// ??
            pub fn all_awardings(&self) -> &Vec<Value> {
                &self.data.common.all_awardings
            }

            /// Whether the comment has been approved.
            pub fn approved(&self) -> &Option<bool> {
                &self.data.common.approved
            }

            /// When the comment was approved.
            pub fn approved_at_utc(&self) -> &Option<f64> {
                &self.data.common.approved_at_utc
            }

            /// The username of the moderator who approved the comment.
            pub fn approved_by(&self) -> &Option<String> {
                &self.data.common.approved_by
            }

            /// Whether the post this comment is under has been archived.
            pub fn archived(&self) -> &bool {
                &self.data.common.archived
            }

            /// ??
            pub fn associated_award(&self) -> &Option<Value> {
                &self.data.common.associated_award
            }

            /// The username of the author of this comment.
            pub fn author(&self) -> &str {
                &self.data.common.author
            }

            /// ??
            pub fn author_flair_background_color(&self) -> &Option<Value> {
                &self.data.common.author_flair_background_color
            }

            /// ??
            pub fn author_flair_css_class(&self) -> Option<&str> {
                self.data.common.author_flair_css_class.as_ref().map(|v| v.as_str())
            }

            /// ??
            pub fn author_flair_richtext(&self) -> &Option<Value> {
                &self.data.common.author_flair_richtext
            }

            /// ??
            pub fn author_flair_text(&self) -> &Option<String> {
                &self.data.common.author_flair_text
            }

            /// ??
            pub fn author_flair_text_color(&self) -> &Option<Value> {
                &self.data.common.author_flair_text_color
            }

            /// ??
            pub fn author_flair_type(&self) -> &Option<String> {
                &self.data.common.author_flair_type
            }

            /// The author's flair's template id.
            pub fn author_flair_template_id(&self) -> Option<&str> {
                self.data
                    .common
                    .author_flair_template_id
                    .as_ref()
                    .map(|v| v.as_str())
            }

            /// The fullname of the author of this comment
            pub fn author_fullname(&self) -> Option<&ThingFullname> {
                self.data.common.author_fullname.as_ref()
            }

            /// Whether you have blocked the author of this comment
            pub fn author_is_blocked(&self) -> bool {
                self.data.common.author_is_blocked
            }

            /// ??
            pub fn author_patreon_flair(&self) -> Option<bool> {
                self.data.common.author_patreon_flair.clone()
            }

            /// Whether the author has premium
            pub fn author_premium(&self) -> Option<bool> {
                self.data.common.author_premium.clone()
            }

            /// ??
            pub fn awarders(&self) -> &Vec<Value> {
                &self.data.common.awarders
            }

            /// When the author was banned?
            pub fn banned_at_utc(&self) -> &Option<f64> {
                &self.data.common.banned_at_utc
            }

            /// ??
            pub fn banned_by(&self) -> &Option<Value> {
                &self.data.common.banned_by
            }

            /// The text content of this comment
            pub fn body(&self) -> &str {
                &self.data.common.body
            }

            /// The HTML encoded content of this comment.
            pub fn body_html(&self) -> &str {
                &self.data.common.body_html
            }

            /// Whether you can gild this comment.
            pub fn can_gild(&self) -> bool {
                self.data.common.can_gild
            }

            /// Whether you can moderator this comment (or the post its on?)
            pub fn can_mod_post(&self) -> bool {
                self.data.common.can_mod_post
            }

            /// Whether this comment has been collapsed.
            pub fn collapsed(&self) -> &bool {
                &self.data.common.collapsed
            }

            /// ??
            pub fn collapsed_because_crowd_control(&self) -> &Option<Value> {
                &self.data.common.collapsed_because_crowd_control
            }

            /// ??
            pub fn collapsed_reason(&self) -> &Option<Value> {
                &self.data.common.collapsed_reason
            }

            /// ??
            pub fn collapsed_reason_code(&self) -> &Option<Value> {
                &self.data.common.collapsed_reason_code
            }

            /// ??
            pub fn comment_type(&self) -> &Option<Value> {
                &self.data.common.comment_type
            }

            /// ??
            pub fn controversiality(&self) -> i32 {
                self.data.common.controversiality
            }

            /// When the comment was created
            pub fn created(&self) -> f64 {
                self.data.common.created
            }
            /// When the comment was created in UTC.
            pub fn created_utc(&self) -> f64 {
                self.data.common.created_utc
            }

            /// The distinguishment of this comment
            pub fn distinguished(&self) -> Distinguished {
                self.data.common.distinguished
            }

            /// The downvotes on this comment. Note that these values are fuzzed.
            pub fn downs(&self) -> i32 {
                self.data.common.downs
            }

            /// Whether this comment has been edited.
            /// Note that edits within the first few minutes of the comment being created will not cause this to be true.
            pub fn edited(&self) -> crate::api::comment::common::Edited {
                self.data.common.edited
            }

            /// ??
            pub fn gilded(&self) -> i32 {
                self.data.common.gilded
            }

            /// ??
            pub fn gildings(&self) -> &Value {
                &self.data.common.gildings
            }

            /// The ID of this comment
            pub fn id(&self) -> &str {
                &self.data.common.id
            }

            /// Whether this comment has reports ignored.
            pub fn ignore_reports(&self) -> &Option<bool> {
                &self.data.common.ignore_reports
            }

            /// Whether you are the submitter of the post this comment is under.
            pub fn is_submitter(&self) -> bool {
                self.data.common.is_submitter
            }

            /// ??
            pub fn likes(&self) -> &Option<Value> {
                &self.data.common.likes
            }

            /// The full name of the post this comment is under.
            pub fn link_id(&self) -> &ThingFullname {
                &self.data.common.link_id
            }

            /// Whether this comment is locked.
            pub fn locked(&self) -> bool {
                self.data.common.locked
            }

            /// ??
            pub fn mod_note(&self) -> &Option<Value> {
                &self.data.common.mod_note
            }

            /// ??
            pub fn mod_reason_by(&self) -> &Option<Value> {
                &self.data.common.mod_reason_by
            }

            /// ??
            pub fn mod_reason_title(&self) -> &Option<Value> {
                &self.data.common.mod_reason_title
            }

            /// The moderator reports. This seems to be a tuple of (reason, moderator name).
            pub fn mod_reports(&self) -> &Vec<[String; 2]> {
                &self.data.common.mod_reports
            }

            /// The full name of this comment
            pub fn name(&self) -> &ThingFullname {
                &self.data.common.name
            }

            /// ??
            pub fn no_follow(&self) -> bool {
                self.data.common.no_follow
            }

            /// How many times this comment has been reported
            pub fn num_reports(&self) -> &Option<i32> {
                &self.data.common.num_reports
            }

            /// The full name of the parent of this comment.
            ///
            /// If this is top-level comment, this will be the Submission's full name (and the [`kind`](crate::models::ThingFullname::kind) will be `t3`).
            /// If this is a reply to another comment, this will instead be the full name of the parent comment (kind `t1`).
            pub fn parent_id(&self) -> &ThingFullname {
                &self.data.common.parent_id
            }

            /// The permalink to this comment
            pub fn permalink(&self) -> &str {
                &self.data.common.permalink
            }

            /// ??
            pub fn removal_reason(&self) -> &Option<Value> {
                &self.data.common.removal_reason
            }

            /// Whether this comment has been removed
            pub fn removed(&self) -> &Option<bool> {
                &self.data.common.removed
            }

            /// ??
            pub fn report_reasons(&self) -> &Option<Vec<Value>> {
                &self.data.common.report_reasons
            }

            /// Whether you have saved this comment.
            pub fn saved(&self) -> bool {
                self.data.common.saved
            }

            /// The sum of `ups + downs`. Note that this value is fuzzed.
            pub fn score(&self) -> i32 {
                self.data.common.score
            }

            /// Whether the score is hidden from view.
            pub fn score_hidden(&self) -> bool {
                self.data.common.score_hidden
            }

            /// Whether replies to this comment should be sent as messages to your inbox.
            pub fn send_replies(&self) -> bool {
                self.data.common.send_replies
            }

            /// Whether this comment has been removed as spam.
            pub fn spam(&self) -> &Option<bool> {
                &self.data.common.spam
            }

            /// Whether the comment has been stickied. Can only apply to top-level comments.
            pub fn stickied(&self) -> bool {
                self.data.common.stickied
            }

            /// The name of the subreddit that this comment was made in.
            pub fn subreddit(&self) -> &str {
                &self.data.common.subreddit
            }

            /// The full name of the subreddit that this comment was made in
            pub fn subreddit_id(&self) -> &ThingFullname {
                &self.data.common.subreddit_id
            }

            /// The name of the subreddit prefixed with `/r/`
            pub fn subreddit_name_prefixed(&self) -> &str {
                &self.data.common.subreddit_name_prefixed
            }

            /// The subreddit type
            /// TODO: make this an enum
            pub fn subreddit_type(&self) -> &str {
                &self.data.common.subreddit_type
            }

            /// ??
            pub fn top_awarded_type(&self) -> &Option<Value> {
                &self.data.common.top_awarded_type
            }

            /// ??
            pub fn total_awards_received(&self) -> i32 {
                self.data.common.total_awards_received
            }

            /// ??
            pub fn treatment_tags(&self) -> &Vec<Value> {
                &self.data.common.treatment_tags
            }

            /// ??
            pub fn unrepliable_reason(&self) -> &Option<Value> {
                &self.data.common.unrepliable_reason
            }

            /// The upvotes on this comment. Note that this value is fuzzed.
            pub fn ups(&self) -> i32 {
                self.data.common.ups
            }

            /// ??
            pub fn user_reports(&self) -> &Vec<Value> {
                &self.data.common.user_reports
            }
        }

        impl<T> crate::models::FromClientAndData<T, $data_name> for $name<T> {
            fn new(client: T, data: $data_name) -> Self {
                Self { client, data }
            }
        }

        impl $name<AuthedClient> {
            /// Reports this comment with a custom reason
            #[maybe_async::maybe_async]
            pub async fn report(&self, reason: &str) -> Result<(), RouxError> {
                let form = FormBuilder::new()
                    .with("id", self.name().full())
                    .with("reason", reason);
                self.client.post("api/report", &form).await?;
                Ok(())
            }

            /// Adds a reply to this comment
            #[maybe_async::maybe_async]
            pub async fn reply(
                &self,
                text: &str,
            ) -> Result<CreatedComment<crate::client::AuthedClient>, RouxError> {
                self.client.comment(text, &self.data.common.name).await
            }

            /// Edits the text of this comment.
            #[maybe_async::maybe_async]
            pub async fn edit(&mut self, text: &str) -> Result<(), RouxError> {
                self.client.edit(text, &self.data.common.name).await?;
                self.data.common.body = text.to_owned();
                Ok(())
            }

            /// Deletes our own comment. This will fail if we did not create the comment.
            #[maybe_async::maybe_async]
            pub async fn delete(&self) -> Result<(), RouxError> {
                let form = FormBuilder::new().with("id", self.name().full());
                let _ = self.client.post("api/del", &form).await?;
                Ok(())
            }

            /// Removes this comment, requires moderator permission in the subreddit.
            #[maybe_async::maybe_async]
            pub async fn remove(&self, spam: bool) -> Result<(), RouxError> {
                self.client.remove(self.name(), spam).await
            }

            /// Locks this comment.
            #[maybe_async::maybe_async]
            pub async fn lock(&self) -> Result<(), RouxError> {
                self.client.lock(self.name()).await
            }

            /// Unlocks this comment.
            #[maybe_async::maybe_async]
            pub async fn unlock(&self) -> Result<(), RouxError> {
                self.client.unlock(self.name()).await
            }

            /// Distinguishes this comment.
            #[maybe_async::maybe_async]
            pub async fn distinguish(
                &self,
                kind: Distinguish,
                sticky: bool,
            ) -> Result<(), RouxError> {
                self.client.distinguish(self.name(), kind, sticky).await
            }
        }
    };
}

macro_rules! impl_comment_with_link_info {
    ($name:ident) => {
        impl<T> $name<T> {
            /// Author of the link post this comment is under.
            pub fn link_author(&self) -> &str {
                &self.data.link_author
            }
            /// Permalink to the post this comment is under.
            pub fn link_permalink(&self) -> &str {
                &self.data.link_permalink
            }
            /// Title of the post this comment is under.
            pub fn link_title(&self) -> &str {
                &self.data.link_title
            }
            /// Link to the content of the post this comment is under.
            pub fn link_url(&self) -> &str {
                &self.data.link_url
            }
        }
    };
}

impl_comment!(LatestComment, LatestCommentData, "Represents a comment found through the [`Subreddit::latest_comments`](crate::client::Subreddit::latest_comments) or similar overview functions. For a comment with full information, see [`ArticleComment`](crate::models::comment::ArticleComment)");
impl_comment!(ArticleComment, ArticleCommentData, "Represents a comment with full information found through either creating it or [`crate::models::Submission::article_comments`]. For a comment with less information, see [`LatestComment`](crate::models::comment::LatestComment)");
impl_comment!(CreatedComment, CreatedCommentData, "Represents a comment that you have created, either under a submission or in reply to another comment.");
impl_comment!(CreatedCommentWithLinkInfo, CreatedCommentWithLinkInfoData, "Represents a comment that you have created, with information about the submission it was made under.");

impl_comment_with_link_info!(LatestComment);
impl_comment_with_link_info!(CreatedCommentWithLinkInfo);

impl<T> ArticleComment<T> {
    /// Gets the underlying raw data.
    pub fn raw_data(&self) -> &ArticleCommentData {
        &self.data
    }

    /// How deep this comment is beneath the post.
    pub fn depth(&self) -> i32 {
        self.data.depth
    }

    /// The replies to this comment.
    pub fn replies(&self) -> &ArticleReplies {
        &self.data.replies
    }
}

/// Either a comment or a marker that more need to be loaded.
pub enum ArticleCommentOrMore<T> {
    /// The comment
    Comment(ArticleComment<T>),
    /// The marker
    More(MoreCommentData),
}

impl<Client> super::FromClientAndData<Client, ArticleCommentOrMoreComments>
    for ArticleCommentOrMore<Client>
{
    fn new(client: Client, data: ArticleCommentOrMoreComments) -> Self {
        match data {
            ArticleCommentOrMoreComments::Comment(data) => {
                Self::Comment(ArticleComment::new(client, data))
            }
            ArticleCommentOrMoreComments::More(data) => Self::More(data),
        }
    }
}
