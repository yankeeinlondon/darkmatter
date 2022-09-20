use serde::{Deserialize, Serialize};

use crate::models::handler::{FrontmatterHandler, Handler};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FrontmatterHooks {
    /// Any _default values_ for the Frontmatter variables
    pub default_values: Option<FrontmatterHandler>,
    /// Any _override values_ for the Frontmatter variables which take
    /// precedent over even page specified values.    
    pub override_values: Option<FrontmatterHandler>,
}

impl Default for FrontmatterHooks {
    fn default() -> Self {
        FrontmatterHooks {
            default_values: None,
            override_values: None,
        }
    }
}

impl FrontmatterHooks {
    fn with_options(options: FrontmatterHooks) -> Self {
        let mut hooks = FrontmatterHooks::default();

        if let Some(default_values) = options.default_values {
            hooks.default_values = Some(default_values);
        }
        if let Some(override_values) = options.override_values {
            hooks.override_values = Some(override_values);
        }

        hooks
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CodeHooks {
    line_content: Option<Handler<String>>,
    block_content: Option<Handler<String>>,
    block_content_formatted: Option<Handler<String>>,
    inline_code: Option<Handler<String>>,
    inline_code_formatted: Option<Handler<String>>,
}

impl Default for CodeHooks {
    fn default() -> Self {
        CodeHooks {
            line_content: None,
            block_content: None,
            block_content_formatted: None,
            inline_code: None,
            inline_code_formatted: None,
        }
    }
}

impl CodeHooks {
    pub fn with_options(options: CodeHooks) -> Self {
        let mut hooks = CodeHooks::default();

        if let Some(line_content) = options.line_content {
            hooks.line_content = Some(line_content);
        }
        if let Some(block_content) = options.block_content {
            hooks.block_content = Some(block_content);
        }
        if let Some(block_content_formatted) = options.block_content_formatted {
            hooks.block_content_formatted = Some(block_content_formatted);
        }
        if let Some(inline_code) = options.inline_code {
            hooks.inline_code = Some(inline_code);
        }
        if let Some(inline_code_formatted) = options.inline_code_formatted {
            hooks.inline_code_formatted = Some(inline_code_formatted);
        }

        hooks
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HookOptions {
    pub frontmatter: Option<FrontmatterHooks>,
    pub code: Option<CodeHooks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HookConfig {
    pub frontmatter: FrontmatterHooks,
    pub code: CodeHooks,
}

impl Default for HookConfig {
    fn default() -> Self {
        HookConfig {
            frontmatter: FrontmatterHooks::default(),
            code: CodeHooks::default(),
        }
    }
}

impl HookConfig {
    pub fn with_options(options: &HookOptions) -> Self {
        HookConfig {
            frontmatter: match &options.frontmatter {
                Some(frontmatter) => FrontmatterHooks::with_options(frontmatter.clone()),
                None => FrontmatterHooks::default(),
            },
            code: match &options.code {
                Some(code) => CodeHooks::with_options(code.clone()),
                None => CodeHooks::default(),
            },
        }
    }
}
