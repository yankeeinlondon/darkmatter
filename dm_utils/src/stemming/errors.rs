use thiserror::Error;

#[derive(Error, Debug)]
pub enum StemmingError {
    #[error("No Stemmer algorithm found for the language: {0}")]
    NoStemmerForLang(String),
    /// Invalid ISO value for language
    #[error("ISO value invalid: {0}")]
    InvalidIso(String),

    #[error("Could not convert the passed in language -- {0} -- to Language struct before using that to lookup the stemmer algorithm.")]
    CanNotConvertToLanguage(String),
}
