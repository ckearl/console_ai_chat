// src/syntax_highlighter/code_block_highlighter.rs

use super::token_highlighter::highlight_code;

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