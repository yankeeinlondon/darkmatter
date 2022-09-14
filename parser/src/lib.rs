use config::Config;
use darkmatter::dm::Darkmatter;
// use content::toc::TocItem;
use error::ParserError;
use frontmatter::frontmatter::{
    Frontmatter, FrontmatterConfig, FrontmatterEngineType, FrontmatterOptions,
};
use lingua::Language;
// use serde::{Deserialize, Serialize};
use serde::{Deserialize, Serialize};

use crate::{frontmatter::extract_frontmatter, md_parser::sfc::convert_to_sfc};

pub mod config;
pub mod darkmatter;
pub mod error;
pub mod frontmatter;
pub mod md_parser;

#[derive(Debug, Serialize, Deserialize)]
pub struct Parsed {
    frontmatter: Frontmatter,
    darkmatter: Darkmatter,
    html: String,
    md: String,
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
pub fn parse(filename: &str, content: &str, options: &Options) -> Result<Parsed, ParserError> {
    let config = Config::default();

    let (md, frontmatter) = extract_frontmatter(filename, content, options);
    let (html, darkmatter) = convert_to_html(filename, frontmatter, md);

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
}
