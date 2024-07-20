// src/response_types/mod.rs

pub mod command;
pub mod short;

pub trait ResponseModifier {
    fn modify_prompt(&self, prompt: &str) -> String;
}
