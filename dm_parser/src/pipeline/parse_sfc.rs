use crate::{
    errors::parser_err::ParserError,
    models::{
        darkmatter::{Darkmatter, DmFinal},
        frontmatter::Frontmatter,
        html::HtmlContent,
        markdown::MarkdownContent,
        sfc::Sfc,
    },
};

use super::{remaining_darkmatter::RemainingDarkmatter, Pipeline, Stage};

pub struct ParseSfc(Pipeline<Self>);

impl Stage for ParseSfc {
    type MD = MarkdownContent;
    type FM = Frontmatter;
    type DM = Darkmatter<DmFinal>;
    type HTML = HtmlContent;
    type SFC = Sfc;
}

impl TryFrom<&RemainingDarkmatter> for ParseSfc {
    type Error = ParserError;
    fn try_from(ingress: &RemainingDarkmatter) -> Result<Self, ParserError> {
        todo!()
    }
}

impl ParseSfc {}
