// src/models/gpt.rs

// this is a module that contains the implementation of the GPT model.

use crate::models::AIModel;
use crate::syntax_highlighter::highlight_code_blocks;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::env;

pub struct GPT {
    conversation_history: Vec<serde_json::Value>,
}

// Implement (Rust trait that defines behavior) the AIModel trait for GPT
impl GPT {
    pub fn new() -> Self {
        GPT {
            conversation_history: vec![json!({
                "role": "system",
                "content": "You are a helpful assistant."
            })],
        }
    }
}

#[async_trait]
impl AIModel for GPT {
    async fn generate_response(
        &mut self,
        prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        // Get the OpenAI API key from the environment variables
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY not set. Please check your .env file.")?;

        // Create a new reqwest client and set the headers
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        self.conversation_history.push(json!({
            "role": "user",
            "content": prompt
        }));

        let body = json!({
            "model": "gpt-3.5-turbo",
            "messages": self.conversation_history,
            "max_tokens": 1000
        });

        // Send a POST request to the OpenAI API
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        // Parse the response from the API, extract the content of the response using serde_json
        let response_text = response.text().await?;
        let response_body: serde_json::Value = serde_json::from_str(&response_text)?;

        if let Some(content) = response_body["choices"][0]["message"]["content"].as_str() {
            self.conversation_history.push(json!({
                "role": "assistant",
                "content": content
            }));

            // Check for code blocks, apply syntax highlighting, and keyword highlighting for supplemental explanations
            let highlighted_content = highlight_code_blocks(content);

            Ok(highlighted_content)
        } else {
            println!("Response structure: {:?}", response_text);
            Err("Failed to parse GPT's response".into())
        }
    }
}
