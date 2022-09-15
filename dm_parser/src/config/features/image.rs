use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageOptions {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageConfig {}

impl ImageConfig {
    pub fn default() -> Self {
        todo!();
    }

    pub fn with_options(options: ImageOptions) -> Self {
        todo!();
    }
}
