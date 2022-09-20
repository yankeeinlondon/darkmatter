use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageOptions {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageConfig {}

impl Default for ImageConfig {
    fn default() -> Self {
        ImageConfig {}
    }
}

impl ImageConfig {
    pub fn with_options(options: ImageOptions) -> Self {
        ImageConfig::default()
    }
}
