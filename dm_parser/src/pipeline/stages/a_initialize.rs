use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    errors::parser_err::ParserError,
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
    pub fn new<'a>(id: &'a str, config: Config) -> Pipeline<Initialize> {
        let source = match id.starts_with("db::") {
            true => Source::Database,
            false => Source::File,
        };
        Pipeline {
            id: id.to_string(),
            route: id,
            source,
            config,
            markdown: None,
            frontmatter: false,
            darkmatter: false,
            html: false,
            sfc: false,
        }
    }

    pub fn load_content(&mut self) -> Result<&mut Pipeline<Initialize>, ParserError> {
        let file = self.id.to_owned();
        match self.source {
            Source::Database => {
                self.add_md_db(&file)?;
            }
            Source::File => {
                self.add_md_file(&file)?;
            }
        }

        Ok(self)
    }

    /// add raw markdown content from a string slice
    pub fn add_md_str(&mut self, md: &str) -> &mut Pipeline<Initialize> {
        self.markdown = Some(MarkdownContentRaw::new(md));

        self
    }

    /// add raw markdown content from a file
    pub fn add_md_file(&mut self, file: &str) -> Result<&mut Pipeline<Initialize>, ParserError> {
        todo!();
    }

    /// add raw markdown content from a database connection
    pub fn add_md_db(&mut self, db: &str) -> Result<&mut Pipeline<Initialize>, ParserError> {
        todo!();
    }

    /// apply userland transforms from the raw_markdown hook
    pub fn h_raw_markdown(&mut self) -> Result<&mut Pipeline<Initialize>, ParserError> {
        Ok(self)
    }

    /// after providing the raw markdown content you can move the
    /// next stage which involves parsing the "raw md" into both
    /// `Frontmatter` and `MarkdownContent`
    pub fn next_stage(self) -> Result<Pipeline<ParseRawMd>, ParserError> {
        Pipeline::try_from(self)
        // if self.markdown.is_none() {
        //     Err(ParserError::Markdown(
        //         MarkdownError::NotReadyForParseRawMdState,
        //     ))
        // } else {
        // }
    }
}
