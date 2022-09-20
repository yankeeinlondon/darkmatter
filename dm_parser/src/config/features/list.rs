use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOptions {
    pub all_lists_are_collapsible: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListConfig {
    pub all_lists_are_collapsible: bool,
}

impl Default for ListConfig {
    fn default() -> Self {
        ListConfig {
            all_lists_are_collapsible: false,
        }
    }
}

impl ListConfig {
    pub fn with_options(options: ListOptions) -> Self {
        let mut config = ListConfig::default();

        if let Some(all_lists_are_collapsible) = options.all_lists_are_collapsible {
            config.all_lists_are_collapsible = all_lists_are_collapsible;
        }

        config
    }
}
