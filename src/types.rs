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
