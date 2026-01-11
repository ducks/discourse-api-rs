use discourse_api_rs::DiscourseClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var("DISCOURSE_URL").expect("DISCOURSE_URL environment variable required");
    let api_key = env::var("DISCOURSE_API_KEY").expect("DISCOURSE_API_KEY environment variable required");
    let username = env::var("DISCOURSE_USERNAME").expect("DISCOURSE_USERNAME environment variable required");
    let mut category_id = env::var("TEST_CATEGORY_ID").ok().and_then(|s| s.parse::<u64>().ok());

    let client = DiscourseClient::with_api_key(&url, &api_key, &username);

    println!("Testing write operations against {}", url);
    println!("---");

    // If no category provided, fetch the first available one
    if category_id.is_none() {
        println!("\n0. Fetching available categories...");
        match client.get_categories().await {
            Ok(categories) => {
                if let Some(first_cat) = categories.first() {
                    category_id = Some(first_cat.id);
                    println!("✓ Using category: {} (id: {})", first_cat.name, first_cat.id);
                } else {
                    println!("✗ No categories found!");
                    return Ok(());
                }
            }
            Err(e) => {
                println!("✗ Failed to fetch categories: {:?}", e);
                return Ok(());
            }
        }
    }

    // Test 1: Create a topic
    println!("\n1. Creating a test topic...");
    let create_topic_result = client.create_topic(
        "Test Topic from discourse-api-rs",
        "This is a test topic created by discourse-api-rs to test write operations.",
        category_id,
    ).await;

    match create_topic_result {
        Ok(first_post) => {
            println!("✓ Created topic #{} with first post #{}", first_post.topic_id, first_post.id);
            let topic_id = first_post.topic_id;
            let first_post_id = first_post.id;

            // Test 2: Create a reply post
            println!("\n2. Creating a reply post...");
            match client.create_post(topic_id, "This is a reply post", None).await {
                Ok(post) => {
                    println!("✓ Created reply post #{}", post.id);
                    let reply_post_id = post.id;

                    // Test 3: Update the reply post
                    println!("\n3. Updating the reply post...");
                    match client.update_post(reply_post_id, "This reply has been updated!").await {
                        Ok(_) => println!("✓ Updated post #{}", reply_post_id),
                        Err(e) => println!("✗ Failed to update: {:?}", e),
                    }

                    // Test 4: Like the reply post
                    println!("\n4. Liking the reply post...");
                    match client.like_post(reply_post_id).await {
                        Ok(_) => println!("✓ Liked post #{}", reply_post_id),
                        Err(e) => println!("✗ Failed to like: {:?}", e),
                    }

                    // Test 5: Unlike the reply post
                    println!("\n5. Unliking the reply post...");
                    match client.unlike_post(reply_post_id).await {
                        Ok(_) => println!("✓ Unliked post #{}", reply_post_id),
                        Err(e) => println!("✗ Failed to unlike: {:?}", e),
                    }

                    // Test 6: Delete the reply post
                    println!("\n6. Deleting the reply post...");
                    match client.delete_post(reply_post_id).await {
                        Ok(_) => println!("✓ Deleted post #{}", reply_post_id),
                        Err(e) => println!("✗ Failed to delete: {:?}", e),
                    }
                }
                Err(e) => {
                    println!("✗ Failed to create reply: {:?}", e);
                }
            }

            // Test 7: Delete the topic (by deleting the first post)
            println!("\n7. Deleting the topic (via first post)...");
            match client.delete_post(first_post_id).await {
                Ok(_) => println!("✓ Deleted topic #{} (first post #{})", topic_id, first_post_id),
                Err(e) => println!("✗ Failed to delete topic: {:?}", e),
            }
        }
        Err(e) => {
            println!("✗ Failed to create topic: {:?}", e);
        }
    }

    println!("\n---");
    println!("All tests complete!");
    Ok(())
}
