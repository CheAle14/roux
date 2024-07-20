use std::collections::HashMap;

use serde_json::Value;

use crate::{
    api::{
        submission::{
            SubmissionData, SubmissionDataGalleryData, SubmissionDataMediaMetadata,
            SubmissionDataPreview,
        },
        ThingId,
    },
    builders::form::FormBuilder,
    client::RedditClient,
    RouxError,
};

use super::{CreatedComment, Listing};

pub(crate) type Submissions<T> = Listing<Submission<T>>;

/// A Submission in a subreddit.
pub struct Submission<T> {
    client: T,
    data: SubmissionData,
}

impl<T> Submission<T> {
    /// Creates a new submission
    pub fn new(client: T, data: SubmissionData) -> Self {
        Self { client, data }
    }

    /// The domain of the link (if link post) or self.subreddit (if self post).
    /// Domains do not include a protocol, e.g. `i.redd.it` or `self.learnprogramming`
    pub fn domain(&self) -> &Option<String> {
        &self.data.domain
    }
    /// Contains the name of the moderator who banned this, if the logged-in user is a moderator
    /// of this subreddit and this is banned.
    pub fn banned_by(&self) -> &Option<String> {
        &self.data.banned_by
    }
    // pub fn media_embed(&self) -> &MediaEmbed { &self.data.media_embed }
    /// The subreddit that this submission was posted in (not including `/r/`)
    pub fn subreddit(&self) -> &String {
        &self.data.subreddit
    }
    /// If this is a self post, it contains the HTML of the post body. Otherwise, it is `None`.
    pub fn selftext_html(&self) -> &Option<String> {
        &self.data.selftext_html
    }
    /// The self text in **Markdown** format, if this is a self post. Unlike `selftext_html`, this
    /// is an **empty string** if this is a link post.
    pub fn selftext(&self) -> &String {
        &self.data.selftext
    }
    /// This is `Some(true)` if the logged-in user has upvoted this submission, `Some(false)` if
    /// the user has downvoted this submission or `None` if the user has not voted.
    pub fn likes(&self) -> Option<bool> {
        self.data.likes.clone()
    }
    /// If a specific sort method is suggested, this is set to the string name of it, otherwise
    /// it is `None`.
    /// # Possible values
    /// - top
    /// - new
    /// - controversial
    /// - old
    /// - qa
    /// - confidence
    pub fn suggested_sort(&self) -> &Option<String> {
        &self.data.suggested_sort
    }
    // skipped user_reports and secure_media
    /// If this post is flaired, this set to `Some(FLAIR TEXT)`. Otherwise, it is `None`.
    /// Link flairs **can** be empty strings.
    pub fn link_flair_text(&self) -> &Option<String> {
        &self.data.link_flair_text
    }
    /// The ID of the post in base-36 form, as used in Reddit's links.
    pub fn id(&self) -> &String {
        &self.data.id
    }
    // skipped from_kind
    /// The amount of times that a user has been gilded (gifted Reddit Gold).
    pub fn gilded(&self) -> u64 {
        self.data.gilded
    }
    /// This is `true` if Reddit has archived the submission (usually done after 6 months).
    /// Archived submissions cannot be voted or commented upon.
    pub fn archived(&self) -> bool {
        self.data.archived
    }
    /// This is `true` if the logged-in user has already followed this link, otherwise `false`.
    pub fn clicked(&self) -> bool {
        self.data.clicked
    }
    // skipped report_reasons
    /// The name of the author of the submission (not including the leading `/u/`)
    pub fn author(&self) -> &String {
        &self.data.author
    }
    // skipped media
    /// The overall points score of this post, as shown on the upvote counter. This is the
    /// same as upvotes - downvotes (however, this figure may be fuzzed by Reddit, and may not
    /// be exact)
    pub fn score(&self) -> f64 {
        self.data.score
    }
    /// This contains the name of the user who approved this submission. This is `None` unless
    /// you are a mod of the subreddit **and** a user has approved this post.
    pub fn approved_by(&self) -> &Option<String> {
        &self.data.approved_by
    }
    /// This is `true` if the 'nsfw' option has been selected for this submission.
    pub fn over_18(&self) -> bool {
        self.data.over_18
    }
    /// This is `true` if the 'spoiler' option has been selected for this submission.
    pub fn spoiler(&self) -> bool {
        self.data.spoiler
    }
    /// This is `true` if the logged-in user has clicked 'hide' on this post.
    pub fn hidden(&self) -> bool {
        self.data.hidden
    }
    /// Object with different sizes of the preview image.
    pub fn preview(&self) -> &Option<SubmissionDataPreview> {
        &self.data.preview
    }
    /// The number of comment replies to this submission.
    pub fn num_comments(&self) -> u64 {
        self.data.num_comments
    }
    /// The URL to the link thumbnail. This is "self" if this is a self post, or "default" if
    /// a thumbnail is not available.
    pub fn thumbnail(&self) -> &String {
        &self.data.thumbnail
    }
    /// The Reddit ID for the subreddit where this was posted.
    pub fn subreddit_id(&self) -> &ThingId {
        &self.data.subreddit_id
    }
    /// This is `true` if the score is being hidden.
    pub fn hide_score(&self) -> bool {
        self.data.hide_score
    }
    /// This is `false` if the submission is not edited and is the edit timestamp if it is edited.
    /// Access through the functions of `Submission` instead.
    pub fn edited(&self) -> &Value {
        &self.data.edited
    }
    /// The CSS class set for the link's flair (if available), otherwise `None`.
    pub fn link_flair_css_class(&self) -> &Option<String> {
        &self.data.link_flair_css_class
    }
    /// The CSS class set for the author's flair (if available). If there is no flair, this is
    /// `None`.
    pub fn author_flair_css_class(&self) -> &Option<String> {
        &self.data.author_flair_css_class
    }
    /// The number of downvotes (fuzzed; see `score` for further explanation)
    pub fn downs(&self) -> f64 {
        self.data.downs
    }
    /// The number of upvotes (fuzzed; see `score` for further explanation)
    pub fn ups(&self) -> f64 {
        self.data.ups
    }
    /// The ratio of upvotes to total votes. Equal to upvotes/(upvotes+downvotes) (fuzzed; see `score` for further explanation)
    pub fn upvote_ratio(&self) -> f64 {
        self.data.upvote_ratio
    }
    // TODO: skipped secure_media_embed
    /// True if the logged-in user has saved this submission.
    pub fn saved(&self) -> bool {
        self.data.saved
    }
    /// The reason for the post removal, if you are a moderator **and** this post has been
    /// removed.
    pub fn removal_reason(&self) -> &Option<String> {
        &self.data.removal_reason
    }
    // TODO: skipped post_hint
    /// This is `true` if this submission is stickied (an 'annoucement' thread)
    pub fn stickied(&self) -> bool {
        self.data.stickied
    }
    // TODO: skipped from
    /// This is `true` if this is a self post.
    pub fn is_self(&self) -> bool {
        self.data.is_self
    }

    /// This is `true` if this is a gallery post.
    pub fn is_gallery(&self) -> bool {
        self.data.is_gallery
    }
    /// This is `true` if this is a video, the `url` would then be to a video.
    pub fn is_video(&self) -> bool {
        self.data.is_video
    }
    // TODO: skipped from_id
    /// The permanent, long link for this submission.
    pub fn permalink(&self) -> &String {
        &self.data.permalink
    }
    /// This is `true` if the submission has been locked by a moderator, and no replies can be
    /// made.
    pub fn locked(&self) -> bool {
        self.data.locked
    }
    /// The full 'Thing ID', consisting of a 'kind' and a base-36 identifier. The valid kinds are:
    /// - t1_ - Comment
    /// - t2_ - Account
    /// - t3_ - Link
    /// - t4_ - Message
    /// - t5_ - Subreddit
    /// - t6_ - Award
    /// - t8_ - PromoCampaign
    pub fn name(&self) -> &ThingId {
        &self.data.name
    }
    /// A timestamp of the time when the post was created, in the logged-in user's **local**
    /// time.
    pub fn created(&self) -> f64 {
        self.data.created
    }
    /// The linked URL, if this is a link post.
    pub fn url(&self) -> &Option<String> {
        &self.data.url
    }
    /// The text of the author's flair, if present. Can be an empty string if the flair is present
    /// but contains no text.
    pub fn author_flair_text(&self) -> &Option<String> {
        &self.data.author_flair_text
    }
    /// This is `true` if the post is from a quarantined subreddit.
    pub fn quarantine(&self) -> bool {
        self.data.quarantine
    }
    /// The title of the post.
    pub fn title(&self) -> &String {
        &self.data.title
    }
    /// A timestamp of the time when the post was created, in **UTC**.
    pub fn created_utc(&self) -> f64 {
        self.data.created_utc
    }
    /// Distinguished
    pub fn distinguished(&self) -> &Option<String> {
        &self.data.distinguished
    }
    /// This is `true` if the user has visited this link.
    pub fn visited(&self) -> bool {
        self.data.visited
    }
    /// The number of reports, if the user is a moderator of this subreddit.
    pub fn num_reports(&self) -> &Option<u64> {
        &self.data.num_reports
    }
    /// The gallery data for this submission, if it is a gallery post.
    pub fn gallery_data(&self) -> &Option<SubmissionDataGalleryData> {
        &self.data.gallery_data
    }
    /// The media metadata, used by the gallery if it is present.
    pub fn media_metadata(&self) -> &Option<HashMap<String, SubmissionDataMediaMetadata>> {
        &self.data.media_metadata
    }
}

impl Submission<crate::client::AuthedClient> {
    /// Reports this submission with a custom reason
    #[maybe_async::maybe_async]
    pub async fn report(&self, reason: &str) -> Result<(), RouxError> {
        let form = FormBuilder::new()
            .with("id", self.name().full())
            .with("reason", reason);

        self.client.post("api/report", &form).await?;
        Ok(())
    }

    /// Adds a comment to this submission
    #[maybe_async::maybe_async]
    pub async fn comment(
        &self,
        text: &str,
    ) -> Result<CreatedComment<crate::client::AuthedClient>, RouxError> {
        self.client.comment(text, &self.data.name).await
    }

    /// Sets the [`Submission::selftext`]
    #[maybe_async::maybe_async]
    pub async fn edit(&mut self, text: &str) -> Result<(), RouxError> {
        self.client.edit(text, self.name()).await?;
        self.data.selftext = text.to_owned();
        Ok(())
    }
}
