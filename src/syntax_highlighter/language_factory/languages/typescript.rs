use crate::syntax_highlighter::language_factory::language_def::LanguageDef;
use lazy_static::lazy_static;
use std::collections::HashSet;

pub struct TypeScript;

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let keywords = vec![
            "break",
            "case",
            "catch",
            "class",
            "const",
            "continue",
            "debugger",
            "default",
            "delete",
            "do",
            "else",
            "enum",
            "export",
            "extends",
            "false",
            "finally",
            "for",
            "function",
            "if",
            "import",
            "in",
            "instanceof",
            "new",
            "null",
            "return",
            "super",
            "switch",
            "this",
            "throw",
            "true",
            "try",
            "typeof",
            "var",
            "void",
            "while",
            "with",
            "yield",
            "let",
            "async",
            "await",
            "implements",
            "interface",
            "package",
            "private",
            "protected",
            "public",
            "static",
            "yield",
            "any",
            "boolean",
            "constructor",
            "declare",
            "get",
            "module",
            "require",
            "number",
            "set",
            "string",
            "symbol",
            "type",
            "from",
            "of",
        ];
        keywords.into_iter().collect()
    };

    static ref OPERATORS: HashSet<&'static str> = {
        let operators = vec![
            "+", "-", "*", "/", "%", "++", "--", "=", "+=", "-=", "*=", "/=", "%=", "==", "!=",
            "===", "!==", ">", "<", ">=", "<=", "&&", "||", "!", "?", ":", ".", "=>", "&", "|",
            "^", "~", "<<", ">>", ">>>", "&=", "|=", "^=", "<<=", ">>=", ">>>=",
        ];
        operators.into_iter().collect()
    };

    // prettier-ignore
    static ref BUILT_INS: HashSet<&'static str> = {
        let built_ins = vec![
            "Array", "Date", "eval", "function", "hasOwnProperty", "Infinity", "isFinite", "isNaN", "isPrototypeOf",
            "length", "Math", "NaN", "name", "Number", "Object", "prototype", "String", "toString", "undefined",
            "valueOf", "console", "window", "document", "Promise", "Set", "Map", "WeakSet", "WeakMap", "Symbol",
            "Proxy", "Reflect",
        ];
        built_ins.into_iter().collect()
    };

    static ref LITERALS: HashSet<&'static str> = {
        let literals = vec!["true", "false", "null", "undefined"];
        literals.into_iter().collect()
    };

    static ref TYPES: HashSet<&'static str> = {
        let types = vec![
            "number", "string", "boolean", "any", "void", "never", "unknown", "symbol", "bigint",
        ];
        types.into_iter().collect()
    };

    static ref MODIFIERS: HashSet<&'static str> = {
        let modifiers = vec!["private", "protected", "public", "readonly"];
        modifiers.into_iter().collect()
    };

    static ref ANNOTATIONS: HashSet<&'static str> = {
        let annotations = vec![
            "@Component", "@Directive", "@Injectable", "@Pipe", "@NgModule",
        ];
        annotations.into_iter().collect()
    };

    static ref PREPROCESSOR_DIRECTIVES: HashSet<&'static str> = {
        let preprocessor_directives = vec![];
        preprocessor_directives.into_iter().collect()
    };

    // unused for now but will be used for multi-line comments
    static ref COMMENTS: HashSet<&'static str> = {
        let comments = vec!["//", "/*", "*/"];
        comments.into_iter().collect()
    };
}

impl LanguageDef for TypeScript {
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
        &['\'', '"', '`']
    }
}
