use serde::{Deserialize, Serialize};

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
    pub user_id: u64,
    pub description: String,
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
    pub title: String,
    pub posts_count: u32,
    pub category_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostStream {
    pub posts: Vec<Post>,
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
    pub id: u64,
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
