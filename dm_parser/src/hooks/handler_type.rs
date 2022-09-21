use serde::{Deserialize, Serialize};

use crate::models::handler::FrontmatterHandler;

#[derive(Debug, Serialize, Deserialize)]
pub enum HandlerType {
    Callback,
    RegExpMatcher,
    RegExpIfElse,
    Static,
}

pub fn handler_type(_ctx: &Option<FrontmatterHandler>) -> HandlerType {
    HandlerType::Callback
}
