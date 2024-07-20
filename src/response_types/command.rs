// src/response_types/command.rs

use super::ResponseModifier;

pub struct Command;

impl ResponseModifier for Command {
    fn modify_prompt(&self, prompt: &str) -> String {
        format!(
            "{} Please respond with just the commands to execute the task in an ordered list.",
            prompt
        )
    }
}
