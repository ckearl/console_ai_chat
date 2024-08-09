use crate::syntax_highlighter::language_factory::language_def::LanguageDef;
use lazy_static::lazy_static;
use std::collections::HashSet;

pub struct Rust;

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let keywords = vec![
            "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
            "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
            "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
            "unsafe", "use", "where", "while", "async", "await", "dyn",
        ];
        keywords.into_iter().collect()
    };
    
    static ref OPERATORS: HashSet<&'static str> = {
        let operators = vec![
            "+", "-", "*", "/", "%", "&", "|", "^", "!", "=", "==", "!=", "<", ">", "<=", ">=",
            "&&", "||", "<<", ">>", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=",
            "->", "?", "::", "=>", "..", "..=", ",", ";", ":",
        ];
        operators.into_iter().collect()
    };
    
    // prettier-ignore
    static ref BUILT_INS: HashSet<&'static str> = {
        let built_ins = vec![
            "Copy", "Send", "Sized", "Sync", "Drop", "Fn", "FnMut", "FnOnce", "Box", "ToOwned",
            "Clone", "PartialEq", "PartialOrd", "Eq", "Ord", "AsRef", "AsMut", "Into", "From",
            "Default", "Iterator", "Extend", "IntoIterator", "DoubleEndedIterator",
            "ExactSizeIterator", "Option", "Result", "Some", "None", "Ok", "Err", "String", "Vec",
        ];
        built_ins.into_iter().collect()
    };
    
    static ref LITERALS: HashSet<&'static str> = {
        let literals = vec!["true", "false"];
        literals.into_iter().collect()
    };
    
    static ref TYPES: HashSet<&'static str> = {
        let types = vec![
            "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64",
            "bool", "char", "str", "String", "Vec", "Option", "Result", "Box",
        ];
        types.into_iter().collect()
    };
    
    static ref MODIFIERS: HashSet<&'static str> = {
        let modifiers = vec!["mut", "ref"];
        modifiers.into_iter().collect()
    };
    
    static ref ANNOTATIONS: HashSet<&'static str> = {
        let annotations = vec!["#[derive]", "#[allow]", "#[warn]", "#[deny]", "#[forbid]"];
        annotations.into_iter().collect()
    };
    
    static ref PREPROCESSOR_DIRECTIVES: HashSet<&'static str> = {
        let preprocessor_directives = vec![];
        preprocessor_directives.into_iter().collect()
    };

    // unused for now but will be used for multi-line comments
    // static ref COMMENTS: HashSet<&'static str> = {
    //     let comments = vec!["//", "/*", "*/"];
    //     comments.into_iter().collect()
    // };
}

impl LanguageDef for Rust {
    fn keywords(&self) -> &'static HashSet<&'static str> {
        &KEYWORDS
    }

    fn operators(&self) -> &'static HashSet<&'static str> {
        &OPERATORS
    }

    fn built_ins(&self) -> &'static HashSet<&'static str> {
        &BUILT_INS
    }

    fn literals(&self) -> &'static HashSet<&'static str> {
        &LITERALS
    }

    fn types(&self) -> &'static HashSet<&'static str> {
        &TYPES
    }

    fn modifiers(&self) -> &'static HashSet<&'static str> {
        &MODIFIERS
    }

    fn annotations(&self) -> &'static HashSet<&'static str> {
        &ANNOTATIONS
    }

    fn preprocessor_directives(&self) -> &'static HashSet<&'static str> {
        &PREPROCESSOR_DIRECTIVES
    }
    // unused for now but will be used for multi-line comments
    // fn comments(&self) -> &'static HashSet<&'static str> {
    //     &COMMENTS
    // }

    fn comment_prefix(&self) -> &'static str {
        "//"
    }

    fn string_delimiters(&self) -> &'static [char] {
        &['\'', '"']
    }
}
