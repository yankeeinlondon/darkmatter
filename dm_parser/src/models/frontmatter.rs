use super::markdown::MarkdownContent;
use super::markdown::MarkdownContentRaw;
use crate::config::features::frontmatter::ExcerptStrategy;
use crate::config::features::frontmatter::FrontmatterEngineType;
use crate::config::Config;
use gray_matter::engine::{Engine, JSON, TOML, YAML};
use gray_matter::{Matter, Pod};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrontmatterError {
    #[error("Problem parsing the frontmatter!")]
    Parsing(#[from] SerdeJsonError),
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
    pub title: Option<String>,
    pub description: Option<String>,
    pub subject: Option<String>,
    pub category: Option<String>,
    pub name: Option<String>,
    pub excerpt: Option<String>,
    pub image: Option<String>,
    pub image_height: Option<u32>,
    pub image_width: Option<u32>,
    pub layout: Option<String>,
    pub requires_auth: Option<bool>,
    pub meta: Vec<MetaProperty>,
    /// Other properties who's type are not known until run time
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

impl Frontmatter {
    pub fn new() -> Self {
        Frontmatter {
            title: None,
            description: None,
            subject: None,
            category: None,
            name: None,
            excerpt: None,
            image: None,
            image_height: None,
            image_width: None,
            layout: None,
            requires_auth: None,
            meta: vec![],
            other: HashMap::new(),
        }
    }

    /// Receives _raw markdown_ content and returns the Frontmatter
    /// and the markdown content with the Frontmatter extracted
    pub fn extract<E: Engine>(
        raw_md: &MarkdownContentRaw,
        config: &Config<E>,
    ) -> Result<(MarkdownContent, Frontmatter), FrontmatterError> {
        let fm = Frontmatter::new();
        let matter = Box::new(Matter::<E>::new());

        if let Some(delimiter) = &config.features.frontmatter.delimiter {
            matter.delimiter = delimiter.clone();
        }

        let mut frontmatter = if let Some(fm) = fm.data {
            Frontmatter::try_from(fm)?
        } else {
            Frontmatter::default()
        };

        let fm = matter.parse(raw_md);
        // Excerpt content extracted from the body parse
        let excerpt = fm.excerpt;
        let mut markdown = MarkdownContent::new(raw_md, &fm.content);
        // Work with excerpt based on strategy
        let preferred: &Option<String> = match config.features.frontmatter.excerpt_strategy {
            ExcerptStrategy::Auto => [&frontmatter.excerpt, &excerpt],
            ExcerptStrategy::Delimited(_) => [&excerpt, &None],
            ExcerptStrategy::Frontmatter => [&frontmatter.excerpt, &None],
            ExcerptStrategy::None => [&None, &None],
        }
        .into_iter()
        .filter(|i| **i.is_some())
        .first()
        .collect();

        frontmatter.excerpt = preferred;

        Ok((markdown, frontmatter))
    }

    /// converts the gray_matter representation of frontmatter
    /// data to the internal representation
    pub fn from_matter(pod: Pod) -> Result<Self, FrontmatterError> {
        let json: Value = pod.deserialize()?;

        todo!();
    }
}

impl TryFrom<Value> for Frontmatter {
    type Error = FrontmatterError;

    fn try_from(json: Value) -> Result<Frontmatter, FrontmatterError> {
        let fm = serde_json::from_value(json)?;

        Ok(fm)
    }
}

impl TryFrom<Pod> for Frontmatter {
    type Error = FrontmatterError;

    fn try_from(pod: Pod) -> Result<Frontmatter, FrontmatterError> {
        let json: Value = pod.deserialize()?;

        Frontmatter::try_from(json)
    }
}
