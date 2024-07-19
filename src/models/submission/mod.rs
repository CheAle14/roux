//! # Subreddit Submission Responses
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{models::response::BasicListing, ThingId};

/// SubmissionsData
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionData {
    /// The domain of the link (if link post) or self.subreddit (if self post).
    /// Domains do not include a protocol, e.g. `i.redd.it` or `self.learnprogramming`
    pub domain: Option<String>,
    /// Contains the name of the moderator who banned this, if the logged-in user is a moderator
    /// of this subreddit and this is banned.
    pub banned_by: Option<String>,
    // pub media_embed: MediaEmbed,
    /// The subreddit that this submission was posted in (not including `/r/`)
    pub subreddit: String,
    /// If this is a self post, it contains the HTML of the post body. Otherwise, it is `None`.
    pub selftext_html: Option<String>,
    /// The self text in **Markdown** format, if this is a self post. Unlike `selftext_html`, this
    /// is an **empty string** if this is a link post.
    pub selftext: String,
    /// This is `Some(true)` if the logged-in user has upvoted this submission, `Some(false)` if
    /// the user has downvoted this submission or `None` if the user has not voted.
    pub likes: Option<bool>,
    /// If a specific sort method is suggested, this is set to the string name of it, otherwise
    /// it is `None`.
    /// # Possible values
    /// - top
    /// - new
    /// - controversial
    /// - old
    /// - qa
    /// - confidence
    pub suggested_sort: Option<String>,
    // skipped user_reports and secure_media
    /// If this post is flaired, this set to `Some(FLAIR TEXT)`. Otherwise, it is `None`.
    /// Link flairs **can** be empty strings.
    pub link_flair_text: Option<String>,
    /// The ID of the post in base-36 form, as used in Reddit's links.
    pub id: String,
    // skipped from_kind
    /// The amount of times that a user has been gilded (gifted Reddit Gold).
    pub gilded: u64,
    /// This is `true` if Reddit has archived the submission (usually done after 6 months).
    /// Archived submissions cannot be voted or commented upon.
    pub archived: bool,
    /// This is `true` if the logged-in user has already followed this link, otherwise `false`.
    pub clicked: bool,
    // skipped report_reasons
    /// The name of the author of the submission (not including the leading `/u/`)
    pub author: String,
    // skipped media
    /// The overall points score of this post, as shown on the upvote counter. This is the
    /// same as upvotes - downvotes (however, this figure may be fuzzed by Reddit, and may not
    /// be exact)
    pub score: f64,
    /// This contains the name of the user who approved this submission. This is `None` unless
    /// you are a mod of the subreddit **and** a user has approved this post.
    pub approved_by: Option<String>,
    /// This is `true` if the 'nsfw' option has been selected for this submission.
    pub over_18: bool,
    /// This is `true` if the 'spoiler' option has been selected for this submission.
    pub spoiler: bool,
    /// This is `true` if the logged-in user has clicked 'hide' on this post.
    pub hidden: bool,
    /// Object with different sizes of the preview image.
    pub preview: Option<SubmissionDataPreview>,
    /// The number of comment replies to this submission.
    pub num_comments: u64,
    /// The URL to the link thumbnail. This is "self" if this is a self post, or "default" if
    /// a thumbnail is not available.
    pub thumbnail: String,
    /// The Reddit ID for the subreddit where this was posted.
    pub subreddit_id: ThingId,
    /// This is `true` if the score is being hidden.
    pub hide_score: bool,
    /// This is `false` if the submission is not edited and is the edit timestamp if it is edited.
    /// Access through the functions of `Submission` instead.
    pub edited: Value,
    /// The CSS class set for the link's flair (if available), otherwise `None`.
    pub link_flair_css_class: Option<String>,
    /// The CSS class set for the author's flair (if available). If there is no flair, this is
    /// `None`.
    pub author_flair_css_class: Option<String>,
    /// The number of downvotes (fuzzed; see `score` for further explanation)
    pub downs: f64,
    /// The number of upvotes (fuzzed; see `score` for further explanation)
    pub ups: f64,
    /// The ratio of upvotes to total votes. Equal to upvotes/(upvotes+downvotes) (fuzzed; see `score` for further explanation)
    pub upvote_ratio: f64,
    // TODO: skipped secure_media_embed
    /// True if the logged-in user has saved this submission.
    pub saved: bool,
    /// The reason for the post removal, if you are a moderator **and** this post has been
    /// removed.
    pub removal_reason: Option<String>,
    // TODO: skipped post_hint
    /// This is `true` if this submission is stickied (an 'annoucement' thread)
    pub stickied: bool,
    // TODO: skipped from
    /// This is `true` if this is a self post.
    pub is_self: bool,

    /// This is `true` if this is a gallery post.
    #[serde(default)]
    pub is_gallery: bool,
    /// This is `true` if this is a video, the `url` would then be to a video.
    #[serde(default)]
    pub is_video: bool,
    // TODO: skipped from_id
    /// The permanent, long link for this submission.
    pub permalink: String,
    /// This is `true` if the submission has been locked by a moderator, and no replies can be
    /// made.
    pub locked: bool,
    /// The full 'Thing ID', consisting of a 'kind' and a base-36 identifier. The valid kinds are:
    /// - t1_ - Comment
    /// - t2_ - Account
    /// - t3_ - Link
    /// - t4_ - Message
    /// - t5_ - Subreddit
    /// - t6_ - Award
    /// - t8_ - PromoCampaign
    pub name: ThingId,
    /// A timestamp of the time when the post was created, in the logged-in user's **local**
    /// time.
    pub created: f64,
    /// The linked URL, if this is a link post.
    pub url: Option<String>,
    /// The text of the author's flair, if present. Can be an empty string if the flair is present
    /// but contains no text.
    pub author_flair_text: Option<String>,
    /// This is `true` if the post is from a quarantined subreddit.
    pub quarantine: bool,
    /// The title of the post.
    pub title: String,
    /// A timestamp of the time when the post was created, in **UTC**.
    pub created_utc: f64,
    /// Distinguished
    pub distinguished: Option<String>,
    /// This is `true` if the user has visited this link.
    pub visited: bool,
    /// The number of reports, if the user is a moderator of this subreddit.
    pub num_reports: Option<u64>,
    /// The gallery data for this submission, if it is a gallery post.
    pub gallery_data: Option<SubmissionDataGalleryData>,
    /// The media metadata, used by the gallery if it is present.
    pub media_metadata: Option<HashMap<String, SubmissionDataMediaMetadata>>,
}

/// SubmissionDataPreview
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionDataPreview {
    /// List of preview images.
    pub images: Vec<SubmissionDataPreviewImage>,
    /// This is `true` if the preview is enabled.
    pub enabled: bool,
}

/// SubmissionDataPreviewImage
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionDataPreviewImage {
    /// Object for the main preview image containing URL, width and height.
    pub source: SubmissionDataPreviewImageSource,
    /// List of objects describing all available resolutions of the preview image.
    pub resolutions: Vec<SubmissionDataPreviewImageSource>,
    // TODO: skipped variants
    /// Preview Image ID
    pub id: String,
}

/// SubmissionDataPreviewImageSource
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionDataPreviewImageSource {
    /// URL
    pub url: String,
    /// Width
    pub width: u64,
    /// Height
    pub height: u64,
}

/// Submission gallery data
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionDataGalleryData {
    /// The gallery items
    pub items: Vec<SubmissionDataGalleryItem>,
}

/// Submission gallery item
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionDataGalleryItem {
    /// Gallery caption
    pub caption: Option<String>,
    /// Id of this item
    pub id: f64,
    /// Media metadata ID, should be present in submission `media_metadata`
    pub media_id: String,
}

/// Submission media metadata
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "e")]
pub enum SubmissionDataMediaMetadata {
    /// An image
    Image {
        /// The ID for this media metadata.
        id: String,
        /// The media type, e.g. `image/png`
        m: String,
        /// The media value
        s: SubmissionDataMediaMetadataValue,
    },
    /// A reddit video
    RedditVideo {
        /// Id to the video
        id: String,
        /// Whether the video is a gif
        #[serde(rename = "isGif")]
        is_gif: bool,
        /// ??
        status: String,
        /// Presuambly width?
        x: i32,
        /// Presumably height?
        y: i32,
        /// ??
        #[serde(rename = "dashUrl")]
        dash_url: String,
        /// ??
        #[serde(rename = "hlsUrl")]
        hls_url: String,
    },
}

/// Submission media metadata values
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionDataMediaMetadataValue {
    /// Media URL
    pub u: String,
    /// Media width
    pub x: u64,
    /// Media height
    pub y: u64,
}

/// Submissions
pub type Submissions = BasicListing<SubmissionData>;
