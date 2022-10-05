use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ContentType {
    Page,
    Post,
    ShortCode,
    Layout,
}

impl Default for ContentType {
    fn default() -> Self {
        ContentType::Page
    }
}
