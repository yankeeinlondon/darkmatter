use serde::{Deserialize, Serialize};

use crate::{
    models::{
        darkmatter::{Darkmatter, DmFinal},
        frontmatter::Frontmatter,
        html::HtmlContent,
        markdown::MarkdownContent,
        sfc::Sfc,
    },
    pipeline::Stage,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SfcConversion;

impl Stage for SfcConversion {
    type MD = MarkdownContent;
    type FM = Frontmatter;
    type DM = Darkmatter<DmFinal>;
    type HTML = HtmlContent;
    type SFC = Sfc;
}
