use syntect::Error as SyntectError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeBlockError {
    #[error("Failed to create a CodeBlock because the language passed in [{0}] is not a valid language!")]
    InvalidLanguage(String),
    #[error("Problem parsing the CodeBlock content into to HTML.")]
    Parsing(#[from] SyntectError),
}
