use crate::{
    errors::parser_err::ParserError,
    models::{
        darkmatter::{Darkmatter, DmFinal},
        frontmatter::Frontmatter,
        html::HtmlContent,
        markdown::MarkdownContent,
    },
};

use super::{parse_html::ParseHtml, parse_sfc::ParseSfc, Pipeline, Stage};

pub struct RemainingDarkmatter(Pipeline<Self>);

impl Stage for RemainingDarkmatter {
    type MD = MarkdownContent;
    type FM = Frontmatter;
    type DM = Darkmatter<DmFinal>;
    type HTML = HtmlContent;
    type SFC = bool;
}

impl TryFrom<&ParseHtml> for RemainingDarkmatter {
    type Error = ParserError;
    fn try_from(ingress: &ParseHtml) -> Result<Self, ParserError> {
        todo!()
    }
}

impl RemainingDarkmatter {
    pub fn parse_to_sfc(&self) -> Result<ParseSfc, ParserError> {
        todo!();
    }

    /// Provides access to the HTML body at large and allows mutations
    ///
    /// Note: this is _after_ all the parsing hooks were executed and
    /// allows one last stab at mutation before the body is considered
    /// frozen.
    pub fn h_html_body(&self) -> Self {
        *self
    }

    /// Provides access to the array of script blocks which have accumulated
    /// so far. These script blocks _can_ be modified but more likely this
    /// hook offers the ability to add or possibly remove blocks.
    pub fn h_header_script_blocks(&self) -> Self {
        *self
    }

    /// Final chance to modify the `<title>` element of the header
    pub fn h_header_title(&self) -> Self {
        *self
    }

    /// Final chance to add/remove/modify `<meta>` tags in the header
    pub fn h_header_meta_tags(&self) -> Self {
        *self
    }

    /// Final chance to add/remove/modify `<meta>` tags in the header
    pub fn h_header_style_blocks(&self) -> Self {
        *self
    }

    pub fn default_metrics(&self) -> Self {
        *self
    }

    /// Allows userland to hook into the metrics generated and mutate as needed
    pub fn h_metrics(&self) -> Self {
        *self
    }
}
