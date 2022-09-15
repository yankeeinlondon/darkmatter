use crate::{
    config::Config,
    models::{dm::Darkmatter, frontmatter::Frontmatter},
};

use super::errors::HookError;

pub fn fm_override_values(
    route: &'static str,
    fm: Frontmatter,
    dm: &Darkmatter,
    config: &Config,
) -> Result<Frontmatter, HookError> {
    match config.hooks.frontmatter.override_values {
        Some(hook) => {
            // TODO: build this out
            Ok(fm)
        }
        None => Ok(fm),
    }
}
