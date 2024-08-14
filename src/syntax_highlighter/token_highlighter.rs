// src/syntax_highlighter/token_highlighter.rs

use std::collections::HashSet;
use crate::syntax_highlighter::language_factory::language_def::LanguageDef;
use crate::syntax_highlighter::language_factory::language_factory::get_language;

pub fn highlight_code(language_name: &str, code: &str) -> String {
    let language = match get_language(language_name) {
        Some(lang) => lang,
        None => panic!("Language not found"),
    };

    let mut highlighted_code = String::new();
    let lines: Vec<&str> = code.lines().collect();

    // Check if the line has a comment and highlight it separately
    for line in lines {
        let comment_prefix = language.comment_prefix();
        let comment_start = line.find(comment_prefix);

        // If there's a comment, highlight the code part and the comment part separately. Ensures that comments are colored appropriately
        if let Some(index) = comment_start {
            let (code_part, comment_part) = line.split_at(index);
            highlighted_code.push_str(&highlight_code_part(language_name, code_part, &language));
            highlighted_code.push_str(&color_token(comment_part, "comment"));
            highlighted_code.push('\n');
        } else {
            // If there's no comment, highlight the whole line
            highlighted_code.push_str(&highlight_code_part(language_name, line, &language));
            highlighted_code.push('\n');
        }
    }

    highlighted_code
}

// This function handles the highlighting of a single line or part of a line that isn't a comment
fn highlight_code_part(
    language_name: &str,
    code_part: &str,
    language: &Box<dyn LanguageDef>,
) -> String {
    let mut colored_line = String::new();
    let mut in_string = false;
    let mut string_start = ' ';
    let mut current_token = String::new();

    for ch in code_part.chars() {
        if in_string {
            // If we're inside a string, keep adding characters until we find the closing delimiter
            current_token.push(ch);

            if ch == string_start {
                colored_line.push_str(&color_token(&current_token, "string"));
                current_token.clear();
                in_string = false;
            }
        } else if language.string_delimiters().contains(&ch) {
            // If we find a string delimiter, start a new string token
            if !current_token.is_empty() {
                colored_line.push_str(&color_token(
                    &current_token,
                    &categorize_token(
                        &current_token,
                        language.keywords(),
                        language.operators(),
                        language.built_ins(),
                        language.literals(),
                        language.types(),
                        language.modifiers(),
                        language.annotations(),
                        language.preprocessor_directives(),
                    ),
                ));
                current_token.clear();
            }

            current_token.push(ch);
            in_string = true;
            string_start = ch;
        } else if ch.is_alphanumeric() || ch == '_' || (language_name == "javascript" && ch == '$')
        {
            // If the character is alphanumeric or an underscore, add it to the current token
            current_token.push(ch);
        } else {
            // If we encounter any other character, it's the end boundary
            if !current_token.is_empty() {
                // now with a complete token, categorize and color it
                colored_line.push_str(&color_token(
                    &current_token,
                    &categorize_token(
                        &current_token,
                        language.keywords(),
                        language.operators(),
                        language.built_ins(),
                        language.literals(),
                        language.types(),
                        language.modifiers(),
                        language.annotations(),
                        language.preprocessor_directives(),
                    ),
                ));
                current_token.clear();
            }

            // Handle the current character (whitespace, operator, etc.)
            if ch.is_whitespace() {
                colored_line.push(ch);
            } else {
                colored_line.push_str(&color_token(
                    &ch.to_string(),
                    &categorize_token(
                        &ch.to_string(),
                        language.keywords(),
                        language.operators(),
                        language.built_ins(),
                        language.literals(),
                        language.types(),
                        language.modifiers(),
                        language.annotations(),
                        language.preprocessor_directives(),
                    ),
                ));
            }
        }
    }

    // Handle any remaining token at the end of the line
    if !current_token.is_empty() {
        colored_line.push_str(&color_token(
            &current_token,
            &categorize_token(
                &current_token,
                language.keywords(),
                language.operators(),
                language.built_ins(),
                language.literals(),
                language.types(),
                language.modifiers(),
                language.annotations(),
                language.preprocessor_directives(),
            ),
        ));
    }

    colored_line
}

fn categorize_token(
    token: &str,
    keywords: &HashSet<&str>,
    operators: &HashSet<&str>,
    built_ins: &HashSet<&str>,
    literals: &HashSet<&str>,
    types: &HashSet<&str>,
    modifiers: &HashSet<&str>,
    annotations: &HashSet<&str>,
    preprocessor_directives: &HashSet<&str>,
) -> String {
    // Check the token against various categories defined by the language
    if keywords.contains(token) {
        "keyword".to_string()
    } else if operators.contains(token) {
        "operator".to_string()
    } else if built_ins.contains(token) {
        "built_in".to_string()
    } else if literals.contains(token) {
        "literal".to_string()
    } else if types.contains(token) {
        "type".to_string()
    } else if modifiers.contains(token) {
        "modifier".to_string()
    } else if annotations.contains(token) {
        "annotation".to_string()
    } else if preprocessor_directives.contains(token) {
        "preprocessor_directive".to_string()
    } else if token.parse::<f64>().is_ok()
        || token == "true"
        || token == "false"
        || token == "null"
        || token == "undefined"
        || token == "None"
        || token == "True"
        || token == "False"
    {
        "literal".to_string()
    } else if token
        .chars()
        .next()
        .map_or(false, |c| c.is_alphabetic() || c == '_' || c == '$')
    {
        if token.chars().next().unwrap().is_uppercase() {
            "method_function".to_string()
        } else {
            "variable".to_string()
        }
    } else {
        // "_", Default category for anything not specifically categorized
        "default".to_string()
    }
}

fn color_token(token: &str, category: &str) -> String {
    let color_code = match category {
        "comment" => "\x1b[38;2;121;121;121m",         // #797979
        "variable" => "\x1b[38;2;214;214;214m",        // #d6d6d6
        "literal" => "\x1b[38;2;229;181;103m",         // #e5b567
        "string" => "\x1b[38;2;229;181;103m",          // #e5b567
        "method_function" => "\x1b[38;2;180;210;115m", // #b4d273
        "built_in" => "\x1b[38;2;232;125;62m",         // #e87d3e
        "modifier" => "\x1b[38;2;232;125;62m",         // #e87d3e
        "annotation" => "\x1b[38;2;158;134;200m",      // #9e86c8
        "preprocessor_directive" => "\x1b[38;2;158;134;200m", // #9e86c8
        "keyword" => "\x1b[38;2;176;82;121m",          // #b05279
        "type" => "\x1b[38;2;176;82;121m",             // #b05279
        "operator" => "\x1b[38;2;108;153;187m",        // #6c99bb
        _ => "\x1b[0m",                                // Default (Reset)
    };
    format!("{}{}\x1b[0m", color_code, token)
}