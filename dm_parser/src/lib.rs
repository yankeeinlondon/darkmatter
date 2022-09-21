use config::{Config, Options};
// use content::toc::TocItem;
use crate::models::markdown::MarkdownContentRaw;
use error::ParserError;
use models::{frontmatter::Frontmatter, pipeline::Pipeline};
use tracing::instrument;

pub mod config;
pub mod error;
pub mod errors;
pub mod hooks;
pub mod models;

/// The key parsing/transform library which converts markdown into a
/// target output that the user specifies as part of their configuration.
#[instrument]
pub fn parse(route: &str, content: &str, options: &Options) -> Result<Pipeline, ParserError> {
    let config = Config::with_options(options);
    // Markdown and Frontmatter separated
    let (markdown, frontmatter) = Frontmatter::extract(
        &MarkdownContentRaw::new(content), //
        &config,
    )?;

    Ok(
        // kick off transformation pipeline
        Pipeline::new(
            &config,
            route, //
            route,
            &frontmatter,
            &None,
        )
        .h_fm_default_values()?
        .h_md_raw()?
        // make configured darkmatter available
        .process_darkmatter()?
        .h_fm_override_values()?
        // Parse to HTML (including parsing hook processing)
        .parse_html()?
        .h_code_block()?
        .highlight_code_blocks()?
        .h_code_block_formatted()?
        // Parse to SFC format (where configured to do so)
        .parse_sfc()?,
    )
}
