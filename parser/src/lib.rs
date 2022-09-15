use config::{Config, Options};
// use content::toc::TocItem;
use error::ParserError;

// use serde::{Deserialize, Serialize};
use models::{dm::Darkmatter, fm::Frontmatter, html::HtmlContent, md::MarkdownContent};
use serde::{Deserialize, Serialize};

use crate::{
    frontmatter::extract_frontmatter, hooks::fm_override_values::fm_override_values,
    md_parser::sfc::convert_to_sfc,
};

pub mod config;
pub mod darkmatter;
pub mod error;
pub mod frontmatter;
pub mod hooks;
pub mod md_parser;
pub mod models;

#[derive(Debug, Serialize, Deserialize)]
pub struct Parsed {
    frontmatter: Frontmatter,
    darkmatter: Darkmatter,
    html: HtmlContent,
    md: MarkdownContent,
    sfc: Option<String>,
    filename: String,
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

    // Markdown and Frontmatter
    let (markdown, frontmatter) = extract_frontmatter(route, content, &config);
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
    let html = HtmlContent::from_markdown(route, frontmatter, md);

    if options.output == OutputFormat::SFC {
        Ok(Parsed {
            frontmatter,
            darkmatter,
            html,
            md,
            sfc: Some(convert_to_sfc(&html, options)),
            filename: filename.to_string(),
        })
    } else {
        Ok(Parsed {
            frontmatter,
            darkmatter,
            html,
            md,
            sfc: None,
            filename: filename.to_string(),
        })
    }

    todo!();
}
