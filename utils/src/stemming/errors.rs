use thiserror::Error;

#[derive(Error, Debug)]
pub enum StemmingErrors {
    #[error("invalid-language-arity: {0}")]
    LanguageArity(String),
    #[error("ISO value invalid: {0}")]
    InvalidIso(String),
}
