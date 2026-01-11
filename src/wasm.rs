use wasm_bindgen::prelude::*;
use crate::DiscourseClient;

#[wasm_bindgen]
pub struct WasmDiscourseClient {
    inner: DiscourseClient,
}

#[wasm_bindgen]
impl WasmDiscourseClient {
    #[wasm_bindgen(constructor)]
    pub fn new(base_url: String) -> Self {
        Self {
            inner: DiscourseClient::new(base_url),
        }
    }

    #[wasm_bindgen(js_name = withApiKey)]
    pub fn with_api_key(base_url: String, api_key: String, api_username: String) -> Self {
        Self {
            inner: DiscourseClient::with_api_key(base_url, api_key, api_username),
        }
    }

    #[wasm_bindgen(js_name = withUserApiKey)]
    pub fn with_user_api_key(base_url: String, user_api_key: String) -> Self {
        Self {
            inner: DiscourseClient::with_user_api_key(base_url, user_api_key),
        }
    }

    #[wasm_bindgen(js_name = getLatest)]
    pub async fn get_latest(&self) -> Result<JsValue, JsValue> {
        self.inner
            .get_latest()
            .await
            .map(|r| serde_wasm_bindgen::to_value(&r).unwrap())
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen(js_name = getTopic)]
    pub async fn get_topic(&self, topic_id: u64) -> Result<JsValue, JsValue> {
        self.inner
            .get_topic(topic_id)
            .await
            .map(|r| serde_wasm_bindgen::to_value(&r).unwrap())
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen(js_name = createTopic)]
    pub async fn create_topic(
        &self,
        title: String,
        raw: String,
        category_id: Option<u64>,
    ) -> Result<JsValue, JsValue> {
        self.inner
            .create_topic(&title, &raw, category_id)
            .await
            .map(|r| serde_wasm_bindgen::to_value(&r).unwrap())
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen(js_name = createPost)]
    pub async fn create_post(
        &self,
        topic_id: u64,
        raw: String,
        reply_to_post_number: Option<u32>,
    ) -> Result<JsValue, JsValue> {
        self.inner
            .create_post(topic_id, &raw, reply_to_post_number)
            .await
            .map(|r| serde_wasm_bindgen::to_value(&r).unwrap())
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen(js_name = updatePost)]
    pub async fn update_post(&self, post_id: u64, raw: String) -> Result<(), JsValue> {
        self.inner
            .update_post(post_id, &raw)
            .await
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen(js_name = deletePost)]
    pub async fn delete_post(&self, post_id: u64) -> Result<(), JsValue> {
        self.inner
            .delete_post(post_id)
            .await
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen(js_name = likePost)]
    pub async fn like_post(&self, post_id: u64) -> Result<(), JsValue> {
        self.inner
            .like_post(post_id)
            .await
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen(js_name = unlikePost)]
    pub async fn unlike_post(&self, post_id: u64) -> Result<(), JsValue> {
        self.inner
            .unlike_post(post_id)
            .await
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen(js_name = getCategories)]
    pub async fn get_categories(&self) -> Result<JsValue, JsValue> {
        self.inner
            .get_categories()
            .await
            .map(|r| serde_wasm_bindgen::to_value(&r).unwrap())
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }
}
