use serde::{Deserialize, Serialize};

use crate::{
    errors::parser_err::ParserError,
    models::{frontmatter::Frontmatter, markdown::MarkdownContent},
    pipeline::{Pipeline, Stage},
};

use super::{a_initialize::Initialize, c_initial_darkmatter::InitialDarkmatter};

/// Stage in the pipeline where the raw markdown content
/// is split into separate `Frontmatter` and `Markdown`
/// structs.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParseRawMd;

impl Stage for ParseRawMd {
    type MD = MarkdownContent;
    type FM = Frontmatter;
    type DM = bool;
    type HTML = bool;
    type SFC = bool;
}

impl TryFrom<Pipeline<Initialize>> for Pipeline<ParseRawMd> {
    type Error = ParserError;

    fn try_from(ingress: Pipeline<Initialize>) -> Result<Self, ParserError> {
        let (
            markdown, //
            frontmatter,
        ) = Frontmatter::extract(
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

impl Pipeline<ParseRawMd> {
    pub fn h_frontmatter_defaults(&mut self) -> Result<Pipeline<ParseRawMd>, ParserError> {
        Ok(*self)
    }

    pub fn h_frontmatter_overrides(&mut self) -> Result<Pipeline<ParseRawMd>, ParserError> {
        Ok(*self)
    }

    pub fn expand_shortcodes(&mut self) -> Self {
        *self
    }
    /// allows the raw markdown content to be mutated before any
    /// processing is done
    pub fn h_mutate_markdown(&mut self) -> Self {
        *self
    }

    /// Gather the first part of the darkmatter metadata which will
    /// then become available to all subsequent hooks/callbacks.
    pub fn next_stage(&mut self) -> Result<Pipeline<InitialDarkmatter>, ParserError> {
        Pipeline::try_from(*self)
    }
}
