// src/models/gpt.rs

use crate::models::AIModel;
use crate::syntax_highlighter::highlight_code;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::env;

pub struct GPT {
    conversation_history: Vec<serde_json::Value>,
}

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

        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY not set. Please check your .env file.")?;

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

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        let response_text = response.text().await?;
        let response_body: serde_json::Value = serde_json::from_str(&response_text)?;

        if let Some(content) = response_body["choices"][0]["message"]["content"].as_str() {
            self.conversation_history.push(json!({
                "role": "assistant",
                "content": content
            }));

            // Check for code blocks and apply syntax highlighting
            let highlighted_content = highlight_code_blocks(content);

            Ok(highlighted_content)
        } else {
            println!("Response structure: {:?}", response_text);
            Err("Failed to parse GPT's response".into())
        }
    }
}

fn highlight_code_blocks(content: &str) -> String {
    let mut highlighted_content = String::new();
    let mut in_code_block = false;
    let mut language_name = String::new();
    let mut code_block = String::new();

    for line in content.lines() {
        if line.starts_with("```") {
            if in_code_block {
                // End of code block
                highlighted_content.push_str(&highlight_code(&language_name, &code_block));
                highlighted_content.push_str("\n```");
                in_code_block = false;
                code_block.clear();
            } else {
                // Start of code block
                language_name = line.trim_start_matches("```").to_string();
                in_code_block = true;
            }
        } else if in_code_block {
            code_block.push_str(line);
            code_block.push('\n');
        } else {
            highlighted_content.push_str(line);
            highlighted_content.push('\n');
        }
    }

    if in_code_block {
        // Handle unclosed code block
        highlighted_content.push_str(&highlight_code(&language_name, &code_block));
    }

    highlighted_content
}
