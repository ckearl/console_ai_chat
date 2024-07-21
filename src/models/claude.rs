// src/models/claude.rs

use crate::models::AIModel;
use async_trait::async_trait;
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::env;

pub struct Claude;

#[async_trait]
impl AIModel for Claude {
    async fn generate_response(&mut self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        dotenv().ok();
        
        let api_key = env::var("ANTHROPIC_API_KEY")
            .map_err(|_| "ANTHROPIC_API_KEY not set. Please check your .env file.")?;
        
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );
        
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let body = json!({
            "model": "claude-3-sonnet-20240229",
            "max_tokens": 1000,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        });

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        let response_text = response.text().await?;

        let response_body: serde_json::Value = serde_json::from_str(&response_text)?;

        if let Some(content) = response_body["content"][0]["text"].as_str() {
            Ok(content.to_string())
        } else {
            println!("Response structure: {:?}", response_text);
            Err("Failed to parse Claude's response".into())
        }
    }
}
