use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineOptions {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineConfig {}

impl InlineConfig {
    pub fn default() -> Self {
        todo!();
    }

    pub fn with_options(options: InlineOptions) -> Self {
        todo!();
    }
}
