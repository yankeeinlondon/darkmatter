use config::{Config, Options};
// use content::toc::TocItem;
use error::ParserError;

use gray_matter::engine::Engine;
use hooks::fm_default_values::fm_default_values;
use models::{
    darkmatter::Darkmatter, frontmatter::Frontmatter, html::HtmlContent, markdown::MarkdownContent,
    sfc::Sfc,
};
use serde::{Deserialize, Serialize};

use crate::{hooks::fm_override_values::fm_override_values, models::markdown::MarkdownContentRaw};

pub mod config;
pub mod error;
pub mod hooks;
pub mod models;

#[derive(Debug, Serialize, Deserialize)]
pub struct Parsed {
    id: String,
    route: String,
    frontmatter: Frontmatter,
    darkmatter: Darkmatter,
    html: HtmlContent,
    markdown: MarkdownContent,
    sfc: Sfc,
}

/// The key parsing/transform library which converts markdown into a
/// target output that the user specifies as part of their configuration.
pub fn parse<E: Engine>(
    route: &str,
    content: &str,
    options: &Options<E>,
) -> Result<Parsed, ParserError> {
    let config = Config::with_options(options);
    // Markdown and Frontmatter separated
    let (markdown, frontmatter) = Frontmatter::extract(
        &MarkdownContentRaw::new(content), //
        &config,
    );

    // Event Hook
    let frontmatter = fm_default_values(
        route, //
        frontmatter,
        &config,
    )?;
    // Darkmatter analysis
    let darkmatter = Darkmatter::analyze_content(
        &markdown, //
        &frontmatter,
        &config,
    );
    // Event Hooks
    let frontmatter = fm_override_values(
        route, //
        frontmatter,
        &darkmatter,
        &config,
    )
    .unwrap();

    // Parse Markdown to HTML
    let html = HtmlContent::from_markdown(
        route, //
        &markdown,
        &frontmatter,
        &config,
    );

    let sfc = Sfc::new(&config);

    Ok(Parsed {
        id: route.to_string(),
        route: route.to_string(),
        frontmatter,
        darkmatter,
        html,
        markdown,
        sfc,
    })
}
