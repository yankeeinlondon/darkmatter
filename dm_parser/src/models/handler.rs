use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::fm::MetaProperty;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Handler<O> {
    Callback,
    Static(O),
    RegExp(HashMap<&'static str, O>),
    IfElse(&'static str, (O, O)),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontmatterHandler {
    pub title: Option<Handler<String>>,
    pub description: Option<Handler<String>>,
    pub subject: Option<Handler<String>>,
    pub category: Option<Handler<String>>,
    pub name: Option<Handler<String>>,
    pub excerpt: Option<Handler<String>>,
    pub image: Option<Handler<String>>,
    pub image_height: Option<Handler<u32>>,
    pub image_width: Option<Handler<u32>>,
    pub layout: Option<Handler<String>>,
    pub requires_auth: Option<Handler<bool>>,
    pub meta: Vec<Handler<MetaProperty>>,
    /// Other properties who's type are not known until run time
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}
