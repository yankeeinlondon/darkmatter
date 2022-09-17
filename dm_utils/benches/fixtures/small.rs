use thiserror::Error;

#[derive(Error, Debug)]
pub enum StemmingError {
    #[error("No Stemmer algorithm found for the language: {0}")]
    NoStemmerForLang(String),
    #[error("ISO value invalid: {0}")]
    InvalidIso(String),
}
