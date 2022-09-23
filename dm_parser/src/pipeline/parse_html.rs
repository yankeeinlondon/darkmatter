use crate::{
    errors::parser_err::ParserError,
    models::{
        darkmatter::{Darkmatter, DmWhileParsing},
        frontmatter::Frontmatter,
        html::HtmlContent,
        markdown::MarkdownContent,
    },
};

use super::{
    initial_darkmatter::InitialDarkmatter, remaining_darkmatter::RemainingDarkmatter, Pipeline,
    Stage,
};

pub struct ParseHtml(Pipeline<Self>);

impl Stage for ParseHtml {
    type MD = MarkdownContent;
    type FM = Frontmatter;
    type DM = Darkmatter<DmWhileParsing>;
    type HTML = Option<HtmlContent>;
    type SFC = bool;
}

impl TryFrom<&InitialDarkmatter> for ParseHtml {
    type Error = ParserError;
    fn try_from(value: &InitialDarkmatter) -> Result<Self, Self::Error> {
        todo!();
    }
}

impl ParseHtml {
    /// Gather the remaining darkmatter which depended on HTML parsing
    /// to be complete.
    pub fn gather_remaining_darkmatter(&self) -> Result<RemainingDarkmatter, ParserError> {
        Ok(RemainingDarkmatter::try_from(self))
    }

    /// The body is wrapped by a block wrapper element if so configured
    pub fn wrap_html_body(&self) -> Self {
        *self
    }
}
