use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollapsibleOptions {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollapsibleConfig {}

impl CollapsibleConfig {
    pub fn default() -> Self {
        todo!();
    }

    pub fn with_options(options: CollapsibleOptions) -> Self {
        todo!();
    }
}
