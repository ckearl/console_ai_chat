// src/models/gpt.rs

use crate::models::AIModel;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::env;

pub struct GPT;

#[async_trait]
impl AIModel for GPT {
    async fn generate_response(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok(); // Try to load .env file
        let api_key = env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY not set. Please check your .env file.")?;

        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key))?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let body = json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful assistant."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 1000
        });

        let response = client.post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        println!("Status: {}", response.status());
        
        let response_text = response.text().await?;
        println!("Response body: {}", response_text);

        let response_body: serde_json::Value = serde_json::from_str(&response_text)?;
        
        if let Some(content) = response_body["choices"][0]["message"]["content"].as_str() {
            println!("GPT response: {}", content);
            Ok(content.to_string())
        } else {
            println!("Response structure: {:?}", response_text);
            Err("Failed to parse GPT's response".into())
        }
    }
}