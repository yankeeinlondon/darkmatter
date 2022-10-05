use serde::{Deserialize, Serialize};

use crate::{
    errors::parser_err::ParserError,
    models::{
        darkmatter::{Darkmatter, DmFinal},
        frontmatter::Frontmatter,
        html::HtmlContent,
        markdown::MarkdownContent,
    },
    pipeline::{Pipeline, Stage},
};

use super::{d_parse_html::ParseHtml, f_finalize_html::FinalizeHtml};

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalizeDarkmatter;

impl Stage for FinalizeDarkmatter {
    type MD = MarkdownContent;
    type FM = Frontmatter;
    type DM = Darkmatter<DmFinal>;
    type HTML = HtmlContent;
    type SFC = bool;
}

impl TryFrom<Pipeline<ParseHtml>> for Pipeline<FinalizeDarkmatter> {
    type Error = ParserError;
    fn try_from(ingress: Pipeline<ParseHtml>) -> Result<Pipeline<FinalizeDarkmatter>, ParserError> {
        todo!()
    }
}

impl Pipeline<FinalizeDarkmatter> {
    pub fn darkmatter_metrics(&self) -> Self {
        *self
    }

    /// Builds the table of contents from the `<h1>`
    /// to `<h6>` tags discovered in the document and
    /// hashes it appropriately.
    pub fn toc(&self) -> Result<Self, ParserError> {
        Ok(*self)
    }

    pub fn next_stage(&mut self) -> Result<Pipeline<FinalizeHtml>, ParserError> {
        Pipeline::try_from(*self)
    }
}
