use super::frontmatter::Frontmatter;
use crate::config::Config;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use dm_utils::hash;

#[derive(Error, Debug)]
pub enum MarkdownError {}

/// Markdown content that may also contain frontmatter at the top
#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownContentRaw {
    content: String,
    hash: String
};

impl MarkdownContentRaw {
    pub fn new(content: &str) -> Self {
        let content = content.to_string();
        MarkdownContentRaw{
            content,
            hash: hash()
        }
    }

    /// This function separates Frontmatter from the Markdown and returns both
    /// after applying all appropriate hook/event transforms on them.
    ///
    /// Event Hooks:
    /// - `fm_default_values`
    /// - `md_raw_content`
    pub fn extract_frontmatter(
        &self,
        id: &str,
        content: &str,
        config: &Config,
    ) -> Result<(MarkdownContent, Frontmatter), MarkdownError> {
        let matter = Matter::<YAML>::new();
        let fm = matter.parse(content);
        let content = MarkdownContent::new(&fm.content);
        let fm = match fm.data {
            Some(data) => Frontmatter::from_matter(data),
            None => Frontmatter::new(),
        };

        Ok((content, fm))
    }
}

/// a string which represents Markdown content (but no frontmatter)
#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownContent(String);
impl MarkdownContent {
    pub fn new(content: &str) -> Self {
        MarkdownContent(content.to_string())
    }
}
