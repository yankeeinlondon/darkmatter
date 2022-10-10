use serde::{Deserialize, Serialize};

use crate::{config::Config, source::Source};

use self::stages::a_initialize::Initialize;

pub mod stages;

pub trait Stage {
    type MD;
    type FM;
    type DM;
    type HTML;
    type SFC;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline<'a, S: Stage> {
    /// an immutable identifier of the content which
    /// also represents the "route/filepath" of the asset
    /// at the beginning of the transformation pipeline
    pub id: String,
    /// starts out as being the same as the `id` but unlike
    /// the `id` which is immutable, the route is mutable via
    /// userland hooks during the transformation pipeline.
    pub route: &'a str,
    /// the userland configuration for a given project / repo
    pub config: Config,
    /// the originating _source_ of the asset
    pub source: Source,
    /// the _markdown_ content
    pub markdown: S::MD,
    /// the _frontmatter_ meta data
    pub frontmatter: S::FM,
    /// the _darkmatter_ meta data
    pub darkmatter: S::DM,
    /// the resultant HTML produced, including inline
    /// styles and scripts
    pub html: S::HTML,
    /// the VueJS SFC file structure (if configured to produce)
    pub sfc: S::SFC,
}

impl<'a> Pipeline<'a, Initialize> {
    pub fn new(id: &str, config: Config) -> Pipeline<Initialize> {
        let source = match &id.starts_with("db::") {
            true => Source::Database,
            false => Source::File,
        };

        Pipeline {
            id: id.to_string(),
            route: id,
            config,
            source,
            markdown: None,
            frontmatter: false,
            darkmatter: false,
            html: false,
            sfc: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_pipeline() {
        let p = Pipeline::new("foobar.md", Config::default());
        assert_eq!(p.id, "foobar.md".to_string());
        assert_eq!(p.route, "foobar.md");
        assert_eq!(p.source, Source::File);
    }
}
