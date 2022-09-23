use crate::{
    errors::parser_err::ParserError,
    models::{
        darkmatter::{Darkmatter, DmInitial},
        frontmatter::Frontmatter,
        markdown::MarkdownContent,
    },
};

use super::{parse_html::ParseHtml, parse_raw_md::ParseRawMd, Pipeline, Stage};

pub struct InitialDarkmatter(Pipeline<Self>);

impl Stage for InitialDarkmatter {
    type MD = MarkdownContent;
    type FM = Frontmatter;
    type DM = Darkmatter<DmInitial>;
    type HTML = bool;
    type SFC = bool;
}

impl TryFrom<&ParseRawMd> for InitialDarkmatter {
    type Error = ParserError;
    fn try_from(ingress: &ParseRawMd) -> Result<Self, ParserError> {
        todo!()
    }
}

impl InitialDarkmatter {
    /// applies the _default values_ for Frontmatter based on
    /// all context gathered so far.
    pub fn h_frontmatter_defaults(&mut self) -> Self {
        *self
    }

    /// Finalizes frontmatter key/values, overriding page values
    /// where appropriate.
    pub fn h_frontmatter_overrides(&self) -> Self {
        todo!();
    }

    /// If the configuration is targeting an HTML output then _all_
    /// references to frontmatter variables encapsulated in curlies will
    /// be replaced with the appropriate value.
    ///
    /// If the target is SFC then in most cases they will be left _as is_
    /// but in cases such as image or link URL references, the value will
    /// be URL encoded prior to VueJS seeing it so it either must be made
    /// static or converted into a bind variable.
    pub fn replace_static_handlebar_refs_to_frontmatter(&self) -> Self {
        *self
    }

    /// Parse the MD content into HTML while engaging all userland
    /// configuration to mutate accordingly.
    ///
    /// Note: this will also tokenize code blocks (and inline code) if
    /// configured to do so.
    pub fn parse_md_to_html(&self) -> Result<ParseHtml, ParserError> {
        todo!();
    }
}
