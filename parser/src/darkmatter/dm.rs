use serde::{Deserialize, Serialize};
use super::toc::TocItem;

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
    /// The Table of Contents of the page
    toc: TocItem,
}
