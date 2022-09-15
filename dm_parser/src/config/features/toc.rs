use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TocOptions {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TocConfig {}

impl TocConfig {
    pub fn default() -> Self {
        todo!();
    }

    pub fn with_options(options: TocOptions) -> Self {
        todo!();
    }
}
