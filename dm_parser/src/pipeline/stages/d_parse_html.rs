use crate::{
    errors::parser_err::ParserError,
    models::{
        darkmatter::{Darkmatter, DmWhileParsing},
        frontmatter::Frontmatter,
        html::HtmlContent,
        markdown::MarkdownContent,
    },
    pipeline::{Pipeline, Stage},
};

use super::{c_initial_darkmatter::InitialDarkmatter, e_finalize_darkmatter::FinalizeDarkmatter};

/// The pipeline stage which focuses on parsing the markdown content
/// into HTML.
pub struct ParseHtml;

impl Stage for ParseHtml {
    type MD = MarkdownContent;
    type FM = Frontmatter;
    type DM = Darkmatter<DmWhileParsing>;
    type HTML = Option<HtmlContent>;
    type SFC = bool;
}

impl TryFrom<Pipeline<InitialDarkmatter>> for Pipeline<ParseHtml> {
    type Error = ParserError;

    fn try_from(value: Pipeline<InitialDarkmatter>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Pipeline<ParseHtml> {
    /// The body is wrapped by a block wrapper element if so configured
    pub fn wrap_html_body(&self) -> Self {
        *self
    }

    /// Use the pulldown parser to convert markdown
    /// to HTML. During this parsing process, the
    /// userland hooks for different structural types
    /// such as `<h1>..<h6>`, `<a>`, etc. tags will be
    /// called to provide efficient transforms _during_
    /// the parsing process.
    pub fn parse_to_html(&self) -> Result<Pipeline<ParseHtml>, ParserError> {
        Ok(*self)
    }

    /// Allows userland to hook into the darkmatter metadata
    /// generated in this initial stage prior to the HTML
    /// parsing.
    pub fn h_initial_darkmatter(&self) -> Result<Pipeline<ParseHtml>, ParserError> {
        Ok(*self)
    }

    /// Gather the remaining darkmatter which depended on HTML parsing
    /// to be complete.
    pub fn next_stage(&mut self) -> Result<Pipeline<FinalizeDarkmatter>, ParserError> {
        Pipeline::try_from(*self)
    }
}
