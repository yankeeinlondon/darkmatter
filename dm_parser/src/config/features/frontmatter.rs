use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FrontmatterEngineType {
    YAML,
    JSON,
    TOML,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
}

impl FrontmatterConfig {
    pub fn default() -> Self {
        FrontmatterConfig {
            delimiter: None,
            excerpt_strategy: ExcerptStrategy::Auto,
            engine: FrontmatterEngineType::YAML,
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

        config
    }
}
