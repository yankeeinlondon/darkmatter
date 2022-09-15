use thiserror::Error;

use crate::{
    darkmatter::errors::DarkmatterError,
    frontmatter::errors::FrontmatterError,
    hooks::errors::HookError,
    models::{html::HtmlError, markdown::MarkdownError, sfc::SfcError},
};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Issues encountered while processing darkmatter analytics.")]
    Darkmatter(#[from] DarkmatterError),
    #[error("Issues encountered while processing frontmatter.")]
    Frontmatter(#[from] FrontmatterError),
    #[error("Issues encountered while processing userland hooks.")]
    Hooks(#[from] HookError),
    #[error("Issues encountered while processing markdown")]
    Markdown(#[from] MarkdownError),
    #[error("Issues encountered while converting Markdown to HTML.")]
    HTML(#[from] HtmlError),
    #[error("Issues encountered while converting HTML to SFC format.")]
    SFC(#[from] SfcError),
}
