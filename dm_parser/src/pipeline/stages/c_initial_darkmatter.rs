use crate::{
    errors::parser_err::ParserError,
    models::{
        darkmatter::{Darkmatter, DmInitial},
        frontmatter::Frontmatter,
        markdown::MarkdownContent,
    },
    pipeline::{Pipeline, Stage},
};

use super::{b_parse_raw_md::ParseRawMd, d_parse_html::ParseHtml};

pub struct InitialDarkmatter;

impl Stage for InitialDarkmatter {
    type MD = MarkdownContent;
    type FM = Frontmatter;
    type DM = Darkmatter<DmInitial>;
    type HTML = bool;
    type SFC = bool;
}

impl TryFrom<Pipeline<ParseRawMd>> for Pipeline<InitialDarkmatter> {
    type Error = ParserError;

    fn try_from(value: Pipeline<ParseRawMd>) -> Result<Self, Self::Error> {
        todo!();
    }
}

impl Pipeline<InitialDarkmatter> {
    /// detects language or uses default
    pub fn lang_detection(&self) -> Result<Pipeline<InitialDarkmatter>, ParserError> {
        todo!();
    }

    pub fn tokenize(&self) -> Result<Pipeline<InitialDarkmatter>, ParserError> {
        todo!();
    }

    /// Produces a sentiment analysis score if so
    /// configured.
    pub fn sentiment(&self) -> Result<Pipeline<InitialDarkmatter>, ParserError> {
        todo!();
    }

    /// uses the XXX algorithm to determine language
    /// complexity (if so configured)
    pub fn complexity(&self) -> Result<Pipeline<InitialDarkmatter>, ParserError> {
        todo!();
    }

    /// calculate the time-to-read (in minutes).
    ///
    /// Note: if complexity is used then this will be
    /// incorporated into the TTR score but does not
    /// require it.
    pub fn ttr(&self) -> Result<Pipeline<InitialDarkmatter>, ParserError> {
        todo!();
    }

    /// Produces a bloom filter for content searching
    /// if so configured.
    pub fn bloom(&self) -> Result<Pipeline<InitialDarkmatter>, ParserError> {
        todo!();
    }

    pub fn next_stage(&mut self) -> Result<Pipeline<ParseHtml>, ParserError> {
        Pipeline::try_from(*self)
    }
}
