use super::errors::HookError;
use crate::{
    config::Config,
    models::{darkmatter::Darkmatter, frontmatter::Frontmatter},
};

pub fn fm_default_values(
    _route: &str,
    fm: Frontmatter,
    config: &Config,
) -> Result<Frontmatter, HookError> {
    match &config.hooks.frontmatter.override_values {
        Some(hook) => {
            // TODO: build this out
            Ok(fm)
        }
        None => Ok(fm),
    }
}

pub fn fm_override_values(
    _route: &str,
    fm: Frontmatter,
    _dm: &Darkmatter,
    config: &Config,
) -> Result<Frontmatter, HookError> {
    match &config.hooks.frontmatter.override_values {
        Some(hook) => {
            // TODO: build this out
            Ok(fm)
        }
        None => Ok(fm),
    }
}
