use crate::language_factory::language_def::LanguageDef;
use lazy_static::lazy_static;
use std::collections::HashSet;

pub struct SQL;

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let keywords = vec![
            "SELECT",
            "FROM",
            "WHERE",
            "INSERT",
            "INTO",
            "VALUES",
            "UPDATE",
            "SET",
            "DELETE",
            "CREATE",
            "TABLE",
            "ALTER",
            "DROP",
            "INDEX",
            "VIEW",
            "TRIGGER",
            "PROCEDURE",
            "FUNCTION",
            "JOIN",
            "INNER",
            "LEFT",
            "RIGHT",
            "FULL",
            "ON",
            "AS",
            "AND",
            "OR",
            "NOT",
            "NULL",
            "DISTINCT",
            "GROUP",
            "BY",
            "ORDER",
            "HAVING",
            "LIMIT",
            "OFFSET",
            "UNION",
            "ALL",
            "EXCEPT",
            "INTERSECT",
            "BETWEEN",
            "LIKE",
            "IN",
            "IS",
            "EXISTS",
            "CASE",
            "WHEN",
            "THEN",
            "ELSE",
            "END",
        ];
        keywords.into_iter().collect()
    };
    static ref OPERATORS: HashSet<&'static str> = {
        let operators = vec![
            "=", "<>", "!=", "<", ">", "<=", ">=", "+", "-", "*", "/", "%", "AND", "OR", "NOT",
            "IN", "LIKE", "BETWEEN", "IS", "NULL",
        ];
        operators.into_iter().collect()
    };
    static ref BUILT_INS: HashSet<&'static str> = {
        let built_ins = vec![
            "COUNT",
            "SUM",
            "AVG",
            "MIN",
            "MAX",
            "UPPER",
            "LOWER",
            "SUBSTRING",
            "TRIM",
            "LENGTH",
            "NOW",
            "CURDATE",
            "CURTIME",
            "DATE",
            "TIME",
            "DATETIME",
            "CHAR",
            "VARCHAR",
            "TEXT",
            "INT",
            "INTEGER",
            "FLOAT",
            "DOUBLE",
            "DECIMAL",
            "BOOLEAN",
            "BOOL",
        ];
        built_ins.into_iter().collect()
    };
    static ref LITERALS: HashSet<&'static str> = {
        let literals = vec!["TRUE", "FALSE", "NULL"];
        literals.into_iter().collect()
    };
    static ref TYPES: HashSet<&'static str> = {
        let types = vec![
            "CHAR", "VARCHAR", "TEXT", "INT", "INTEGER", "FLOAT", "DOUBLE", "DECIMAL", "BOOLEAN",
            "DATE", "TIME", "DATETIME",
        ];
        types.into_iter().collect()
    };
    static ref MODIFIERS: HashSet<&'static str> = {
        let modifiers = vec![
            "UNIQUE",
            "PRIMARY",
            "KEY",
            "FOREIGN",
            "REFERENCES",
            "AUTO_INCREMENT",
        ];
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
        let comments = vec!["--", "/*", "*/"];
        comments.into_iter().collect()
    };
}

impl LanguageDef for SQL {
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
        "--"
    }

    fn string_delimiters(&self) -> &'static [char] {
        &['\'', '"']
    }
}
