use crate::error::Result;
use crate::types::*;
use reqwest::Client;

pub enum AuthType {
    None,
    AdminKey { api_key: String, api_username: String },
    UserKey { user_api_key: String, user_api_client_id: Option<String> },
}

pub struct DiscourseClient {
    base_url: String,
    client: Client,
    auth: AuthType,
}

impl DiscourseClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
            auth: AuthType::None,
        }
    }

    pub fn with_api_key(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        api_username: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
            auth: AuthType::AdminKey {
                api_key: api_key.into(),
                api_username: api_username.into(),
            },
        }
    }

    pub fn with_user_api_key(
        base_url: impl Into<String>,
        user_api_key: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
            auth: AuthType::UserKey {
                user_api_key: user_api_key.into(),
                user_api_client_id: None,
            },
        }
    }

    pub fn with_user_api_key_and_client_id(
        base_url: impl Into<String>,
        user_api_key: impl Into<String>,
        user_api_client_id: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
            auth: AuthType::UserKey {
                user_api_key: user_api_key.into(),
                user_api_client_id: Some(user_api_client_id.into()),
            },
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn add_auth_headers(&self, mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self.auth {
            AuthType::None => {},
            AuthType::AdminKey { api_key, api_username } => {
                request = request.header("Api-Key", api_key).header("Api-Username", api_username);
            }
            AuthType::UserKey { user_api_key, user_api_client_id } => {
                request = request.header("User-Api-Key", user_api_key);
                if let Some(client_id) = user_api_client_id {
                    request = request.header("User-Api-Client-Id", client_id);
                }
            }
        }
        request
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();
        if !status.is_success() {
            // Try to parse as Discourse error response
            if let Ok(error_response) = response.json::<ErrorResponse>().await {
                return Err(crate::error::Error::Api(error_response.errors.join(", ")));
            }
            return Err(crate::error::Error::Api(format!("HTTP {}", status)));
        }
        let data: T = response.json().await?;
        Ok(data)
    }

    pub async fn get_latest(&self) -> Result<LatestResponse> {
        let url = self.build_url("/latest.json");
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let url = self.build_url("/categories.json");
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        let data: CategoryList = self.handle_response(response).await?;
        Ok(data.category_list.categories)
    }

    pub async fn get_topic(&self, topic_id: u64) -> Result<TopicResponse> {
        let url = self.build_url(&format!("/t/{}.json?include_raw=1", topic_id));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn get_post(&self, post_id: u64) -> Result<Post> {
        let url = self.build_url(&format!("/posts/{}.json", post_id));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn get_category_topics(&self, category_id: u64) -> Result<LatestResponse> {
        let url = self.build_url(&format!("/c/{}/l/latest.json", category_id));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn get_user_channels(&self) -> Result<ChatChannelsResponse> {
        let url = self.build_url("/chat/api/me/channels");
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn get_channel_messages(&self, channel_id: u64) -> Result<ChatMessagesResponse> {
        let url = self.build_url(&format!("/chat/api/channels/{}/messages", channel_id));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn send_chat_message(
        &self,
        channel_id: u64,
        message: &str,
    ) -> Result<CreateMessageResponse> {
        let url = self.build_url(&format!("/chat/{}", channel_id));
        let mut request = self.add_auth_headers(self.client.post(&url));
        let body = serde_json::json!({
            "message": message,
        });
        request = request.json(&body);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn create_post(
        &self,
        topic_id: u64,
        raw: &str,
        reply_to_post_number: Option<u32>,
    ) -> Result<CreatePostResponse> {
        let url = self.build_url("/posts.json");
        let mut request = self.add_auth_headers(self.client.post(&url));
        let mut body = serde_json::json!({
            "raw": raw,
            "topic_id": topic_id,
        });
        if let Some(reply_to) = reply_to_post_number {
            body["reply_to_post_number"] = serde_json::json!(reply_to);
        }
        request = request.json(&body);
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn get_notifications(&self) -> Result<NotificationsResponse> {
        let url = self.build_url("/notifications.json");
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }
}
