use self::{
    features::{FeatureOptions, FeaturesConfig},
    hooks::{HookConfig, HookOptions},
};
use serde::{Deserialize, Serialize};

pub mod features;
pub mod hooks;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum OutputFormat {
    HTML,
    SFC,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    /// determines the assets which this pipeline will target as the final product
    pub output: Option<OutputFormat>,
    /// specify the features you would like
    pub features: Option<FeatureOptions>,
    /// specify the hooks
    pub hooks: Option<HookOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// the _output_ format which the transformation pipeline is to emit
    pub output: OutputFormat,
    /// configuration for NLP algorithms
    pub features: FeaturesConfig,
    /// Frontmatter specific configuration
    pub hooks: HookConfig,
}

impl Default for Config {
    fn default() -> Self {
        let output = OutputFormat::SFC;
        let features = FeaturesConfig::default();
        let hooks = HookConfig::default();

        Config {
            output,
            features,
            hooks,
        }
    }
}

impl Config {
    pub fn with_options(options: &Options) -> Self {
        let output = OutputFormat::SFC;
        let features = match &options.features {
            Some(features) => FeaturesConfig::with_options(features),
            None => FeaturesConfig::default(),
        };

        let hooks = match &options.hooks {
            Some(hooks) => HookConfig::with_options(hooks),
            None => HookConfig::default(),
        };

        Config {
            output,
            features,
            hooks,
        }
    }
}
