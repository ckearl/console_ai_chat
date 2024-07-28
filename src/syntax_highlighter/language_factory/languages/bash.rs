use crate::syntax_highlighter::language_factory::language_def::LanguageDef;
use lazy_static::lazy_static;
use std::collections::HashSet;

pub struct Bash;

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let keywords = vec![
            "if", "then", "else", "elif", "fi", "case", "esac", "for", "select", "while", "until",
            "do", "done", "in", "function", "time", "coproc",
        ];
        keywords.into_iter().collect()
    };
    static ref OPERATORS: HashSet<&'static str> = {
        let operators = vec![
            "+", "-", "*", "/", "%", "=", "==", "!=", "<", ">", "<=", ">=", "&&", "||", "!", "?",
            ":", ".", "->", "++", "--", "&", "|", "^", "~", "<<", ">>", "+=", "-=", "*=", "/=",
            "%=", "<<=", ">>=", "&=", "|=", "^=",
        ];
        operators.into_iter().collect()
    };
    static ref BUILT_INS: HashSet<&'static str> = {
        let built_ins = vec![
            "echo",
            "printf",
            "read",
            "cd",
            "pwd",
            "pushd",
            "popd",
            "dirs",
            "let",
            "eval",
            "exec",
            "set",
            "unset",
            "export",
            "alias",
            "unalias",
            "type",
            "typeset",
            "trap",
            "exit",
            "return",
            "source",
            "test",
            "[",
            "]",
            "true",
            "false",
            ":",
            "mapfile",
            "readarray",
            "readlink",
            "getopts",
            "shopt",
            "caller",
            "command",
            "compgen",
            "complete",
            "compopt",
            "declare",
            "help",
            "history",
            "jobs",
            "kill",
            "wait",
            "suspend",
            "disown",
            "builtin",
            "fc",
            "hash",
            "bind",
        ];
        built_ins.into_iter().collect()
    };
    static ref LITERALS: HashSet<&'static str> = {
        let literals = vec!["true", "false"];
        literals.into_iter().collect()
    };
    static ref TYPES: HashSet<&'static str> = {
        let types = vec![];
        types.into_iter().collect()
    };
    static ref MODIFIERS: HashSet<&'static str> = {
        let modifiers = vec![];
        modifiers.into_iter().collect()
    };
    static ref ANNOTATIONS: HashSet<&'static str> = {
        let annotations = vec![];
        annotations.into_iter().collect()
    };
    static ref PREPROCESSOR_DIRECTIVES: HashSet<&'static str> = {
        let preprocessor_directives = vec![];
        preprocessor_directives.into_iter().collect()
    };
    static ref COMMENTS: HashSet<&'static str> = {
        let comments = vec!["#"];
        comments.into_iter().collect()
    };
}

impl LanguageDef for Bash {
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
        "#"
    }

    fn string_delimiters(&self) -> &'static [char] {
        &['\'', '"']
    }
}
