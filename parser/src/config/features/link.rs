use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkOptions {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkConfig {}

impl LinkConfig {
    pub fn default() -> Self {
        todo!();
    }

    pub fn with_options(options: LinkOptions) -> Self {
        todo!();
    }
}
