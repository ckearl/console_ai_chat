// src/syntax_highlighter/mod.rs

pub mod language_factory;

use crate::syntax_highlighter::language_factory::language_def::LanguageDef;
use crate::syntax_highlighter::language_factory::language_factory::get_language;
use std::collections::HashSet;

pub fn highlight_code_blocks(content: &str) -> String {
    let mut highlighted_content = String::new();
    let mut in_code_block = false;
    let mut language_name = String::new();
    let mut code_block = String::new();

    for line in content.lines() {
        if line.starts_with("```") {
            if in_code_block {
                // End of code block
                highlighted_content.push_str(&highlight_code(&language_name, &code_block));

                highlighted_content.push_str("\x1b[32m");
                highlighted_content.push_str("\n ---- END OF CODE BLOCK ---- \n");
                highlighted_content.push_str("\x1b[0m");

                in_code_block = false;
                code_block.clear();
            } else {
                // Start of code block
                language_name = line.trim_start_matches("```").to_string();
                highlighted_content.push_str("\x1b[32m");
                highlighted_content.push_str("\n ---- START OF CODE BLOCK ---- \n");
                highlighted_content.push_str("\x1b[0m");
                in_code_block = true;
            }
        } else if in_code_block {
            code_block.push_str(line);
            code_block.push('\n');
        } else {
            // Format explanatory text
            let formatted_line = format_explanatory_line(line);
            highlighted_content.push_str(&formatted_line);
            highlighted_content.push('\n');
        }
    }

    if in_code_block {
        // Handle unclosed code block
        highlighted_content.push_str(&highlight_code(&language_name, &code_block));
    }

    highlighted_content
}

fn highlight_code(language_name: &str, code: &str) -> String {
    let language = match get_language(language_name) {
        Some(lang) => lang,
        None => panic!("Language not found"),
    };

    let mut highlighted_code = String::new();
    let lines: Vec<&str> = code.lines().collect();

    for line in lines {
        let comment_prefix = language.comment_prefix();
        let comment_start = line.find(comment_prefix);

        if let Some(index) = comment_start {
            let (code_part, comment_part) = line.split_at(index);
            highlighted_code.push_str(&highlight_code_part(language_name, code_part, &language));
            highlighted_code.push_str(&color_token(comment_part, "comment"));
            highlighted_code.push('\n');
        } else {
            highlighted_code.push_str(&highlight_code_part(language_name, line, &language));
            highlighted_code.push('\n');
        }
    }

    highlighted_code
}

fn format_explanatory_line(line: &str) -> String {
    let mut formatted = String::new();
    let mut in_backticks = false;
    let mut keyword = String::new();

    for c in line.chars() {
        if c == '`' {
            if in_backticks {
                // End of keyword
                formatted.push_str("\x1b[1;48;5;235m"); // Bold and dark gray background
                formatted.push_str(&keyword);
                formatted.push_str("\x1b[0m"); // Reset formatting
                keyword.clear();
            } else {
                // Start of keyword
                // Don't add anything to formatted string yet
            }
            in_backticks = !in_backticks;
        } else if in_backticks {
            keyword.push(c);
        } else {
            formatted.push(c);
        }
    }

    // Handle case where backticks aren't closed
    if !keyword.is_empty() {
        formatted.push('`');
        formatted.push_str(&keyword);
    }

    formatted
}

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
            current_token.push(ch);

            if ch == string_start {
                colored_line.push_str(&color_token(&current_token, "string"));
                current_token.clear();
                in_string = false;
            }
        } else if language.string_delimiters().contains(&ch) {
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
            current_token.push(ch);
        } else {
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
