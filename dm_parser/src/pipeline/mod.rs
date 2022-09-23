use serde::{Deserialize, Serialize};

use crate::{config::Config, source::Source};
pub mod initial_darkmatter;
pub mod initialize;
pub mod parse_html;
pub mod parse_raw_md;
pub mod parse_sfc;
pub mod remaining_darkmatter;
pub trait Stage {
    type MD;
    type FM;
    type DM;
    type HTML;
    type SFC;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline<S: Stage> {
    id: String,
    route: String,
    config: Config,
    source: Source,
    markdown: S::MD,
    frontmatter: S::FM,
    darkmatter: S::DM,
    html: S::HTML,
    sfc: S::SFC,
}
