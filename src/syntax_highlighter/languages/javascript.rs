use crate::language_factory::language_def::LanguageDef;
use lazy_static::lazy_static;
use std::collections::HashSet;

pub struct JavaScript;

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let keywords = vec![
            "const",
            "let",
            "var",
            "function",
            "async",
            "await",
            "if",
            "else",
            "for",
            "while",
            "do",
            "switch",
            "case",
            "break",
            "continue",
            "return",
            "try",
            "catch",
            "throw",
            "new",
            "typeof",
            "instanceof",
            "in",
            "of",
            "class",
            "extends",
            "super",
            "this",
            "import",
            "export",
            "default",
            "finally",
        ];
        keywords.iter().cloned().collect()
    };
    static ref OPERATORS: HashSet<&'static str> = {
        let operators = vec![
            "+", "-", "*", "/", "%", "=", "==", "===", "!=", "!==", "<", ">", "<=", ">=", "&&",
            "||", "!", "?", ":", ".", "=>", "++", "--", "&", "|", "^", "~", "<<", ">>", ">>>",
            "+=", "-=", "*=", "/=", "%=", "<<=", ">>=", ">>>=", "&=", "|=", "^=",
        ];
        operators.iter().cloned().collect()
    };
    static ref BUILT_INS: HashSet<&'static str> = {
        let built_ins = vec![
            "console", "Math", "Date", "Array", "Object", "String", "Number", "Boolean", "RegExp",
            "JSON", "Promise", "Set", "Map", "WeakSet", "WeakMap", "Symbol", "Proxy", "Reflect",
            "fetch", "log", "error", "warn", "info", "debug", "dir", "dirxml", "table", "trace",
            "group", "signal", "abort",
        ];
        built_ins.iter().cloned().collect()
    };
    static ref LITERALS: HashSet<&'static str> = {
        let literals = vec!["true", "false", "null", "undefined"];
        literals.iter().cloned().collect()
    };
    static ref TYPES: HashSet<&'static str> = {
        let types = vec!["number", "string", "boolean", "object", "symbol", "bigint"];
        types.iter().cloned().collect()
    };
    static ref MODIFIERS: HashSet<&'static str> = {
        let modifiers = vec![
            "public",
            "private",
            "protected",
            "static",
            "readonly",
            "const",
        ];
        modifiers.iter().cloned().collect()
    };
    static ref ANNOTATIONS: HashSet<&'static str> = {
        let annotations = vec!["@deprecated", "@override"];
        annotations.iter().cloned().collect()
    };
    static ref PREPROCESSOR_DIRECTIVES: HashSet<&'static str> = {
        let preprocessor_directives = vec![];
        preprocessor_directives.iter().cloned().collect()
    };
    static ref COMMENTS: HashSet<&'static str> = {
        let comments = vec!["//", "/*", "*/"];
        comments.iter().cloned().collect()
    };
}

impl LanguageDef for JavaScript {
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

    fn comments(&self) -> &'static HashSet<&'static str> {
        &COMMENTS
    }

    fn comment_prefix(&self) -> &'static str {
        "//"
    }

    fn string_delimiters(&self) -> &'static [char] {
        &['\'', '"', '`']
    }
}
