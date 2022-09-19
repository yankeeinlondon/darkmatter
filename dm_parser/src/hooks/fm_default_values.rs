use gray_matter::engine::Engine;

use crate::{
    config::Config,
    models::{darkmatter::Darkmatter, frontmatter::Frontmatter},
};

use super::errors::HookError;

pub fn fm_default_values<E: Engine>(
    route: &str,
    fm: Frontmatter,
    config: &Config<E>,
) -> Result<Frontmatter, HookError> {
    match &config.hooks.frontmatter.override_values {
        Some(hook) => {
            // TODO: build this out
            Ok(fm)
        }
        None => Ok(fm),
    }
}
