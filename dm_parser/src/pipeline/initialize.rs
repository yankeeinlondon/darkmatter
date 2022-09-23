use crate::{
    config::Config, errors::parser_err::ParserError, models::markdown::MarkdownContentRaw,
    source::Source,
};
use serde::{Deserialize, Serialize};

use super::{parse_raw_md::ParseRawMd, Pipeline, Stage};

#[derive(Debug, Serialize, Deserialize)]
pub struct Initialize(Pipeline<Self>);

impl Initialize {
    pub fn new(id: &str, config: Config) -> Self {
        let source = if id.starts_with("db::") {
            Source::Database
        } else {
            Source::File
        };

        Initialize(Pipeline {
            config,
            id: id.to_string(),
            route: id.to_string(),
            source,
            markdown: None,
            frontmatter: false,
            darkmatter: false,
            html: false,
            sfc: false,
        })
    }

    /// Convert the raw markdown content into discrete `Frontmatter` and
    /// `Markdown` content.
    pub fn parse_raw_md(&mut self, raw_md: &str) -> Result<ParseRawMd, ParserError> {
        self.0.markdown = Some(MarkdownContentRaw::new(raw_md));

        Ok(ParseRawMd::try_from(self.0)?)
    }
}

impl Stage for Initialize {
    type MD = Option<MarkdownContentRaw>;
    type FM = bool;
    type DM = bool;
    type HTML = bool;
    type SFC = bool;
}
