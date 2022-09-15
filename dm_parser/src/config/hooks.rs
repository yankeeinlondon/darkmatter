use serde::{Deserialize, Serialize};

use crate::models::handler::{FrontmatterHandler, Handler};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontmatterHooks {
    /// Any _default values_ for the Frontmatter variables
    pub default_values: Option<FrontmatterHandler>,
    /// Any _override values_ for the Frontmatter variables which take
    /// precedent over even page specified values.    
    pub override_values: Option<FrontmatterHandler>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeHooks {
    line_content: Option<Handler<String>>,
    block_content: Option<Handler<String>>,
    block_content_formatted: Option<Handler<String>>,
    inline_code: Option<Handler<String>>,
    inline_code_formatted: Option<Handler<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HookOptions {
    pub frontmatter: Option<FrontmatterHooks>,
    pub code: Option<CodeHooks>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HookConfig {
    pub frontmatter: FrontmatterHooks,
    pub code: CodeHooks,
}

impl HookConfig {
    pub fn default() -> Self {
        todo!();
    }

    pub fn with_options(options: &HookOptions) -> Self {
        todo!();
    }
}
