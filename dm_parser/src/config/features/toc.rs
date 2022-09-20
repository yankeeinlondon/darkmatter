use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TocOptions {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TocConfig {}

impl Default for TocConfig {
    fn default() -> Self {
        TocConfig {}
    }
}

impl TocConfig {
    pub fn with_options(options: TocOptions) -> Self {
        TocConfig::default()
    }
}
