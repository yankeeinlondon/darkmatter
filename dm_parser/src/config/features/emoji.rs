use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojiOptions {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmojiConfig {}

impl Default for EmojiConfig {
    fn default() -> Self {
        EmojiConfig {}
    }
}

impl EmojiConfig {
    pub fn with_options(options: EmojiOptions) -> Self {
        EmojiConfig::default()
    }
}
