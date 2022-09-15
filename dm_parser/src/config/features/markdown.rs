use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownOptions {
    /// Github style tables
    tables: Option<bool>,
    /// Github style tasklists
    tasklists: Option<bool>,
    /// automatically convert standard punctuation to "smart punctuation"
    smart_punctuation: Option<bool>,
    ///
    heading_attributes: Option<bool>,
    footnotes: Option<bool>,
    strikethrough: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownConfig {
    tables: bool,
    tasklists: bool,
    smart_punctuation: bool,
    heading_attributes: bool,
    footnotes: bool,
    strikethrough: bool,
}

impl MarkdownConfig {
    pub fn default() -> Self {
        MarkdownConfig {
            tables: true,
            tasklists: true,
            smart_punctuation: true,
            heading_attributes: true,
            footnotes: true,
            strikethrough: true,
        }
    }

    pub fn with_options(options: MarkdownOptions) {
        let mut config = MarkdownConfig::default();

        if let Some(tables) = options.tables {
            config.tables = tables;
        }
        if let Some(tasklists) = options.tasklists {
            config.tasklists = tasklists;
        }
        if let Some(smart_punctuation) = options.smart_punctuation {
            config.smart_punctuation = smart_punctuation;
        }
        if let Some(heading_attributes) = options.heading_attributes {
            config.heading_attributes = heading_attributes;
        }
        if let Some(footnotes) = options.footnotes {
            config.footnotes = footnotes;
        }
        if let Some(strikethrough) = options.strikethrough {
            config.strikethrough = strikethrough;
        }
    }
}
