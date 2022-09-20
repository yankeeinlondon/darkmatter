use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnOptions {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ColumnConfig {}

impl Default for ColumnConfig {
    fn default() -> Self {
        ColumnConfig {}
    }
}

impl ColumnConfig {
    pub fn with_options(options: ColumnOptions) -> Self {
        ColumnConfig::default()
    }
}
