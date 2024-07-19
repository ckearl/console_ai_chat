// src/response_types/mod.rs

pub mod short;
pub mod command;

pub trait ResponseModifier {
    fn modify_prompt(&self, prompt: &str) -> String;
}