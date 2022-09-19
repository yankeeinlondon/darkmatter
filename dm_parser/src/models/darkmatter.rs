use super::{frontmatter::Frontmatter, markdown::MarkdownContent};
use crate::config::Config;
use gray_matter::engine::Engine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum DarkmatterError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Darkmatter {
    /// The uniqueness hash for full content of the page
    hash: String,
    /// A hash of the table-of-contents which indicates whether
    /// the structure of the document has changed
    structure_hash: String,
    /// The maximum depth/nesting level the page goes to
    max_nesting: usize,
    /// Results from language detection
    // language: Language,
    /// The estimated time to read (in minutes)
    time_to_read: Option<u8>,
}

impl Darkmatter {
    pub fn new() -> Self {
        todo!();
    }

    pub fn analyze_content<E: Engine>(
        md: &MarkdownContent,
        fm: &Frontmatter,
        config: &Config<E>,
    ) -> Self {
        todo!();
    }
}
