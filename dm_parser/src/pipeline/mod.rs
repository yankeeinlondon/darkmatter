use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    errors::parser_err::ParserError,
    models::{html::HtmlContent, markdown::MarkdownContentRaw},
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
pub struct Pipeline<S: Stage> {
    pub id: String,
    pub route: String,
    pub config: Config,
    pub source: Source,
    pub markdown: S::MD,
    pub frontmatter: S::FM,
    pub darkmatter: S::DM,
    pub html: S::HTML,
    pub sfc: S::SFC,
}
