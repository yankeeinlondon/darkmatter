use thiserror::Error;

#[derive(Error, Debug)]
pub enum StemmingError {
    #[error("no-stemmer-for-lang: {0}")]
    NoStemmerForLang(String),
    #[error("ISO value invalid: {0}")]
    InvalidIso(String),
}
