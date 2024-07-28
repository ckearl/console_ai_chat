// src/syntax_highlighter/language_factory/language_factory.rs

use crate::syntax_highlighter::language_factory::language_def::LanguageDef;
use crate::syntax_highlighter::language_factory::languages::bash::Bash;
use crate::syntax_highlighter::language_factory::languages::csharp::CSharp;
use crate::syntax_highlighter::language_factory::languages::css::CSS;
use crate::syntax_highlighter::language_factory::languages::html::HTML;
use crate::syntax_highlighter::language_factory::languages::javascript::JavaScript;
use crate::syntax_highlighter::language_factory::languages::python::Python;
use crate::syntax_highlighter::language_factory::languages::rust::Rust;
use crate::syntax_highlighter::language_factory::languages::sql::SQL;
use crate::syntax_highlighter::language_factory::languages::swift::Swift;
use crate::syntax_highlighter::language_factory::languages::typescript::TypeScript;

pub fn get_language(name: &str) -> Option<Box<dyn LanguageDef>> {
    match name.to_lowercase().as_str() {
        "python" => Some(Box::new(Python)),
        "bash" => Some(Box::new(Bash)),
        "csharp" => Some(Box::new(CSharp)),
        "css" => Some(Box::new(CSS)),
        "html" => Some(Box::new(HTML)),
        "javascript" => Some(Box::new(JavaScript)),
        "rust" => Some(Box::new(Rust)),
        "sql" => Some(Box::new(SQL)),
        "swift" => Some(Box::new(Swift)),
        "typescript" => Some(Box::new(TypeScript)),
        _ => None,
    }
}
