use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result <(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} \"your message in quotes\"", args[0]);
        std::process::exit(1);
    }

    let question = &args[1];
    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set");

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key))?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let body = json!({
        "model": "claude-3-sonnet-20240229",
        "max_tokens": 1000, 
        "messages": [
         {   "role": "user",
            "content": question}
        ]
    });

    let response = client.post("https://api.anthropic.com/v1/messages")
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    println!("Status: {}", response.status());

    let response_text = response.text().await?;
    println!("Response text: {}", response_text);

    let response_body: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(content) = response_body["content"][0]["text"].as_str() {
        println!("Claude's response:\n{}", content);
    } else {
        eprintln!("Failed to parse Claude's response");
        println!("Response structure: {:?}", response_body);
    }

    Ok(())
}