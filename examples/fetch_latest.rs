use discourse_api::DiscourseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example: Fetch latest topics from Discourse Meta
    let client = DiscourseClient::new("https://meta.discourse.org");

    println!("Fetching latest topics...\n");

    let response = client.get_latest().await?;

    for topic in response.topic_list.topics.iter().take(10) {
        println!("📝 {}", topic.title);
        println!("   ID: {} | Replies: {} | Views: {}",
            topic.id, topic.reply_count, topic.views);
        println!();
    }

    println!("\nFetching categories...\n");

    let categories = client.get_categories().await?;

    for category in categories.iter().take(5) {
        println!("📁 {}", category.name);
        println!("   Slug: {} | Topics: {}", category.slug, category.topic_count);
        println!();
    }

    Ok(())
}
