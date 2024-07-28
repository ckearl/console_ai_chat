use std::collections::HashSet;

pub trait LanguageDef {
    fn annotations(&self) -> &'static HashSet<&'static str>;
    fn built_ins(&self) -> &'static HashSet<&'static str>;
    fn comments(&self) -> &'static HashSet<&'static str>;
    fn comment_prefix(&self) -> &'static str;
    fn keywords(&self) -> &'static HashSet<&'static str>;
    fn literals(&self) -> &'static HashSet<&'static str>;
    fn modifiers(&self) -> &'static HashSet<&'static str>;
    fn operators(&self) -> &'static HashSet<&'static str>;
    fn preprocessor_directives(&self) -> &'static HashSet<&'static str>;
    fn string_delimiters(&self) -> &'static [char];
    fn types(&self) -> &'static HashSet<&'static str>;
}
