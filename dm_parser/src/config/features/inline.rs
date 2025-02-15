use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineOptions {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InlineConfig {}

impl Default for InlineConfig {
    fn default() -> Self {
        InlineConfig {}
    }
}

impl InlineConfig {
    pub fn with_options(options: InlineOptions) -> Self {
        InlineConfig::default()
    }
}
