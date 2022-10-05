use thiserror::Error;

use super::fm_err::FrontmatterError;

#[derive(Error, Debug)]
pub enum MarkdownError {
    #[error("The markdown file could not be loaded!")]
    FileNotFound(#[from] std::io::Error),

    #[error(
        "Attempt to move to ParseRawMd stage before providing the raw MD to the initial state"
    )]
    NotReadyForParseRawMdState,

    #[error("Problems extracting frontmatter when loading markdown file")]
    FrontmatterParsing(#[from] FrontmatterError),
}
