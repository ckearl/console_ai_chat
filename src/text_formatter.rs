// src/text_formatter.rs

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
        format!("{}. ", Colour::Cyan.paint(&caps[1]))
    }).to_string()
}

pub fn create_header() -> String {
    Colour::Green.bold().paint("AI response:").to_string()
}

pub fn format_error(error: &str) -> String {
    Colour::Red.bold().paint(format!("Error: {}", error)).to_string()
}

pub fn print_formatted_response(response: &str, is_command_mode: bool) {
    let header = create_header();
    let formatted_response = format_response(response, is_command_mode);
    println!("{}\n{}", header, formatted_response);
}