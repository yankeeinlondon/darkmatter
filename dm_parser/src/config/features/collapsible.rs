use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollapsibleOptions {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollapsibleConfig {}

impl Default for CollapsibleConfig {
    fn default() -> Self {
        CollapsibleConfig {}
    }
}

impl CollapsibleConfig {
    pub fn with_options(options: CollapsibleOptions) -> Self {
        CollapsibleConfig::default()
    }
}
