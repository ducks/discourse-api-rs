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
        self.get_latest_page(0).await
    }

    pub async fn get_latest_page(&self, page: u32) -> Result<LatestResponse> {
        let url = self.build_url(&format!("/latest.json?page={}", page));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Full-page search.
    ///
    /// The term is passed as a query parameter rather than interpolated so
    /// that spaces and Discourse's own filter syntax (`#category`,
    /// `@username`, `in:title`) survive intact.
    ///
    /// Core rejects a term shorter than `min_search_term_length` with a 400,
    /// so short queries surface as an API error rather than an empty result.
    pub async fn search(&self, term: &str) -> Result<SearchResponse> {
        self.search_page(term, 0).await
    }

    /// One page of search results. Pages are 1-based in core; page 0 and
    /// page 1 both return the first page.
    pub async fn search_page(&self, term: &str, page: u32) -> Result<SearchResponse> {
        let url = self.build_url("/search.json");
        let request = self
            .add_auth_headers(self.client.get(&url))
            .query(&[("q", term), ("page", &page.to_string())]);
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
        self.get_topic_from_post(topic_id, None).await
    }

    pub async fn get_topic_from_post(&self, topic_id: u64, after_post_number: Option<u32>) -> Result<TopicResponse> {
        let url = if let Some(post_num) = after_post_number {
            format!("/t/{}/{}.json?include_raw=1", topic_id, post_num)
        } else {
            format!("/t/{}.json?include_raw=1", topic_id)
        };
        let url = self.build_url(&url);
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn get_topic_posts(&self, topic_id: u64, post_ids: Option<Vec<u64>>) -> Result<TopicResponse> {
        let mut url = format!("/t/{}/posts.json?include_raw=1", topic_id);
        if let Some(ids) = post_ids {
            for id in ids {
                url.push_str(&format!("&post_ids[]={}", id));
            }
        }
        let url = self.build_url(&url);
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

    pub async fn create_topic(
        &self,
        title: &str,
        raw: &str,
        category_id: Option<u64>,
    ) -> Result<CreatePostResponse> {
        let url = self.build_url("/posts.json");
        let request = self.add_auth_headers(self.client.post(&url));
        let mut body = serde_json::json!({
            "title": title,
            "raw": raw,
        });
        if let Some(cat_id) = category_id {
            body["category"] = serde_json::json!(cat_id);
        }
        let response = request.json(&body).send().await?;
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

    pub async fn update_post(&self, post_id: u64, raw: &str) -> Result<()> {
        let url = self.build_url(&format!("/posts/{}.json", post_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let body = serde_json::json!({
            "post": {
                "raw": raw,
            }
        });
        let response = request.json(&body).send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn delete_post(&self, post_id: u64) -> Result<()> {
        let url = self.build_url(&format!("/posts/{}.json", post_id));
        let request = self.add_auth_headers(self.client.delete(&url));
        let response = request.send().await?;
        let status = response.status();
        if !status.is_success() {
            if let Ok(error_response) = response.json::<crate::types::ErrorResponse>().await {
                return Err(crate::error::Error::Api(error_response.errors.join(", ")));
            }
            return Err(crate::error::Error::Api(format!("HTTP {}", status)));
        }
        Ok(())
    }

    pub async fn like_post(&self, post_id: u64) -> Result<()> {
        let url = self.build_url("/post_actions");
        let request = self.add_auth_headers(self.client.post(&url));
        let body = serde_json::json!({
            "id": post_id,
            "post_action_type_id": 2,
        });
        let response = request.json(&body).send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn unlike_post(&self, post_id: u64) -> Result<()> {
        let url = self.build_url(&format!("/post_actions/{}", post_id));
        let request = self.add_auth_headers(self.client.delete(&url));
        let response = request
            .query(&[("post_action_type_id", "2")])
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            if let Ok(error_response) = response.json::<crate::types::ErrorResponse>().await {
                return Err(crate::error::Error::Api(error_response.errors.join(", ")));
            }
            return Err(crate::error::Error::Api(format!("HTTP {}", status)));
        }
        Ok(())
    }

    pub async fn get_notifications(&self) -> Result<NotificationsResponse> {
        let url = self.build_url("/notifications.json");
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    // -- Users --

    pub async fn get_user(&self, username: &str) -> Result<UserResponse> {
        let url = self.build_url(&format!("/users/{}.json", username));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn get_user_by_external_id(&self, external_id: &str) -> Result<UserResponse> {
        let url = self.build_url(&format!("/users/by-external/{}.json", external_id));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn create_user(
        &self,
        name: &str,
        email: &str,
        password: &str,
        username: &str,
    ) -> Result<CreateUserResponse> {
        let url = self.build_url("/users.json");
        let request = self.add_auth_headers(self.client.post(&url));
        let body = serde_json::json!({
            "name": name,
            "email": email,
            "password": password,
            "username": username,
            "active": true,
            "approved": true,
        });
        let response = request.json(&body).send().await?;
        self.handle_response(response).await
    }

    pub async fn update_user(
        &self,
        username: &str,
        params: serde_json::Value,
    ) -> Result<UserResponse> {
        let url = self.build_url(&format!("/u/{}.json", username));
        let request = self.add_auth_headers(self.client.put(&url));
        let response = request.json(&params).send().await?;
        self.handle_response(response).await
    }

    pub async fn list_users(&self, list_type: &str) -> Result<Vec<UserListItem>> {
        let url = self.build_url(&format!("/admin/users/list/{}.json", list_type));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn suspend_user(
        &self,
        user_id: i64,
        duration: u32,
        reason: &str,
    ) -> Result<()> {
        let url = self.build_url(&format!("/admin/users/{}/suspend.json", user_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let body = serde_json::json!({
            "suspend_until": format!("{}d", duration),
            "reason": reason,
        });
        let response = request.json(&body).send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn unsuspend_user(&self, user_id: i64) -> Result<()> {
        let url = self.build_url(&format!("/admin/users/{}/unsuspend.json", user_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let response = request.send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn delete_user(&self, user_id: i64, delete_posts: bool) -> Result<()> {
        let url = self.build_url(&format!(
            "/admin/users/{}.json?delete_posts={}",
            user_id, delete_posts
        ));
        let request = self.add_auth_headers(self.client.delete(&url));
        let response = request.send().await?;
        let status = response.status();
        if !status.is_success() {
            if let Ok(error_response) = response.json::<ErrorResponse>().await {
                return Err(crate::error::Error::Api(error_response.errors.join(", ")));
            }
            return Err(crate::error::Error::Api(format!("HTTP {}", status)));
        }
        Ok(())
    }

    pub async fn grant_admin(&self, user_id: i64) -> Result<()> {
        let url = self.build_url(&format!("/admin/users/{}/grant_admin.json", user_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let response = request.send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn revoke_admin(&self, user_id: i64) -> Result<()> {
        let url = self.build_url(&format!("/admin/users/{}/revoke_admin.json", user_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let response = request.send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn grant_moderation(&self, user_id: i64) -> Result<()> {
        let url = self.build_url(&format!("/admin/users/{}/grant_moderation.json", user_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let response = request.send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn revoke_moderation(&self, user_id: i64) -> Result<()> {
        let url = self.build_url(&format!("/admin/users/{}/revoke_moderation.json", user_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let response = request.send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn update_username(&self, username: &str, new_username: &str) -> Result<()> {
        let url = self.build_url(&format!("/u/{}/preferences/username.json", username));
        let request = self.add_auth_headers(self.client.put(&url));
        let body = serde_json::json!({ "new_username": new_username });
        let response = request.json(&body).send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn update_email(&self, username: &str, email: &str) -> Result<()> {
        let url = self.build_url(&format!("/u/{}/preferences/email.json", username));
        let request = self.add_auth_headers(self.client.put(&url));
        let body = serde_json::json!({ "email": email });
        let response = request.json(&body).send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn check_username(&self, username: &str) -> Result<UsernameCheckResponse> {
        let url = self.build_url(&format!("/users/check_username.json?username={}", username));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn activate_user(&self, user_id: i64) -> Result<()> {
        let url = self.build_url(&format!("/admin/users/{}/activate.json", user_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let response = request.send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn deactivate_user(&self, user_id: i64) -> Result<()> {
        let url = self.build_url(&format!("/admin/users/{}/deactivate.json", user_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let response = request.send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn update_trust_level(&self, user_id: i64, level: u32) -> Result<()> {
        let url = self.build_url(&format!("/admin/users/{}/trust_level.json", user_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let body = serde_json::json!({ "level": level });
        let response = request.json(&body).send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn log_out_user(&self, user_id: i64) -> Result<()> {
        let url = self.build_url(&format!("/admin/users/{}/log_out.json", user_id));
        let request = self.add_auth_headers(self.client.post(&url));
        let response = request.send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    pub async fn anonymize_user(&self, user_id: i64) -> Result<()> {
        let url = self.build_url(&format!("/admin/users/{}/anonymize.json", user_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let response = request.send().await?;
        let _: serde_json::Value = self.handle_response(response).await?;
        Ok(())
    }

    // -- Groups --

    pub async fn get_groups(&self) -> Result<GroupsResponse> {
        let url = self.build_url("/groups.json");
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn get_group(&self, group_name: &str) -> Result<GroupResponse> {
        let url = self.build_url(&format!("/groups/{}.json", group_name));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn create_group(
        &self,
        name: &str,
        params: serde_json::Value,
    ) -> Result<GroupResponse> {
        let url = self.build_url("/admin/groups.json");
        let request = self.add_auth_headers(self.client.post(&url));
        let mut body = params;
        body["group"]["name"] = serde_json::json!(name);
        let response = request.json(&body).send().await?;
        self.handle_response(response).await
    }

    pub async fn delete_group(&self, group_id: u64) -> Result<()> {
        let url = self.build_url(&format!("/admin/groups/{}.json", group_id));
        let request = self.add_auth_headers(self.client.delete(&url));
        let response = request.send().await?;
        let status = response.status();
        if !status.is_success() {
            if let Ok(error_response) = response.json::<ErrorResponse>().await {
                return Err(crate::error::Error::Api(error_response.errors.join(", ")));
            }
            return Err(crate::error::Error::Api(format!("HTTP {}", status)));
        }
        Ok(())
    }

    pub async fn get_group_members(
        &self,
        group_name: &str,
    ) -> Result<GroupMembersResponse> {
        let url = self.build_url(&format!("/groups/{}/members.json", group_name));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        self.handle_response(response).await
    }

    pub async fn add_group_members(
        &self,
        group_id: u64,
        usernames: &[&str],
    ) -> Result<GroupModifyMembersResponse> {
        let url = self.build_url(&format!("/admin/groups/{}/members.json", group_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let body = serde_json::json!({
            "usernames": usernames.join(","),
        });
        let response = request.json(&body).send().await?;
        self.handle_response(response).await
    }

    pub async fn remove_group_members(
        &self,
        group_id: u64,
        usernames: &[&str],
    ) -> Result<GroupModifyMembersResponse> {
        let url = self.build_url(&format!("/admin/groups/{}/members.json", group_id));
        let request = self.add_auth_headers(self.client.delete(&url));
        let body = serde_json::json!({
            "usernames": usernames.join(","),
        });
        let response = request.json(&body).send().await?;
        self.handle_response(response).await
    }

    pub async fn update_group(
        &self,
        group_id: u64,
        params: serde_json::Value,
    ) -> Result<GroupResponse> {
        let url = self.build_url(&format!("/groups/{}.json", group_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let response = request.json(&params).send().await?;
        self.handle_response(response).await
    }

    pub async fn add_group_owners(
        &self,
        group_id: u64,
        usernames: &[&str],
    ) -> Result<GroupModifyMembersResponse> {
        let url = self.build_url(&format!("/admin/groups/{}/owners.json", group_id));
        let request = self.add_auth_headers(self.client.put(&url));
        let body = serde_json::json!({
            "usernames": usernames.join(","),
        });
        let response = request.json(&body).send().await?;
        self.handle_response(response).await
    }

    pub async fn remove_group_owners(
        &self,
        group_id: u64,
        usernames: &[&str],
    ) -> Result<GroupModifyMembersResponse> {
        let url = self.build_url(&format!("/admin/groups/{}/owners.json", group_id));
        let request = self.add_auth_headers(self.client.delete(&url));
        let body = serde_json::json!({
            "usernames": usernames.join(","),
        });
        let response = request.json(&body).send().await?;
        self.handle_response(response).await
    }
}
