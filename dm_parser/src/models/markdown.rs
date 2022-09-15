use super::frontmatter::Frontmatter;
use crate::config::Config;
use dm_utils::hash;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MarkdownError {}

/// Markdown content that may also contain frontmatter at the top
#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownContentRaw {
    hash: u64,
    content: String,
}

impl MarkdownContentRaw {
    pub fn new(content: &str) -> Self {
        // let content = content.to_string().as_str();
        MarkdownContentRaw {
            content: content.to_string(),
            hash: hash(&content, None),
        }
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    /// This function separates Frontmatter from the Markdown and returns both
    /// as separate entities.
    pub fn extract_frontmatter(
        &mut self,
        content: &str,
        config: &Config,
    ) -> Result<(MarkdownContent, Frontmatter), MarkdownError> {
        // get parser
        let mut matter = Matter::<YAML>::new();
        // configure parser if needed
        if let Some(delimiter) = &config.features.frontmatter.delimiter {
            matter.delimiter = delimiter.clone();
        }

        // match config.features.frontmatter.excerpt_strategy {
        //     ExcerptStrategy::Delimited(delimiter) => {
        //         matter.excerpt_delimiter = delimiter;
        //     }
        //     default => {}
        // }

        let fm = matter.parse(content);
        // mutate content state to reflect the isolated markdown content
        self.content = fm.content;
        // convert type to MarkdownContent and Frontmatter
        let content = MarkdownContent::new(self);
        let fm = match fm.data {
            Some(data) => Frontmatter::from_matter(data),
            None => Frontmatter::new(),
        };

        Ok((content, fm))
    }
}

/// a string which represents Markdown content (but no frontmatter)
#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownContent {
    /// the hash generated on the raw content from the file/db
    /// before any mutations took place
    pub raw_hash: u64,
    /// A hash of _current state_ of the content
    pub hash: u64,
    content: String,
}
impl MarkdownContent {
    pub fn new(md: &MarkdownContentRaw) -> Self {
        let content = md.content();
        MarkdownContent {
            content: content.to_string(),
            raw_hash: md.hash(),
            hash: hash(&content, None),
        }
    }

    /// gets the current state of the markdown content
    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// mutates the state of the content and updates the hash
    pub fn mutate(&mut self, content: &str) {
        self.hash = hash(content, None);
        self.content = content.to_string();
    }
}
