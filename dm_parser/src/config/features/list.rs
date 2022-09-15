use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOptions {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListConfig {}

impl ListConfig {
    pub fn default() -> Self {
        todo!();
    }

    pub fn with_options(options: ListOptions) -> Self {
        todo!();
    }
}
