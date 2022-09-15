use std::collections::HashMap;

use gray_matter::Pod;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

    /// converts the gray_matter representation of frontmatter
    /// data to the internal representation
    pub fn from_matter(pod: Pod) -> Self {
        todo!();
    }
}
