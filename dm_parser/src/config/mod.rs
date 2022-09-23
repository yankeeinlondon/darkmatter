use crate::models::content_type::ContentType;

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
    /// Allows specifying which directories will have various "content-types"
    /// where a "content-type" are things like `Page`, `ShortCode`, and `Post`.
    ///
    /// By default Darkmatter is opinionated on this topic:
    /// ```json
    /// [
    ///     ["src/pages/**/*.(md|dm)", "page"],
    ///     ["src/shortcodes/**/*.html", "shortCode"],
    ///     ["src/posts", "post"],
    /// ]
    /// ```
    pub dir_mapping: Option<Vec<(String, ContentType)>>,
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
    /// Allows specifying which directories will have various "content-types"
    /// where a "content-type" are things like `Page`, `ShortCode`, and `Post`.
    pub dir_mapping: Vec<(String, ContentType)>,

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
        let dir_mapping = vec![
            (String::from("src/pages/**/*.(md|dm)"), ContentType::Page),
            (
                String::from("src/shortcodes/**/*.html"),
                ContentType::ShortCode,
            ),
            (String::from("src/posts**/*.(md|dm)"), ContentType::Post),
        ];

        Config {
            output,
            features,
            dir_mapping,
            hooks,
        }
    }
}

impl Config {
    pub fn with_options(options: &Options) -> Self {
        let mut config = Config::default();
        if let Some(output) = options.output {
            config.output = output;
        }
        if let Some(features) = options.features {
            config.features = FeaturesConfig::with_options(options.features);
        }
        if let Some(dir_mapping) = options.dir_mapping {
            config.dir_mapping = dir_mapping;
        }
        if let Some(hooks) = options.hooks {
            config.hooks = HookConfig::with_options(options.hooks);
        }

        config
    }
}
