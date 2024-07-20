// src/response_types/short.rs

use super::ResponseModifier;

pub struct Short;

impl ResponseModifier for Short {
    fn modify_prompt(&self, prompt: &str) -> String {
        format!(
            "{} Please be sure to keep your response to a paragraph or less.",
            prompt
        )
    }
}
