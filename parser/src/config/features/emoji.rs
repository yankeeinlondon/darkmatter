use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojiOptions {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmojiConfig {}

impl EmojiConfig {
    pub fn default() -> Self {
        todo!();
    }

    pub fn with_options(options: EmojiOptions) -> Self {
        todo!();
    }
}
