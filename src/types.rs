use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<String>,
    #[serde(default)]
    pub error_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub posts_count: u32,
    pub reply_count: u32,
    pub views: u32,
    pub like_count: u32,
    pub created_at: String,
    pub last_posted_at: Option<String>,
    pub pinned: bool,
    pub visible: bool,
    pub closed: bool,
    pub archived: bool,
    pub has_summary: bool,
    pub category_id: Option<u64>,
    pub posters: Vec<Poster>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poster {
    pub user_id: i64,
    pub description: String,
    #[serde(default)]
    pub extras: Option<String>,
    #[serde(default)]
    pub primary_group_id: Option<u64>,
    #[serde(default)]
    pub flair_group_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestResponse {
    pub topic_list: TopicList,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicList {
    pub topics: Vec<Topic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicResponse {
    pub post_stream: PostStream,
    pub id: u64,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub posts_count: Option<u32>,
    pub category_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostStream {
    pub posts: Vec<Post>,
    #[serde(default)]
    pub stream: Vec<u64>,
}

/// The like action. Matches `PostActionType::LIKE_POST_ACTION_ID` in core.
pub const LIKE_POST_ACTION_ID: u32 = 2;

/// What the current user can do with one action type on a post, and what
/// others have already done.
///
/// Core builds these in `PostSerializer#actions_summary`, and it *omits*
/// fields rather than sending falsey values: `count` is deleted when it is
/// zero, `acted` appears only if you acted, and `can_act` only if you are
/// permitted. An entry is left out entirely unless at least one of those
/// applies, so an absent summary is meaningful rather than an error.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ActionSummary {
    pub id: u32,
    /// Absent from the payload when zero.
    #[serde(default)]
    pub count: u32,
    /// Whether the current user has taken this action.
    #[serde(default)]
    pub acted: bool,
    /// Whether the current user may take this action now.
    #[serde(default)]
    pub can_act: bool,
    /// Whether an action already taken may be undone. Core withholds this
    /// even for some posts you have liked, so it is not simply `acted`.
    #[serde(default)]
    pub can_undo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: u64,
    pub username: String,
    pub created_at: String,
    pub cooked: String,
    #[serde(default)]
    pub raw: Option<String>,
    pub post_number: u32,
    pub post_type: u32,
    pub reply_count: u32,
    pub quote_count: u32,
    pub reads: u32,
    pub score: f64,
    pub topic_id: u64,
    /// Per-action state. Absent for endpoints that do not serialize it.
    #[serde(default)]
    pub actions_summary: Vec<ActionSummary>,
}

impl Post {
    /// The like summary, if core sent one.
    pub fn like_summary(&self) -> Option<&ActionSummary> {
        self.actions_summary
            .iter()
            .find(|a| a.id == LIKE_POST_ACTION_ID)
    }

    /// How many likes this post has. Zero when core omitted the summary,
    /// which is what it does for a post nobody has liked.
    pub fn like_count(&self) -> u32 {
        self.like_summary().map_or(0, |a| a.count)
    }

    /// Whether the current user has liked this post.
    pub fn is_liked(&self) -> bool {
        self.like_summary().is_some_and(|a| a.acted)
    }

    /// Whether the current user can like this post now.
    pub fn can_like(&self) -> bool {
        self.like_summary().is_some_and(|a| a.can_act)
    }

    /// Whether an existing like can be withdrawn. Core sends `can_undo`
    /// separately from `acted`, so a liked post is not always unlikeable.
    pub fn can_unlike(&self) -> bool {
        self.like_summary().is_some_and(|a| a.acted && a.can_undo)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub name: Option<String>,
    pub avatar_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub color: String,
    pub text_color: String,
    pub slug: String,
    pub topic_count: u32,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub has_children: Option<bool>,
    pub parent_category_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryList {
    pub category_list: CategoryListData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryListData {
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChannel {
    pub id: u64,
    pub title: String,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub chatable_id: Option<u64>,
    #[serde(default)]
    pub chatable_type: Option<String>,
    #[serde(default)]
    pub memberships_count: Option<u32>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub allow_channel_wide_mentions: Option<bool>,
    #[serde(default)]
    pub chatable: Option<serde_json::Value>,
    #[serde(default)]
    pub chatable_url: Option<String>,
    #[serde(default)]
    pub current_user_membership: Option<serde_json::Value>,
    #[serde(default)]
    pub icon_upload_url: Option<String>,
    #[serde(default)]
    pub last_message: Option<serde_json::Value>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
    #[serde(default)]
    pub threading_enabled: Option<bool>,
    #[serde(default)]
    pub unicode_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChannelsResponse {
    pub public_channels: Option<Vec<ChatChannel>>,
    pub direct_message_channels: Option<Vec<ChatChannel>>,
    #[serde(default)]
    pub channels: Vec<ChatChannel>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
    #[serde(default)]
    pub tracking: Option<serde_json::Value>,
    #[serde(default)]
    pub global_presence_channel_state: Option<serde_json::Value>,
    #[serde(default)]
    pub unread_thread_overview: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: u64,
    pub message: String,
    pub cooked: String,
    pub created_at: String,
    pub user: User,
    pub chat_channel_id: u64,
    #[serde(default)]
    pub deleted_at: Option<String>,
    #[serde(default)]
    pub excerpt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessagesResponse {
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub meta: ChatMessagesMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatMessagesMeta {
    pub can_load_more_past: Option<bool>,
    pub can_load_more_future: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageResponse {
    pub success: String,
    pub message_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostResponse {
    pub id: u64,
    pub name: Option<String>,
    pub username: String,
    pub avatar_template: String,
    pub created_at: String,
    pub cooked: String,
    pub post_number: u32,
    pub post_type: u32,
    pub updated_at: String,
    pub reply_count: u32,
    pub reply_to_post_number: Option<u32>,
    pub quote_count: u32,
    pub topic_id: u64,
    pub topic_slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: u64,
    pub user_id: u64,
    pub notification_type: u32,
    pub read: bool,
    pub high_priority: bool,
    pub created_at: String,
    pub post_number: Option<u32>,
    pub topic_id: Option<u64>,
    pub slug: Option<String>,
    pub fancy_title: Option<String>,
    #[serde(default)]
    pub data: NotificationData,
    #[serde(default)]
    pub acting_user_avatar_template: Option<String>,
    #[serde(default)]
    pub acting_user_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationData {
    #[serde(default)]
    pub topic_title: Option<String>,
    #[serde(default)]
    pub original_username: Option<String>,
    #[serde(default)]
    pub display_username: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsResponse {
    pub notifications: Vec<Notification>,
}

// -- Users --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: UserDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDetails {
    pub id: i64,
    pub username: String,
    #[serde(default)]
    pub name: Option<String>,
    pub avatar_template: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub bio_raw: Option<String>,
    #[serde(default)]
    pub bio_cooked: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub admin: Option<bool>,
    #[serde(default)]
    pub moderator: Option<bool>,
    #[serde(default)]
    pub trust_level: Option<u32>,
    pub created_at: String,
    #[serde(default)]
    pub last_seen_at: Option<String>,
    #[serde(default)]
    pub groups: Vec<UserGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroup {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub flair_url: Option<String>,
    #[serde(default)]
    pub flair_bg_color: Option<String>,
    #[serde(default)]
    pub flair_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListResponse {
    #[serde(flatten)]
    pub data: Vec<UserListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListItem {
    pub id: i64,
    pub username: String,
    #[serde(default)]
    pub name: Option<String>,
    pub avatar_template: String,
    #[serde(default)]
    pub admin: Option<bool>,
    #[serde(default)]
    pub moderator: Option<bool>,
    #[serde(default)]
    pub trust_level: Option<u32>,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub staged: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub success: bool,
    pub active: bool,
    pub message: String,
    #[serde(default)]
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameCheckResponse {
    pub available: bool,
    #[serde(default)]
    pub suggestion: Option<String>,
}

// -- Groups --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub user_count: Option<u32>,
    #[serde(default)]
    pub mentionable_level: Option<u32>,
    #[serde(default)]
    pub messageable_level: Option<u32>,
    #[serde(default)]
    pub visibility_level: Option<u32>,
    #[serde(default)]
    pub automatic: Option<bool>,
    #[serde(default)]
    pub bio_raw: Option<String>,
    #[serde(default)]
    pub bio_cooked: Option<String>,
    #[serde(default)]
    pub flair_url: Option<String>,
    #[serde(default)]
    pub flair_bg_color: Option<String>,
    #[serde(default)]
    pub flair_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupsResponse {
    pub groups: Vec<Group>,
    #[serde(default)]
    pub total_rows_groups: Option<u32>,
    #[serde(default)]
    pub load_more_groups: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupResponse {
    pub group: Group,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMembersResponse {
    pub members: Vec<GroupMember>,
    pub owners: Vec<GroupMember>,
    pub meta: GroupMembersMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    pub id: i64,
    pub username: String,
    #[serde(default)]
    pub name: Option<String>,
    pub avatar_template: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub last_seen_at: Option<String>,
    #[serde(default)]
    pub added_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMembersMeta {
    pub total: u32,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupModifyMembersResponse {
    pub success: String,
    #[serde(default)]
    pub usernames: Vec<String>,
}

#[cfg(test)]
mod action_summary_tests {
    use super::*;

    /// A post as core serializes it, with whatever actions_summary is given.
    fn post_json(actions_summary: &str) -> String {
        format!(
            r#"{{
                "id": 1,
                "username": "someone",
                "created_at": "2026-08-10T12:00:00.000Z",
                "cooked": "<p>hi</p>",
                "post_number": 1,
                "post_type": 1,
                "reply_count": 0,
                "quote_count": 0,
                "reads": 1,
                "score": 0.0,
                "topic_id": 1,
                "actions_summary": {}
            }}"#,
            actions_summary
        )
    }

    fn post(actions_summary: &str) -> Post {
        serde_json::from_str(&post_json(actions_summary)).expect("deserialise post")
    }

    #[test]
    fn a_post_nobody_liked_has_no_like_entry() {
        // Core deletes count when zero and drops the whole summary unless
        // can_act, count, or acted applies, so an anonymous view of an
        // unliked post sends an empty array.
        let p = post("[]");
        assert_eq!(p.like_count(), 0);
        assert!(!p.is_liked());
        assert!(!p.can_like());
        assert!(!p.can_unlike());
    }

    #[test]
    fn a_likeable_unliked_post_sends_can_act_with_no_count() {
        // The shape core sends most often: you may like it, nobody has.
        let p = post(r#"[{"id": 2, "can_act": true}]"#);
        assert_eq!(p.like_count(), 0);
        assert!(!p.is_liked());
        assert!(p.can_like());
        assert!(!p.can_unlike());
    }

    #[test]
    fn a_liked_post_reports_the_count_and_acted() {
        let p = post(r#"[{"id": 2, "count": 3, "acted": true, "can_undo": true}]"#);
        assert_eq!(p.like_count(), 3);
        assert!(p.is_liked());
        assert!(p.can_unlike());
        // Having liked it, you cannot like it again.
        assert!(!p.can_like());
    }

    #[test]
    fn a_liked_post_without_can_undo_cannot_be_unliked() {
        // Core withholds can_undo once the undo window has passed, so acted
        // alone does not mean the like can be withdrawn.
        let p = post(r#"[{"id": 2, "count": 1, "acted": true}]"#);
        assert!(p.is_liked());
        assert!(!p.can_unlike());
    }

    #[test]
    fn likes_by_others_show_a_count_without_acted() {
        let p = post(r#"[{"id": 2, "count": 5, "can_act": true}]"#);
        assert_eq!(p.like_count(), 5);
        assert!(!p.is_liked());
        assert!(p.can_like());
    }

    #[test]
    fn flag_summaries_are_not_mistaken_for_likes() {
        // Flags share the array and have their own type ids; only id 2 is
        // the like.
        let p = post(r#"[{"id": 8, "count": 2, "acted": true}, {"id": 2, "count": 1}]"#);
        assert_eq!(p.like_count(), 1);
        assert!(!p.is_liked(), "acted on a flag, not a like");
    }

    #[test]
    fn a_post_with_no_actions_summary_field_still_deserialises() {
        // Not every endpoint serializes actions_summary.
        let json = r#"{
            "id": 1, "username": "someone",
            "created_at": "2026-08-10T12:00:00.000Z",
            "cooked": "<p>hi</p>", "post_number": 1, "post_type": 1,
            "reply_count": 0, "quote_count": 0, "reads": 1,
            "score": 0.0, "topic_id": 1
        }"#;
        let p: Post = serde_json::from_str(json).expect("deserialise without the field");
        assert!(p.actions_summary.is_empty());
        assert_eq!(p.like_count(), 0);
        assert!(!p.is_liked());
    }

    #[test]
    fn the_like_action_id_matches_core() {
        // PostActionType::LIKE_POST_ACTION_ID = 2
        assert_eq!(LIKE_POST_ACTION_ID, 2);
    }
}
