use serde::{Deserialize, Serialize};

use crate::{
    errors::{md_err::MarkdownError, parser_err::ParserError},
    models::{frontmatter::Frontmatter, markdown::MarkdownContentRaw},
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

impl<'a> Pipeline<'a, Initialize> {
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
    pub fn next(&mut self) -> Result<Pipeline<ParseRawMd>, ParserError> {
        // Pipeline::try_from(self)
        if self.markdown.is_none() {
            Err(ParserError::Markdown(
                MarkdownError::NotReadyForParseRawMdState,
            ))
        } else {
            let markdown = self.markdown.unwrap();
            let (markdown, frontmatter) = Frontmatter::extract(&markdown, &self.config)?;

            Ok(Pipeline {
                id: self.id.clone(),
                route: self.route,
                config: self.config,
                source: self.source,
                markdown,
                frontmatter,
                darkmatter: self.darkmatter,
                html: self.html,
                sfc: self.sfc,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    use super::*;

    #[test]
    fn p_init_can_add_md_str() {
        let p = Pipeline::new("foobar.md", Config::default());
        p.add_md_str("# Testing\nOne two three.");
        assert!(p.markdown.is_some());
    }

    // it's a pre-requisite that the markdown be set before calling "next()"
    fn p_init_can_move_to_next_stage_with_md() {
        let p = Pipeline::new("foobar.md", Config::default())
            .add_md_str("---\ntitle: this is a test\n---\n# Testing\nOne two three.");
        let p = p.next().unwrap();

        assert!(p.frontmatter.title.is_some());
        let title = p.frontmatter.title.unwrap();
        assert_eq!(title, "this is a test".to_string());
    }
}
