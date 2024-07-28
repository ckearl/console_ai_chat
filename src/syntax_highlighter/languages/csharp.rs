use crate::language_factory::language_def::LanguageDef;
use lazy_static::lazy_static;
use std::collections::HashSet;

pub struct CSharp;

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let keywords = vec!["abstract", "as", "base", "bool", "break", "byte", "case", "catch", "char", "checked", "class", "const", "continue", "decimal", "default", "delegate", "do", "double", "else", "enum", "event", "explicit", "extern", "false", "finally", "fixed", "float", "for", "foreach", "goto", "if", "implicit", "in", "int", "interface", "internal", "is", "lock", "long", "namespace", "new", "null", "object", "operator", "out", "override", "params", "private", "protected", "public", "readonly", "ref", "return", "sbyte", "sealed", "short", "sizeof", "stackalloc", "static", "string", "struct", "switch", "this", "throw", "true", "try", "typeof", "uint", "ulong", "unchecked", "unsafe", "ushort", "using", "virtual", "void", "volatile", "while"];
        keywords.into_iter().collect()
    };
    static ref OPERATORS: HashSet<&'static str> = {
        let operators = vec![
            "+", "-", "*", "/", "%", "=", "==", "!=", "<", ">", "<=", ">=", "&&", "||", "!", "?",
            ":", ".", "=>", "++", "--", "&", "|", "^", "~", "<<", ">>", "+=", "-=", "*=", "/=",
            "%=", "<<=", ">>=", "&=", "|=", "^=", "??", "?.", "::",
        ];
        operators.into_iter().collect()
    };
    static ref BUILT_INS: HashSet<&'static str> = {
        let built_ins = vec![
            "Console",
            "Math",
            "DateTime",
            "String",
            "Int32",
            "Int64",
            "Double",
            "Boolean",
            "Array",
            "List",
            "Dictionary",
            "HashSet",
            "Stack",
            "Queue",
            "Task",
            "Action",
            "Func",
        ];
        built_ins.into_iter().collect()
    };
    static ref LITERALS: HashSet<&'static str> = {
        let literals = vec!["true", "false", "null"];
        literals.into_iter().collect()
    };
    static ref TYPES: HashSet<&'static str> = {
        let types = vec!["int", "float", "double", "bool", "char", "string", "object"];
        types.into_iter().collect()
    };
    static ref MODIFIERS: HashSet<&'static str> = {
        let modifiers = vec![
            "public",
            "private",
            "protected",
            "internal",
            "static",
            "readonly",
            "const",
            "volatile",
            "sealed",
            "abstract",
            "virtual",
            "override",
            "new",
        ];
        modifiers.into_iter().collect()
    };
    static ref ANNOTATIONS: HashSet<&'static str> = {
        let annotations = vec![
            "Serializable",
            "Obsolete",
            "DllImport",
            "StructLayout",
            "FieldOffset",
        ];
        annotations.into_iter().collect()
    };
    static ref PREPROCESSOR_DIRECTIVES: HashSet<&'static str> = {
        let preprocessor_directives = vec![
            "#define",
            "#undef",
            "#if",
            "#else",
            "#elif",
            "#endif",
            "#line",
            "#error",
            "#warning",
            "#region",
            "#endregion",
            "#pragma",
            "#nullable",
        ];
        preprocessor_directives.into_iter().collect()
    };
    static ref COMMENTS: HashSet<&'static str> = {
        let comments = vec!["//", "/*", "*/"];
        comments.into_iter().collect()
    };
}

impl LanguageDef for CSharp {
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
