use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TocItem {
    /// The text associated to the Header item
    text: String,
    level: usize,
    class_list: Vec<String>,
    children: Vec<TocItem>,
}

impl TocItem {
    pub fn new(text: String, level: usize) -> Self {
        TocItem {
            text,
            level,
            class_list: vec![],
            children: vec![],
        }
    }
}
