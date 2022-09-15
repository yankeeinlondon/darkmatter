use thiserror::Error;

#[derive(Error, Debug)]
pub enum StopWordError {
    #[error("no-stop-words: {0}")]
    NoStopWords(String),
}
