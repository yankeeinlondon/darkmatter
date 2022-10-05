use serde::{Deserialize, Serialize};

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

impl TryFrom<&RemainingDarkmatter> for ParseSfc {
    type Error = ParserError;
    fn try_from(ingress: &RemainingDarkmatter) -> Result<Self, ParserError> {
        todo!()
    }
}

impl ParseSfc {}
