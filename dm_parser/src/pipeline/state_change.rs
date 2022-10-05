use crate::{errors::parser_err::ParserError, models::frontmatter::Frontmatter};

use super::{
    //
    initialize::Initialize,
    parse_html::ParseHtml,
    parse_raw_md::ParseRawMd,
    Pipeline,
};

impl TryFrom<Pipeline<Initialize>> for Pipeline<ParseRawMd> {
    type Error = ParserError;
    fn try_from(ingress: Pipeline<Initialize>) -> Result<Self, ParserError> {
        let (markdown, frontmatter) = Frontmatter::extract(
            &ingress.markdown.unwrap(), //
            &ingress.config,
        )
        .unwrap();

        Ok(Pipeline {
            id: ingress.id,
            route: ingress.route,
            source: ingress.source,
            config: ingress.config,
            markdown,
            frontmatter,
            darkmatter: false,
            html: false,
            sfc: false,
        })
    }
}

/// Move from `ParseRawMd` to `ParseHtm` state
impl TryFrom<Pipeline<ParseRawMd>> for Pipeline<ParseHtml> {
    type Error = ParserError;
    fn try_from(value: Pipeline<ParseRawMd>) -> Result<Self, Self::Error> {
        todo!();
    }
}

impl TryFrom<Pipeline<ParseHtml>> for Pipeline<Htm> {
    type Error = ParserError;
    fn try_from(value: Pipeline<Initialize>) -> Result<Self, Self::Error> {
        todo!();
    }
}
