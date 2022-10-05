use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    errors::{md_err::MarkdownError, parser_err::ParserError},
    models::markdown::MarkdownContentRaw,
    pipeline::{Pipeline, Stage},
    source::Source,
};

use super::b_parse_raw_md::ParseRawMd;

/// The initial stage of the transformation pipeline where
/// all that is locked down is user configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Initialize;

impl Stage for Initialize {
    type MD = Option<MarkdownContentRaw>;
    type FM = bool;
    type DM = bool;
    type HTML = bool;
    type SFC = bool;
}

impl Pipeline<Initialize> {
    pub fn new(id: &str, config: Config) -> Pipeline<Initialize> {
        Pipeline {
            id: id.to_string(),
            route: id.to_string(),
            source: match id.starts_with("db::") {
                true => Source::Database,
                false => Source::File,
            },
            config,
            markdown: None,
            frontmatter: false,
            darkmatter: false,
            html: false,
            sfc: false,
        }
    }

    /// add raw markdown content from a string slice
    pub fn add_md_str(&self, md: &str) {
        self.markdown = Some(MarkdownContentRaw::new(md));
    }

    /// add raw markdown content from a file
    pub fn add_md_file(&self, file: &str) {
        todo!();
    }

    /// add raw markdown content from a database connection
    pub fn add_md_db(&self, db: &str) {
        todo!();
    }

    /// apply userland transforms from the raw_markdown hook
    pub fn h_raw_markdown(&self) -> Self {
        *self
    }

    /// after providing the raw markdown content you can move the
    /// next stage which involves parsing the "raw md" into both
    /// `Frontmatter` and `MarkdownContent`
    pub fn next_stage(mut self) -> Result<Pipeline<ParseRawMd>, ParserError> {
        if self.markdown.is_none() {
            Err(ParserError::Markdown(
                MarkdownError::NotReadyForParseRawMdState,
            ))
        } else {
            Pipeline::try_from(self)
        }
    }
}
