//! Search deserialisation, checked against responses captured from a real
//! Discourse instance rather than JSON written to match the structs.

use discourse_api_rs::{SearchResponse, SearchTopic};

fn with_results() -> SearchResponse {
    let raw = include_str!("fixtures/search_meta_discourse.json");
    serde_json::from_str(raw).expect("deserialise a real search response")
}

fn without_results() -> SearchResponse {
    let raw = include_str!("fixtures/search_no_results.json");
    serde_json::from_str(raw).expect("deserialise an empty search response")
}

#[test]
fn parses_a_real_search_response() {
    let response = with_results();
    assert!(!response.posts.is_empty(), "expected hits");
    assert!(!response.topics.is_empty(), "expected topics alongside");
    assert!(!response.is_empty());
}

#[test]
fn search_hits_carry_a_blurb_rather_than_cooked() {
    // SearchPostSerializer sets include_cooked? to false, so the excerpt is
    // the only body text available for a result row.
    let response = with_results();
    let post = &response.posts[0];
    let blurb = post.blurb.as_ref().expect("a blurb on a real hit");
    assert!(!blurb.is_empty());
}

#[test]
fn a_missing_display_name_parses() {
    // Core sends "" for users with no name and null for some system
    // accounts; both have to survive.
    let response = with_results();
    assert!(
        response.posts.iter().any(|p| p
            .name
            .as_deref()
            .is_none_or(|n| n.is_empty())),
        "fixture should include a blank or absent name"
    );
}

#[test]
fn posts_are_joined_to_their_topic_by_id() {
    // Search returns topics as a sibling array, not nested per post.
    let response = with_results();
    let post = &response.posts[0];
    let topic = response
        .topic_for(post)
        .expect("the topic for the first hit");
    assert_eq!(topic.id, post.topic_id);
    assert!(!topic.title.is_empty());
}

#[test]
fn a_hit_whose_topic_was_not_returned_resolves_to_none() {
    let mut response = with_results();
    response.topics.clear();
    assert!(response.topic_for(&response.posts[0]).is_none());
}

#[test]
fn an_empty_search_parses_and_reports_empty() {
    let response = without_results();
    assert!(response.posts.is_empty());
    assert!(response.topics.is_empty());
    assert!(response.is_empty());
    assert!(!response.has_more());
}

#[test]
fn paging_reads_more_full_page_results_not_more_posts() {
    // Core only writes @more_#{type} for header search, so more_posts is
    // null even on a full page of results. Reading it would report "no more
    // results" for every query. The fixture has exactly that combination.
    let response = with_results();
    assert_eq!(
        response.grouped_search_result.more_posts, None,
        "fixture should keep the null that makes this test meaningful"
    );
    assert_eq!(
        response.grouped_search_result.more_full_page_results,
        Some(true)
    );
    assert!(response.has_more());
}

#[test]
fn the_term_survives_the_round_trip() {
    let response = with_results();
    assert!(!response.grouped_search_result.term.is_empty());
}

#[test]
fn a_narrow_search_topic_parses_without_topic_list_fields() {
    // Search sends no views, posters, like_count or has_summary. Topic
    // requires all of them, which is why SearchTopic exists; this pins that
    // the narrow shape really does parse.
    let raw = r#"{
        "id": 42,
        "title": "A topic",
        "slug": "a-topic",
        "created_at": "2026-08-10T12:00:00.000Z",
        "closed": false,
        "archived": false
    }"#;
    let topic: SearchTopic = serde_json::from_str(raw).expect("narrow topic parses");
    assert_eq!(topic.id, 42);
    assert_eq!(topic.posts_count, 0, "absent count defaults rather than failing");
    assert!(topic.tags.is_empty());
    assert!(topic.category_id.is_none());
}

#[test]
fn a_server_side_error_is_readable() {
    // Core rejects a term under min_search_term_length; the reason arrives
    // in the grouped result rather than as a transport failure.
    let raw = r#"{
        "posts": [],
        "grouped_search_result": {
            "term": "a",
            "error": "Search term too short",
            "more_posts": null
        }
    }"#;
    let response: SearchResponse = serde_json::from_str(raw).expect("error response parses");
    assert_eq!(
        response.grouped_search_result.error.as_deref(),
        Some("Search term too short")
    );
    assert!(response.is_empty());
}
