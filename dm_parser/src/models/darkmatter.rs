use super::pipeline::Pipeline;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Error)]
pub enum DarkmatterError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn analyze_content(_ctx: &Pipeline) -> Self {
        todo!();
    }
}
