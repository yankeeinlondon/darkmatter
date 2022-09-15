use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::Config;

use super::md::MarkdownContent;

#[derive(Error, Debug)]
pub enum HtmlError {}

/// a string which represents HTML content
#[derive(Debug, Serialize, Deserialize)]
pub struct HtmlContent(String);
impl HtmlContent {
    pub fn new(content: &str) -> Self {
        HtmlContent(content.to_string())
    }

    pub fn from_markdown(md: &MarkdownContent, config: &Config) -> Self {
        todo!();
    }
}
