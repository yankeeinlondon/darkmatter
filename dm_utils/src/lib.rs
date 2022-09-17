#[macro_use]
extern crate slugify;

pub mod code_highlighting;
pub mod detect_lang;
pub mod hasher;
pub mod stemming;
pub mod stop_words;
pub mod words;

pub use crate::detect_lang::{detect_language, LanguageOptions, LanguageScope};
pub use crate::hasher::hash;
pub use crate::stop_words::StopWords;
