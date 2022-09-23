use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ContentType {
    Page,
    Post,
    ShortCode,
    Layout,
}
