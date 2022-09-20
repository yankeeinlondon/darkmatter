use super::{darkmatter::Darkmatter, frontmatter::Frontmatter};
use crate::config::Config;
use serde::{Deserialize, Serialize};

/// Provides a hash of metadata used throughout the transformation
/// process.
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseContext {
    pub id: String,
    pub route: String,
    pub frontmatter: Frontmatter,
    pub darkmatter: Option<Darkmatter>,
    pub config: Config,
}

impl BaseContext {
    pub fn new(
        id: &str,
        route: &str,
        frontmatter: &Frontmatter,
        darkmatter: &Option<Darkmatter>,
        config: &Config,
    ) -> Self {
        BaseContext {
            id: id.to_string(),
            route: route.to_string(),
            frontmatter: frontmatter.clone(),
            darkmatter: if let Some(darkmatter) = darkmatter {
                Some(darkmatter.clone())
            } else {
                None
            },
            config: config.clone(),
        }
    }
}
