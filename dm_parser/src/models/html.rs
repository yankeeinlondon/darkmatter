use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::Config;

use super::{frontmatter::Frontmatter, markdown::MarkdownContent};

#[derive(Error, Debug)]
pub enum HtmlError {}

/// a string which represents HTML content
#[derive(Debug, Serialize, Deserialize)]
pub struct HtmlContent(String);
impl HtmlContent {
    pub fn new(content: &str) -> Self {
        HtmlContent(content.to_string())
    }

    /// Transforms a Markdown input (along with relevant
    /// metadata) into an HTML output.
    pub fn from_markdown(
        _route: &str,
        _md: &MarkdownContent,
        _fm: &Frontmatter,
        _config: &Config,
    ) -> Self {
        todo!();
    }
}
