use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::{Config, OutputFormat};

#[derive(Error, Debug)]
pub enum SfcError {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sfc {
    /// The SFC build target is not always meant to be
    /// built so this flag indicates whether the final
    /// state of this struct is really anything of value
    is_target_of_build: bool,
    pub setup_script: String,
    pub script_blocks: Vec<String>,
    pub template: String,
    pub style_blocks: Vec<String>,
}

impl Sfc {
    pub fn new(config: &Config) -> Self {
        Sfc {
            is_target_of_build: config.output == OutputFormat::SFC,
            setup_script: String::from(""),
            script_blocks: vec![],
            template: String::from(""),
            style_blocks: vec![],
        }
    }
}
