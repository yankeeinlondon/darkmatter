use dm_utils::hash;
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
