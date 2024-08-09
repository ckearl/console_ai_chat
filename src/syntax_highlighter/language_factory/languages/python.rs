use crate::syntax_highlighter::language_factory::language_def::LanguageDef;
use lazy_static::lazy_static;
use std::collections::HashSet;

pub struct Python;

lazy_static! {
    // prettier-ignore
    static ref BUILT_INS: HashSet<&'static str> = {
        let built_ins = vec![
            "abs", "delattr", "hash", "memoryview", "set", "all", "dict", "help", "min",
            "setattr", "any", "dir", "hex", "next", "slice", "ascii", "divmod", "id", "object",
            "sorted", "bin", "enumerate", "input", "oct", "staticmethod", "bool", "eval", "int",
            "open", "str", "breakpoint", "exec", "isinstance", "ord", "sum", "bytearray", "filter",
            "issubclass", "pow", "super", "bytes", "float", "iter", "print", "tuple", "callable",
            "format", "len", "property", "type", "chr", "frozenset", "list", "range", "vars",
            "classmethod", "getattr", "locals", "repr", "zip", "compile", "globals", "map",
            "reversed", "__import__", "complex", "hasattr", "max", "round", "del",
        ];
        built_ins.into_iter().collect()
    };

    // prettier-ignore
    static ref KEYWORDS: HashSet<&'static str> = {
        let keywords = vec![
            "False", "await", "else", "import", "pass", "None", "break", "except", "in", "raise",
            "True", "class", "finally", "is", "return", "and", "continue", "for", "lambda", "try",
            "as", "def", "from", "nonlocal", "while", "assert", "del", "global", "not", "with",
            "async", "elif", "if", "or", "yield",
        ];
        keywords.into_iter().collect()
    };

    static ref OPERATORS: HashSet<&'static str> = {
        let operators = vec![
            "+", "-", "*", "**", "/", "//", "%", "@", "<<", ">>", "&", "|", "^", "~", ":", "=",
            "==", "!=", ">", "<", ">=", "<=", "->", "+=", "-=", "*=", "/=", "//=", "%=", "@=",
            "&=", "|=", "^=", ">>=", "<<=", "**=",
        ];
        operators.into_iter().collect()
    };

    static ref LITERALS: HashSet<&'static str> = {
        let literals = vec!["True", "False", "None"];
        literals.into_iter().collect()
    };

    // prettier-ignore
    static ref TYPES: HashSet<&'static str> = {
        let types = vec![
            "int", "float", "bool", "str", "list", "tuple", "dict",
            "set", "frozenset", "bytes", "bytearray", "complex",
        ];
        types.into_iter().collect()
    };

    static ref MODIFIERS: HashSet<&'static str> = {
        let modifiers = vec!["global", "nonlocal"];
        modifiers.into_iter().collect()
    };

    static ref ANNOTATIONS: HashSet<&'static str> = {
        let annotations = vec!["@staticmethod", "@classmethod", "@property"];
        annotations.into_iter().collect()
    };

    static ref PREPROCESSOR_DIRECTIVES: HashSet<&'static str> = {
        let preprocessor_directives = vec![
            "time", "threading", "os", "sys", "math", "random", "pandas", "numpy", 
            "matplotlib", "seaborn", "scipy", "sklearn", "tensorflow", "keras", "torch", 
            "cv2", "openai", "requests", "beautifulsoup4", "selenium", "flask", "django", 
            "fastapi", "pyqt5", "tkinter", "pygame", "pyautogui", "pyperclip", "pywhatkit", 
            "pyttsx3", "pyaudio", "pydub", "pytube", "pyqrcode", "pyzbar", "pytesseract"
        ];
        preprocessor_directives.into_iter().collect()
    };

    // unused for now but will be used for multi-line comments
    // static ref COMMENTS: HashSet<&'static str> = {
    //     let comments = vec!["#", "'''", "'''"];
    //     comments.into_iter().collect()
    // };
}

impl LanguageDef for Python {
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

    // unused for now but will be used for multi-line comments
    // fn comments(&self) -> &'static HashSet<&'static str> {
    //     &COMMENTS
    // }

    fn comment_prefix(&self) -> &'static str {
        "#"
    }

    fn string_delimiters(&self) -> &'static [char] {
        &['\'', '"']
    }
}
