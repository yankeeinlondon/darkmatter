use std::{fs::read_to_string, path::Path};

use super::frontmatter::Frontmatter;
use crate::{
    config::Config,
    errors::{fm_err::FrontmatterError, md_err::MarkdownError},
};
use dm_utils::hash;
use serde::{Deserialize, Serialize};
use tracing::instrument;

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

    /// The raw content contained by this `MarkdownContentRaw`
    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// The hash value of the raw markdown content
    pub fn hash(&self) -> u64 {
        self.hash
    }

    /// Separates the raw markdown content into pure markdown and frontmatter
    pub fn parse(
        &self,
        config: &Config,
    ) -> Result<(MarkdownContent, Frontmatter), FrontmatterError> {
        Frontmatter::extract(self, config)
    }
}

/// a string which represents Markdown content (but no frontmatter)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarkdownContent {
    /// the hash generated on the raw content from the file/db
    /// before any mutations took place
    pub raw_hash: u64,
    /// A hash of _current state_ of the content
    pub hash: u64,
    content: String,
}

impl MarkdownContent {
    /// takes the **raw** Markdown along with the markdown content
    /// after the frontmatter has been extracted. This allows the
    /// creation of a struct which has hashes for both states but
    /// only the content for the _actual_ markdown.
    pub fn new(md: &MarkdownContentRaw, content: &str) -> Self {
        MarkdownContent {
            content: content.to_string(),
            raw_hash: md.hash(),
            hash: hash(&content, None),
        }
    }

    /// Load markdown content from a file and return a tuple with the **Markdown**
    /// and **Frontmatter**.
    #[instrument]
    pub fn from_file(file: &str, config: &Config) -> Result<(Self, Frontmatter), MarkdownError> {
        let path = Path::new(file);
        let raw = read_to_string(path).map_err(|e| MarkdownError::FileNotFound(e))?;
        let raw = MarkdownContentRaw::new(&raw);

        let (markdown, frontmatter) = Frontmatter::extract(&raw, &config)?;

        Ok((markdown, frontmatter))
    }

    /// gets the current state of the markdown content
    #[instrument]
    pub fn content(&self) -> String {
        self.content.clone()
    }

    /// mutates the state of the content and updates the hash
    #[instrument]
    pub fn mutate(&mut self, content: &str) {
        self.hash = hash(content, None);
        self.content = content.to_string();
    }
}
