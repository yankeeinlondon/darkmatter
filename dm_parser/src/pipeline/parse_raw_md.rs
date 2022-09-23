use crate::{
    errors::parser_err::ParserError,
    models::{frontmatter::Frontmatter, markdown::MarkdownContent},
};
use serde::{Deserialize, Serialize};

use super::{initial_darkmatter::InitialDarkmatter, initialize::Initialize, Pipeline, Stage};

/// Stage in the pipeline where the raw markdown content
/// has been split into separate `Frontmatter` and `Markdown`
/// structs.
#[derive(Debug, Serialize, Deserialize)]
pub struct ParseRawMd(Pipeline<Self>);

impl Stage for ParseRawMd {
    type MD = MarkdownContent;
    type FM = Frontmatter;
    type DM = bool;
    type HTML = bool;
    type SFC = bool;
}

impl TryFrom<Pipeline<Initialize>> for ParseRawMd {
    type Error = ParserError;
    fn try_from(ingress: Pipeline<Initialize>) -> Result<Self, ParserError> {
        let (markdown, frontmatter) = Frontmatter::extract(
            &ingress.markdown.unwrap(), //
            &ingress.config,
        )
        .unwrap();

        Ok(ParseRawMd(Pipeline {
            id: ingress.id,
            route: ingress.route,
            source: ingress.source,
            config: ingress.config,
            markdown,
            frontmatter,
            darkmatter: false,
            html: false,
            sfc: false,
        }))
    }
}

impl ParseRawMd {
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
    pub fn gather_initial_darkmatter(&self) -> Result<InitialDarkmatter, ParserError> {
        todo!()
    }
}
