use self::{
    features::{FeatureOptions, FeaturesConfig},
    hooks::{HookConfig, HookOptions},
};
use gray_matter::engine::Engine;
use serde::{Deserialize, Serialize};

pub mod features;
pub mod hooks;

#[derive(Debug, Serialize, Deserialize)]
pub enum OutputFormat {
    HTML,
    SFC,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options<E: Engine> {
    /// determines the assets which this pipeline will target as the final product
    pub output: Option<OutputFormat>,
    /// specify the features you would like
    pub features: Option<FeatureOptions<E>>,
    /// specify the hooks
    pub hooks: Option<HookOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config<E: Engine> {
    /// the _output_ format which the transformation pipeline is to emit
    pub output: OutputFormat,
    /// configuration for NLP algorithms
    pub features: FeaturesConfig<E>,
    /// Frontmatter specific configuration
    pub hooks: HookConfig,
}

impl Config<dyn Engine> {
    pub fn default() -> Self {
        let output = OutputFormat::SFC;
        let features = FeaturesConfig::default();
        let hooks = HookConfig::default();

        Config {
            output,
            features,
            hooks,
        }
    }

    pub fn with_options<E: Engine>(options: &Options<E>) -> Self {
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
