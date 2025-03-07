use serde::{Deserialize, Serialize};

use crate::api::{
    response::{BasicListing, BasicThing, ListingNotFullname},
    ThingFullname,
};

/// The different kinds of subreddit moderator actions.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModActionType {
    /// Ban user from subreddit
    BanUser,
    /// Unban a user
    UnbanUser,
    /// Remove a link as spam
    SpamLink,
    /// Remove a link (not as spam)
    RemoveLink,
    /// Approve a link
    ApproveLink,
    /// Remove a comment as spam
    SpamComment,
    /// Remove a comment (not as spam)
    RemoveComment,
    /// Approve a comment
    ApproveComment,
    /// Add a moderator to the subreddit
    AddModerator,
    /// ShowComment
    ShowComment,
    /// InviteModerator
    InviteModerator,
    /// UninviteModerator
    UninviteModerator,
    /// AcceptModeratorInvite
    AcceptModeratorInvite,
    /// RemoveModerator
    RemoveModerator,
    /// AddContributor
    AddContributor,
    /// RemoveContributor
    RemoveContributor,
    /// EditSettings
    EditSettings,
    /// EditFlair
    EditFlair,
    /// Distinguish
    Distinguish,
    /// MarkNsfw
    MarkNsfw,
    /// WikiBanned
    WikiBanned,
    /// WikiContributor
    WikiContributor,
    /// WikiUnbanned
    WikiUnbanned,
    /// WikiPageListed
    WikiPageListed,
    /// RemoveWikiContributor
    RemoveWikiContributor,
    /// WikiRevise
    WikiRevise,
    /// WikiPermLevel
    WikiPermLevel,
    /// IgnoreReports
    IgnoreReports,
    /// UnignoreReports
    UnignoreReports,
    /// SetPermissions
    SetPermissions,
    /// SetSuggestedSort
    SetSuggestedSort,
    /// Sticky
    Sticky,
    /// Unsticky
    Unsticky,
    /// SetContestMode
    SetContestMode,
    /// UnsetContestMode
    UnsetContestMode,
    /// Lock
    Lock,
    /// Unlock
    Unlock,
    /// MuteUser
    MuteUser,
    /// UnmuteUser
    UnmuteUser,
    /// CreateRule
    CreateRule,
    /// EditRule
    EditRule,
    /// ReorderRules
    ReorderRules,
    /// DeleteRule
    DeleteRule,
    /// Spoiler
    Spoiler,
    /// Unspoiler
    Unspoiler,
    #[serde(rename = "modmail_enrollment")]
    /// ModmailEnrollment
    ModmailEnrollment,
    #[serde(rename = "community_status")]
    /// CommunityStatus
    CommunityStatus,
    #[serde(rename = "community_styling")]
    /// CommunityStyling
    CommunityStyling,
    #[serde(rename = "community_welcome_page")]
    /// CommunityWelcomePage
    CommunityWelcomePage,
    #[serde(rename = "community_widgets")]
    /// CommunityWidgets
    CommunityWidgets,
    /// MarkOriginalContent
    MarkOriginalContent,
    /// Collections
    Collections,
    /// Events
    Events,
    #[serde(rename = "hidden_award")]
    /// HiddenAward
    HiddenAward,
    #[serde(rename = "add_community_topics")]
    /// AddCommunityTopics
    AddCommunityTopics,
    #[serde(rename = "remove_community_topics")]
    /// RemoveCommunityTopics
    RemoveCommunityTopics,
    #[serde(rename = "create_scheduled_post")]
    /// CreateScheduledPost
    CreateScheduledPost,
    #[serde(rename = "edit_scheduled_post")]
    /// EditScheduledPost
    EditScheduledPost,
    #[serde(rename = "delete_scheduled_post")]
    /// DeleteScheduledPost
    DeleteScheduledPost,
    #[serde(rename = "submit_scheduled_post")]
    /// SubmitScheduledPost
    SubmitScheduledPost,
    #[serde(rename = "edit_comment_requirements")]
    /// EditCommentRequirements
    EditCommentRequirements,
    #[serde(rename = "edit_post_requirements")]
    /// EditPostRequirements
    EditPostRequirements,
    /// InviteSubscriber
    InviteSubscriber,
    #[serde(rename = "submit_content_rating_survey")]
    /// SubmitContentRatingSurvey
    SubmitContentRatingSurvey,
    #[serde(rename = "adjust_post_crowd_control_level")]
    /// AdjustPostCrowdControlLevel
    AdjustPostCrowdControlLevel,
    #[serde(rename = "enable_post_crowd_control_filter")]
    /// EnablePostCrowdControlFilter
    EnablePostCrowdControlFilter,
    #[serde(rename = "disable_post_crowd_control_filter")]
    /// DisablePostCrowdControlFilter
    DisablePostCrowdControlFilter,
    /// DeleteOverriddenClassification
    DeleteOverriddenClassification,
    /// OverrideClassification
    OverrideClassification,
    /// ReorderModerators
    ReorderModerators,
    #[serde(rename = "request_assistance")]
    /// RequestAssistance
    RequestAssistance,
    /// SnoozeReports
    SnoozeReports,
    /// UnsnoozeReports
    UnsnoozeReports,
    /// AddNote
    AddNote,
    /// DeleteNote
    DeleteNote,
    /// AddRemovalReason
    AddRemovalReason,
    /// CreateRemovalReason
    CreateRemovalReason,
    /// UpdateRemovalReason
    UpdateRemovalReason,
    /// DeleteRemovalReason
    DeleteRemovalReason,
    /// ReorderRemovalReason
    ReorderRemovalReason,
    #[serde(rename = "dev_platform_app_changed")]
    /// DevPlatformAppChanged
    DevPlatformAppChanged,
    #[serde(rename = "dev_platform_app_disabled")]
    /// DevPlatformAppDisabled
    DevPlatformAppDisabled,
    #[serde(rename = "dev_platform_app_enabled")]
    /// DevPlatformAppEnabled
    DevPlatformAppEnabled,
    #[serde(rename = "dev_platform_app_installed")]
    /// DevPlatformAppInstalled
    DevPlatformAppInstalled,
    #[serde(rename = "dev_platform_app_uninstalled")]
    /// DevPlatformAppUninstalled
    DevPlatformAppUninstalled,
    #[serde(rename = "edit_saved_response")]
    /// EditSavedResponse
    EditSavedResponse,
    #[serde(rename = "chat_approve_message")]
    /// ChatApproveMessage
    ChatApproveMessage,
    #[serde(rename = "chat_remove_message")]
    /// ChatRemoveMessage
    ChatRemoveMessage,
    #[serde(rename = "chat_ban_user")]
    /// ChatBanUser
    ChatBanUser,
    #[serde(rename = "chat_unban_user")]
    /// ChatUnbanUser
    ChatUnbanUser,
    #[serde(rename = "chat_invite_host")]
    /// ChatInviteHost
    ChatInviteHost,
    #[serde(rename = "chat_remove_host")]
    /// ChatRemoveHost
    ChatRemoveHost,
    #[serde(rename = "approve_award")]
    /// ApproveAward
    ApproveAward,
}

/// An action taken by a subreddit moderator (or an admin / reddit)
#[derive(Debug, Deserialize)]
pub struct ModActionData {
    /// The unique ID of this mod action.
    pub id: String,
    /// Optional details of the action (can be empty string)
    pub details: String,
    /// What action was taken
    pub action: ModActionType,
    /// ??
    pub mod_id36: String,
    /// When the mod action was taken
    pub created_utc: f64,
    /// The name of the subreddit
    pub subreddit: String,
    /// If related to a post, the title of that post.
    pub target_title: Option<String>,
    /// If related to a post or comment, the relative permalink to it.
    pub target_permalink: Option<String>,
    /// If related to a post or comment, its markdown content.
    pub target_body: Option<String>,
    /// The name of the author
    pub target_author: Option<String>,
    /// The fullname of the related item
    pub target_fullname: Option<ThingFullname>,
    /// The moderator which took the action
    #[serde(rename = "mod")]
    pub moderator: String,
}

pub(crate) type ModLogListing = BasicThing<ListingNotFullname<BasicThing<ModActionData>>>;
