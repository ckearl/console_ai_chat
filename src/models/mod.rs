// src/models/mod.rs

pub mod claude;
pub mod gpt;

use async_trait::async_trait;

#[async_trait]
pub trait AIModel {
    async fn generate_response(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
}