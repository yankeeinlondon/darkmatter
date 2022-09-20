use serde::{Deserialize, Serialize};

use crate::models::{handler::FrontmatterHandler, pipeline::Pipeline};

#[derive(Debug, Serialize, Deserialize)]
pub enum HandlerType {
    Callback,
    RegExpMatcher,
    RegExpIfElse,
    Static,
}

pub fn handler_type(ctx: &Option<FrontmatterHandler>) -> HandlerType {
    HandlerType::Callback
}
