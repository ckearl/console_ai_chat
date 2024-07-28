// src/syntax_highlighter/language_factory.rs

use crate::syntax_highlighter::language_def::LanguageDef;
use crate::syntax_highlighter::languages::bash::Bash;
use crate::syntax_highlighter::languages::csharp::CSharp;
use crate::syntax_highlighter::languages::css::CSS;
use crate::syntax_highlighter::languages::html::HTML;
use crate::syntax_highlighter::languages::javascript::JavaScript;
use crate::syntax_highlighter::languages::python::Python;
use crate::syntax_highlighter::languages::rust::Rust;
use crate::syntax_highlighter::languages::sql::SQL;
use crate::syntax_highlighter::languages::swift::Swift;
use crate::syntax_highlighter::languages::typescript::TypeScript;

pub fn get_language(name: &str) -> Box<dyn LanguageDef> {
    match name.to_lowercase().as_str() {
        "python" => Box::new(Python),
        "bash" => Box::new(Bash),
        "csharp" => Box::new(CSharp),
        "css" => Box::new(CSS),
        "html" => Box::new(HTML),
        "javascript" => Box::new(JavaScript),
        "rust" => Box::new(Rust),
        "sql" => Box::new(SQL),
        "swift" => Box::new(Swift),
        "typescript" => Box::new(TypeScript),
        _ => panic!("Unsupported language: {}", name),
    }
}