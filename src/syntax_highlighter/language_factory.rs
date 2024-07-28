// src/syntax_highlighter/language_factory.rs

use crate::language_factory::language_def::LanguageDef;
use crate::language_factory::languages::bash::Bash;
use crate::language_factory::languages::csharp::CSharp;
use crate::language_factory::languages::css::CSS;
use crate::language_factory::languages::html::HTML;
use crate::language_factory::languages::javascript::JavaScript;
use crate::language_factory::languages::python::Python;
use crate::language_factory::languages::rust::Rust;
use crate::language_factory::languages::sql::SQL;
use crate::language_factory::languages::swift::Swift;
use crate::language_factory::languages::typescript::TypeScript;

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