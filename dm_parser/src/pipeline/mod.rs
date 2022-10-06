use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    errors::{md_err::MarkdownError, parser_err::ParserError},
    models::frontmatter::Frontmatter,
    source::Source,
};

use self::stages::{a_initialize::Initialize, b_parse_raw_md::ParseRawMd};

pub mod stages;

pub trait Stage {
    type MD;
    type FM;
    type DM;
    type HTML;
    type SFC;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline<'a, S: Stage> {
    pub id: String,
    pub route: &'a str,
    pub config: &'a Config,
    pub source: &'a Source,
    pub markdown: &'a S::MD,
    pub frontmatter: &'a S::FM,
    pub darkmatter: &'a S::DM,
    pub html: &'a S::HTML,
    pub sfc: &'a S::SFC,
}

impl Pipeline<&'a mut ParseRawMd> {
    pub fn parse_raw_md<'a>(
        ingest: &'a Pipeline<Initialize>,
    ) -> Result<Pipeline<&'a ParseRawMd>, ParserError> {
        if ingest.markdown.is_none() {
            Err(ParserError::Markdown(
                MarkdownError::NotReadyForParseRawMdState,
            ))
        } else {
            let (markdown, frontmatter) =
                Frontmatter::extract(&ingest.markdown.unwrap(), &ingest.config)?;

            Ok(Self {
                id: ingest.id,
                route: ingest.route,
                source: ingest.source,
                config: ingest.config,
                markdown,
                frontmatter,
                darkmatter: false,
                html: false,
                sfc: false,
            })
        }
    }
}
