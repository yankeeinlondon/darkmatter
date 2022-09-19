use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum HandlerType {
    Callback,
    RegExpMatcher,
    RegExpIfElse,
    Static,
}

#[derive(Debug, Serialize, Deserialize)]
pub fn handler_type(ctx: &Context) -> HandlerType {
    todo!();
}
