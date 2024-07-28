use crate::syntax_highlighter::language_factory::language_def::LanguageDef;
use lazy_static::lazy_static;
use std::collections::HashSet;

pub struct Swift;

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let keywords = vec![
            "class",
            "deinit",
            "enum",
            "extension",
            "func",
            "import",
            "init",
            "inout",
            "let",
            "operator",
            "precedencegroup",
            "protocol",
            "struct",
            "subscript",
            "typealias",
            "var",
            "break",
            "case",
            "continue",
            "default",
            "defer",
            "do",
            "else",
            "fallthrough",
            "for",
            "guard",
            "if",
            "in",
            "repeat",
            "return",
            "switch",
            "where",
            "while",
            "as",
            "Any",
            "catch",
            "false",
            "is",
            "nil",
            "rethrows",
            "super",
            "self",
            "Self",
            "throw",
            "throws",
            "true",
            "try",
            "associativity",
            "convenience",
            "dynamic",
            "didSet",
            "final",
            "get",
            "infix",
            "indirect",
            "lazy",
            "left",
            "mutating",
            "none",
            "nonmutating",
            "optional",
            "override",
            "postfix",
            "precedence",
            "prefix",
            "Protocol",
            "required",
            "right",
            "set",
            "Type",
            "unowned",
            "weak",
            "willSet",
        ];
        keywords.into_iter().collect()
    };
    static ref OPERATORS: HashSet<&'static str> = {
        let operators = vec![
            "+", "-", "*", "/", "%", "=", "==", "===", "!=", "!==", "<", ">", "<=", ">=", "&&",
            "||", "!", "?", ":", ".", "->", "++", "--", "&", "|", "^", "~", "<<", ">>", ">>>",
            "+=", "-=", "*=", "/=", "%=", "<<=", ">>=", ">>>=", "&=", "|=", "^=",
        ];
        operators.into_iter().collect()
    };
    static ref BUILT_INS: HashSet<&'static str> = {
        let built_ins = vec![
            "print",
            "abs",
            "min",
            "max",
            "round",
            "floor",
            "ceil",
            "Int",
            "Double",
            "String",
            "Array",
            "Dictionary",
            "Set",
            "Bool",
            "Optional",
            "Sequence",
            "Collection",
            "Comparable",
            "Equatable",
            "Hashable",
            "Codable",
            "Decodable",
            "Encodable",
            "Error",
            "Iterator",
            "Result",
            "Never",
        ];
        built_ins.into_iter().collect()
    };
    static ref LITERALS: HashSet<&'static str> = {
        let literals = vec!["true", "false", "nil"];
        literals.into_iter().collect()
    };
    static ref TYPES: HashSet<&'static str> = {
        let types = vec![
            "Int",
            "UInt",
            "Float",
            "Double",
            "Bool",
            "String",
            "Character",
            "Array",
            "Dictionary",
            "Set",
            "Optional",
            "Any",
            "AnyObject",
            "AnyClass",
            "Never",
            "Self",
            "Void",
        ];
        types.into_iter().collect()
    };
    static ref MODIFIERS: HashSet<&'static str> = {
        let modifiers = vec![
            "final",
            "lazy",
            "dynamic",
            "inout",
            "mutating",
            "nonmutating",
            "convenience",
            "override",
            "required",
            "static",
            "unowned",
            "weak",
        ];
        modifiers.into_iter().collect()
    };
    static ref ANNOTATIONS: HashSet<&'static str> = {
        let annotations = vec![];
        annotations.into_iter().collect()
    };
    static ref PREPROCESSOR_DIRECTIVES: HashSet<&'static str> = {
        let preprocessor_directives = vec![
            "#if",
            "#else",
            "#elseif",
            "#endif",
            "#available",
            "#warning",
            "#error",
        ];
        preprocessor_directives.into_iter().collect()
    };
    static ref COMMENTS: HashSet<&'static str> = {
        let comments = vec!["//", "/*", "*/"];
        comments.into_iter().collect()
    };
}

impl LanguageDef for Swift {
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
        &['\'', '"']
    }
}
