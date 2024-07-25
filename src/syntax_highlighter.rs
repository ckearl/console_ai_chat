// src/syntax_highlighter.rs

use std::str::FromStr;
use syntect::easy::HighlightLines;
use syntect::highlighting::{
    Color, ScopeSelectors, Style, StyleModifier, Theme, ThemeItem, ThemeSet,
};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

fn create_custom_theme() -> Theme {
    let mut theme = ThemeSet::load_defaults().themes["base16-ocean.dark"].clone();

    // Define custom colors
    let comments = Color {
        r: 121,
        g: 121,
        b: 121,
        a: 255,
    };
    let white = Color {
        r: 214,
        g: 214,
        b: 214,
        a: 255,
    };
    let yellow = Color {
        r: 229,
        g: 181,
        b: 103,
        a: 255,
    };
    let green = Color {
        r: 180,
        g: 210,
        b: 115,
        a: 255,
    };
    let orange = Color {
        r: 232,
        g: 125,
        b: 62,
        a: 255,
    };
    let purple = Color {
        r: 158,
        g: 134,
        b: 200,
        a: 255,
    };
    let pink = Color {
        r: 176,
        g: 82,
        b: 121,
        a: 255,
    };
    let blue = Color {
        r: 108,
        g: 153,
        b: 187,
        a: 255,
    };

    // Apply custom colors to different scopes
    theme.settings.foreground = Some(white);
    theme.settings.background = Some(Color {
        r: 30,
        g: 30,
        b: 30,
        a: 255,
    }); // Dark background

    let scopes = &mut theme.scopes;
    scopes.push(ThemeItem {
        scope: ScopeSelectors::from_str("comment").unwrap(),
        style: StyleModifier {
            foreground: Some(comments),
            ..Default::default()
        },
    });
    scopes.push(ThemeItem {
        scope: ScopeSelectors::from_str("keyword").unwrap(),
        style: StyleModifier {
            foreground: Some(yellow),
            ..Default::default()
        },
    });
    scopes.push(ThemeItem {
        scope: ScopeSelectors::from_str("storage.type").unwrap(),
        style: StyleModifier {
            foreground: Some(green),
            ..Default::default()
        },
    });
    scopes.push(ThemeItem {
        scope: ScopeSelectors::from_str("string").unwrap(),
        style: StyleModifier {
            foreground: Some(pink),
            ..Default::default()
        },
    });
    scopes.push(ThemeItem {
        scope: ScopeSelectors::from_str("variable").unwrap(),
        style: StyleModifier {
            foreground: Some(blue),
            ..Default::default()
        },
    });
    scopes.push(ThemeItem {
        scope: ScopeSelectors::from_str("constant.numeric").unwrap(),
        style: StyleModifier {
            foreground: Some(purple),
            ..Default::default()
        },
    });
    scopes.push(ThemeItem {
        scope: ScopeSelectors::from_str("support.class").unwrap(),
        style: StyleModifier {
            foreground: Some(orange),
            ..Default::default()
        },
    });
    scopes.push(ThemeItem {
        scope: ScopeSelectors::from_str("punctuation").unwrap(),
        style: StyleModifier {
            foreground: Some(white),
            ..Default::default()
        },
    });

    theme
}

fn highlight_code(code: &str, language: &str, ps: &SyntaxSet, theme: &Theme) -> String {
    let syntax = ps
        .find_syntax_by_extension(language)
        .unwrap_or_else(|| ps.find_syntax_plain_text());
    let mut h = HighlightLines::new(syntax, theme);

    LinesWithEndings::from(code)
        .map(|line| {
            let ranges: Vec<(Style, &str)> = h.highlight_line(line, ps).unwrap();
            as_24_bit_terminal_escaped(&ranges[..], true)
        })
        .collect()
}

pub fn highlight_code_blocks(text: &str) -> String {
    let ps = SyntaxSet::load_defaults_newlines();
    let theme = create_custom_theme();
    let mut result = String::new();
    let mut in_code_block = false;
    let mut code_block = String::new();

    for line in text.lines() {
        if line.trim().starts_with("```") {
            if in_code_block {
                // End of code block
                let language = detect_language(&code_block);
                result.push_str(&highlight_code(&code_block, &language, &ps, &theme));
                result.push_str("```\n");
                in_code_block = false;
                code_block.clear();
            } else {
                // Start of code block
                in_code_block = true;
                result.push_str(line);
                result.push('\n');
            }
        } else if in_code_block {
            code_block.push_str(line);
            code_block.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    // Handle case where code block is not closed
    if in_code_block {
        let language = detect_language(&code_block);
        result.push_str(&highlight_code(&code_block, &language, &ps, &theme));
    }

    result.trim().to_string()
}

fn detect_language(code: &str) -> String {
    // Simple language detection based on common keywords or file extensions
    // This is a basic implementation and can be expanded
    if code.contains("fn ") || code.contains("let ") || code.contains("impl ") {
        "rust".to_string()
    } else if code.contains("def ") || code.contains("import ") {
        "python".to_string()
    } else if code.contains("function ") || code.contains("var ") || code.contains("const ") {
        "javascript".to_string()
    } else {
        "txt".to_string()  // fallback to plain text
    }
}