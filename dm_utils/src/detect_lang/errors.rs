use thiserror::Error;

#[derive(Error, Debug)]
pub enum LangDetectionError {
    #[error("invalid-language-arity: {0}")]
    LanguageArity(String),
    #[error("ISO value invalid: {0}")]
    InvalidIso(String),
}
