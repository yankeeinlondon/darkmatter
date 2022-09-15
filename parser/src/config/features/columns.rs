use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnOptions {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnConfig {}

impl ColumnConfig {
    pub fn default() -> Self {
        todo!();
    }

    pub fn with_options(options: ColumnOptions) -> Self {
        todo!();
    }
}
