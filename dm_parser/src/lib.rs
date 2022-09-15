use config::{Config, Options};
// use content::toc::TocItem;
use error::ParserError;

// use serde::{Deserialize, Serialize};
use models::{
    dm::Darkmatter, frontmatter::Frontmatter, html::HtmlContent, markdown::MarkdownContent,
    sfc::Sfc,
};
use serde::{Deserialize, Serialize};

use crate::{hooks::fm_override_values::fm_override_values, models::markdown::MarkdownContentRaw};

pub mod config;
pub mod darkmatter;
pub mod error;
pub mod frontmatter;
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

/// Transforms the raw text content into:
///
/// 1. **Frontmatter**: _traditional name/value pairs representing meta data and usable during rendering process_
/// 2. **Darkmatter**: _meta data derived from a combination of NLP models and structured parsing;
/// kept separate from Frontmatter to avoid namespace collisions_
/// 3. **HTML**: _resultant HTML after converting Markdown to HTML and applying pipeline transforms_
/// 4. **Markdown**: _the markdown text with the frontmatter extracted_
pub fn parse(route: &str, content: &str, options: &Options) -> Result<Parsed, ParserError> {
    let config = Config::with_options(options);
    // Raw Markdown content
    let markdown = MarkdownContentRaw::new(content);
    // Markdown and Frontmatter separated
    let (markdown, frontmatter) = markdown.extract_frontmatter(route, content, &config);
    // Darkmatter analysis
    let darkmatter = Darkmatter::analyze_content(
        markdown, //
        frontmatter,
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

    Ok(Parsed {
        id: route.to_string(),
        route: route.to_string(),
        frontmatter,
        darkmatter,
        html,
        markdown,
        sfc: Sfc::new(false),
    })
}
