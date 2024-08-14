// src/text_formatter.rs

// this is a module that contains functions for formatting text output. 
// It is only used to color very specific elements of the responses of the AI models, such as the response header, error messages, and ordered lists.

use ansi_term::Colour;
use regex::Regex;

pub fn format_response(response: &str, is_command_mode: bool) -> String {
    if is_command_mode {
        color_ordered_list(response)
    } else {
        response.to_string()
    }
}

fn color_ordered_list(text: &str) -> String {
    let re = Regex::new(r"(?m)^\s*(\d+)\.\s").unwrap();
    re.replace_all(text, |caps: &regex::Captures| {
        format!("{}. ", color_text(&caps[1], "cyan"))
    }).to_string()
}

pub fn create_header() -> String {
    color_text("AI response:", "green")
}

pub fn format_error(error: &str) -> String {
    color_text(&format!("Error: {}", error), "red")
}

pub fn print_formatted_response(response: &str, is_command_mode: bool) {
    let header = create_header();
    let formatted_response = format_response(response, is_command_mode);
    println!("\n{}\n{}", header, formatted_response);
}

pub fn color_text(text: &str, color: &str) -> String {
    match color {
        "red" => Colour::Red.bold().paint(text).to_string(),
        "green" => Colour::Green.bold().paint(text).to_string(),
        "yellow" => Colour::Yellow.bold().paint(text).to_string(),
        "cyan" => Colour::Cyan.bold().paint(text).to_string(),
        _ => text.to_string(),
    }
}