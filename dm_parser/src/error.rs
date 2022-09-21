use thiserror::Error;

use crate::{
    errors::fm_err::FrontmatterError,
    hooks::errors::HookError,
    models::{
        darkmatter::DarkmatterError, html::HtmlError, markdown::MarkdownError, sfc::SfcError,
    },
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
