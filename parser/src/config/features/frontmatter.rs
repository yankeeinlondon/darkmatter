use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum FrontmatterEngineType {
    YAML,
    JSON,
    TOML,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExcerptStrategy {
    /// the **auto** strategy will use the "excerpt" property if found
    /// on the document, if not present it will try extract from the body
    /// using the `\n---` delimiter. Finally, if the NLP `summarize` is turned
    /// on it will add this summary as the excerpt.
    Auto,
    /// The **delimited** strategy parses the excerpt from the body of the page where it can find the "delimiter". You may specify what this delimiter should be
    /// but by default it will be `\n---`.
    Delimited(Option<String>),
    /// will not attempt to use the delimited text in the body of the page but
    /// instead rely exclusively on the `excerpt` property in Frontmatter.
    Frontmatter,
    /// do not attempt to create an excerpt
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontmatterOptions {
    /// Specifies how the "excerpt" property on frontmatter should be derived
    /// (if at all). The Auto strategy will try frontmatter first, then use
    /// the `\n---` excerpt delimiter on body, and finally use the "generated"
    /// excerpt if the NLP feature is enabled.
    ///
    /// @default Auto
    excerpt_strategy: Option<ExcerptStrategy>,

    /// The language structure/engine used to set frontmatter dictionary;
    /// options are YAML, JSON, and TOML but _defaults to_ YAML.
    ///
    /// @default YAML
    engine: Option<FrontmatterEngineType>,

    /// Any _default values_ for the Frontmatter variables
    ///
    /// @default None
    default_values: Option<FrontmatterSetter>,

    /// Any _override values_ for the Frontmatter variables which take
    /// precedent over even page specified values.
    ///
    /// @default None
    override_values: Option<FrontmatterSetter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontmatterConfig {
    delimiter: Option<String>,
    /// Specifies how the "excerpt" property on frontmatter should be derived
    /// (if at all).
    ///
    /// @default Auto
    excerpt_strategy: ExcerptStrategy,
    /// the language structure/engine used to set frontmatter dictionary;
    /// options are YAML, JSON, and TOML but _defaults to_ YAML.
    ///
    /// @default YAML
    engine: FrontmatterEngineType,

    /// Any _default values_ for the Frontmatter variables
    default_values: Option<FrontmatterSetter>,
    /// Any _override values_ for the Frontmatter variables which take
    /// precedent over even page specified values.
    override_values: Option<FrontmatterSetter>,
}

impl FrontmatterConfig {
    pub fn default() -> Self {
        FrontmatterConfig {
            delimiter: None,
            excerpt_strategy: ExcerptStrategy::Auto,
            engine: FrontmatterEngineType::YAML,
            default_values: None,
            override_values: None,
        }
    }

    pub fn with_options(options: FrontmatterOptions) -> Self {
        let config = FrontmatterConfig::default();

        if let Some(engine) = options.engine {
            config.engine = engine;
        }
        if let Some(excerpt_strategy) = options.excerpt_strategy {
            config.excerpt_strategy = excerpt_strategy;
        }
        config.default_values = options.default_values;
        config.override_values = options.override_values;

        config
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaProperty {
    key: Option<String>,
    /// the "name" property used by Facebook and other providers who
    /// use the OpenGraph standards
    property: Option<String>,
    /// used by google to identify the "name" of the name/value pair
    itemprop: Option<String>,
    /// used by Twitter to indicate the "name" field in a meta properties
    /// name/value pairing
    name: Option<String>,
    /// The value of the meta property
    content: Option<Value>,
    /// properties only known at run time
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frontmatter {
    title: Option<String>,
    description: Option<String>,
    subject: Option<String>,
    category: Option<String>,
    name: Option<String>,
    excerpt: Option<String>,
    image: Option<String>,
    layout: Option<String>,
    requires_auth: Option<bool>,
    meta: Vec<MetaProperty>,
    /// Other properties who's type are not known until run time
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

/// A data structure which allows for setting _default_ and _override_
/// values for Frontmatter.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontmatterSetter {
    title: ValueSetter<Option<String>>,
    description: ValueSetter<Option<String>>,
    subject: ValueSetter<Option<String>>,
    category: ValueSetter<Option<String>>,
    name: ValueSetter<Option<String>>,
    excerpt: ValueSetter<Option<String>>,
    image: ValueSetter<Option<String>>,
    layout: ValueSetter<Option<String>>,
    requires_auth: ValueSetter<Option<bool>>,
    meta: Vec<MetaProperty>,
    /// Other properties who's type are not known until run time
    #[serde(flatten)]
    other: HashMap<String, Value>,
}
