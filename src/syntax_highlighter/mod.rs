// src/syntax_highlighter/mod.rs

pub mod language_factory;
mod code_block_highlighter;
mod token_highlighter;

pub use code_block_highlighter::highlight_code_blocks;