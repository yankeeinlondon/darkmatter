use thiserror::Error;

use super::fm_err::FrontmatterError;

#[derive(Error, Debug)]
pub enum MarkdownError {
    #[error("The markdown file could not be loaded!")]
    FileNotFound(#[from] std::io::Error),

    #[error("Problems extracting frontmatter when loading markdown file")]
    FrontmatterParsing(#[from] FrontmatterError),
}
