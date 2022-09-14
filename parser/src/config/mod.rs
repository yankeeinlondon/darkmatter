use self::{
    features::FeatureOptions,
    frontmatter::{FrontmatterConfig, FrontmatterOptions},
    nlp::{NlpConfig, NlpOptions},
};
use serde::{Deserialize, Serialize};

pub mod features;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpMatcher<T> {
    /// match arms which contain a _string_ which is intended to be a Regex expression
    /// and the value `T` which will result if this Regex passes
    arms: Vec<(String, T)>,
    /// optionally a value which will be used if no arms are matched
    fallback: Option<T>,
}

impl RegExpMatcher<T> {
    pub fn new(arms: Vec<(String, T)>, fallback: Option<T>) -> Self {
        RegExpMatcher { arms, fallback }
    }
}

/// An enumerated set of ways in which a value can be set
#[derive(Debug, Serialize, Deserialize)]
pub enum ValueSetter<T> {
    // returns a static value
    Static(T),
    RegExp(RegExpMatcher<T>),
    RegisterCallback(),
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub hooks: Option<HookOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// the _output_ format which the transformation pipeline is to emit
    pub output: OutputFormat,
    /// configuration for NLP algorithms
    pub nlp: NlpConfig,
    /// Frontmatter specific configuration
    pub frontmatter: FrontmatterConfig,
}

impl Config {
    pub fn default() -> Self {
        let output = OutputFormat::SFC;
        let nlp = NlpConfig::default();
        let frontmatter = FrontmatterConfig::default();

        Config {
            output,
            nlp,
            frontmatter,
        }
    }
}
