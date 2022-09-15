use serde::{Deserialize, Serialize};

/// a string which represents Markdown content (no frontmatter)
#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownContent(String);
impl MarkdownContent {
    pub fn new(content: &str) -> Self {
        MarkdownContent(content.to_string())
    }
}
