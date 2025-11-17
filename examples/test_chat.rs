use discourse_api_rs::DiscourseClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var("DISCOURSE_URL").unwrap_or_else(|_| "https://meta.discourse.org".to_string());
    let api_key = env::var("DISCOURSE_API_KEY").expect("DISCOURSE_API_KEY environment variable required");

    let client = DiscourseClient::with_user_api_key(&url, &api_key);

    match client.get_user_channels().await {
        Ok(channels) => {
            println!("Success! Got {} public channels",
                channels.public_channels.as_ref().map(|c| c.len()).unwrap_or(0));
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    Ok(())
}
