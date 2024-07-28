use crate::syntax_highlighter::language_factory::language_def::LanguageDef;
use lazy_static::lazy_static;
use std::collections::HashSet;

pub struct HTML;

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let keywords = vec![
            "html",
            "head",
            "title",
            "base",
            "link",
            "meta",
            "style",
            "script",
            "noscript",
            "body",
            "section",
            "nav",
            "article",
            "aside",
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            "header",
            "footer",
            "address",
            "main",
            "p",
            "hr",
            "pre",
            "blockquote",
            "ol",
            "ul",
            "li",
            "dl",
            "dt",
            "dd",
            "figure",
            "figcaption",
            "div",
            "a",
            "em",
            "strong",
            "small",
            "s",
            "cite",
            "q",
            "dfn",
            "abbr",
            "ruby",
            "rt",
            "rp",
            "data",
            "time",
            "code",
            "var",
            "samp",
            "kbd",
            "sub",
            "sup",
            "i",
            "b",
            "u",
            "mark",
            "bdi",
            "bdo",
            "span",
            "br",
            "wbr",
            "ins",
            "del",
            "picture",
            "source",
            "img",
            "iframe",
            "embed",
            "object",
            "param",
            "video",
            "audio",
            "track",
            "map",
            "area",
            "table",
            "caption",
            "colgroup",
            "col",
            "tbody",
            "thead",
            "tfoot",
            "tr",
            "td",
            "th",
            "form",
            "label",
            "input",
            "button",
            "select",
            "datalist",
            "optgroup",
            "option",
            "textarea",
            "output",
            "progress",
            "meter",
            "fieldset",
            "legend",
            "details",
            "summary",
            "dialog",
            "menu",
            "menuitem",
            "template",
            "canvas",
            "svg",
            "math",
        ];
        keywords.into_iter().collect()
    };
    static ref OPERATORS: HashSet<&'static str> = {
        let operators = vec!["<", ">", "</", "/>"];
        operators.into_iter().collect()
    };
    static ref BUILT_INS: HashSet<&'static str> = {
        let built_ins = vec![];
        built_ins.into_iter().collect()
    };
    static ref LITERALS: HashSet<&'static str> = {
        let literals = vec![];
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
        let comments = vec!["<!--", "-->"];
        comments.into_iter().collect()
    };
}

impl LanguageDef for HTML {
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
        "<!--"
    }

    fn string_delimiters(&self) -> &'static [char] {
        &['\'', '"']
    }
}
