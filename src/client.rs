use crate::error::Result;
use crate::types::*;
use reqwest::Client;

pub struct DiscourseClient {
    base_url: String,
    client: Client,
    api_key: Option<String>,
    api_username: Option<String>,
}

impl DiscourseClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
            api_key: None,
            api_username: None,
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
            api_key: Some(api_key.into()),
            api_username: Some(api_username.into()),
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn add_auth_headers(&self, mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let (Some(key), Some(username)) = (&self.api_key, &self.api_username) {
            request = request.header("Api-Key", key).header("Api-Username", username);
        }
        request
    }

    pub async fn get_latest(&self) -> Result<LatestResponse> {
        let url = self.build_url("/latest.json");
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        let data: LatestResponse = response.json().await?;
        Ok(data)
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let url = self.build_url("/categories.json");
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        let data: CategoryList = response.json().await?;
        Ok(data.category_list.categories)
    }

    pub async fn get_topic(&self, topic_id: u64) -> Result<TopicResponse> {
        let url = self.build_url(&format!("/t/{}.json?include_raw=1", topic_id));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        let data: TopicResponse = response.json().await?;
        Ok(data)
    }

    pub async fn get_post(&self, post_id: u64) -> Result<Post> {
        let url = self.build_url(&format!("/posts/{}.json", post_id));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        let data: Post = response.json().await?;
        Ok(data)
    }

    pub async fn get_category_topics(&self, category_id: u64) -> Result<LatestResponse> {
        let url = self.build_url(&format!("/c/{}/l/latest.json", category_id));
        let request = self.add_auth_headers(self.client.get(&url));
        let response = request.send().await?;
        let data: LatestResponse = response.json().await?;
        Ok(data)
    }
}
