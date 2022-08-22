use thiserror::Error;

#[derive(Error, Debug)]
pub enum WordsError {
    #[error("Issues loading file intended to source the Words utility")]
    UnableToLoadFile(#[from] std::io::Error),
}
