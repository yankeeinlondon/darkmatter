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

use super::e_finalize_darkmatter::FinalizeDarkmatter;

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalizeHtml;

impl Stage for FinalizeHtml {
    type MD = MarkdownContent;
    type FM = Frontmatter;
    type DM = Darkmatter<DmFinal>;
    type HTML = HtmlContent;
    type SFC = bool;
}

impl TryFrom<Pipeline<FinalizeDarkmatter>> for Pipeline<FinalizeHtml> {
    type Error = ParserError;
    fn try_from(
        ingress: Pipeline<FinalizeDarkmatter>,
    ) -> Result<Pipeline<FinalizeHtml>, ParserError> {
        todo!()
    }
}

impl Pipeline<FinalizeHtml> {
    /// Provides access to the HTML body at large and allows mutations
    ///
    /// Note: this is _after_ all the parsing hooks were executed and
    /// allows one last stab at mutation before the body is considered
    /// frozen.
    pub fn h_html_body(self) -> Result<Self, ParserError> {
        Ok(self)
    }

    /// Provides access to the array of script blocks which have accumulated
    /// so far. These script blocks _can_ be modified but more likely this
    /// hook offers the ability to add or possibly remove blocks.
    pub fn h_script_blocks(self) -> Result<Self, ParserError> {
        Ok(self)
    }

    /// Final chance to modify the `<title>` element of the header
    pub fn h_title(self) -> Result<Self, ParserError> {
        Ok(self)
    }

    /// Final chance to add/remove/modify `<meta>` tags in the header
    pub fn h_meta_tags(self) -> Result<Self, ParserError> {
        Ok(self)
    }

    /// Final chance to add/remove/modify `<meta>` tags in the header
    pub fn h_style_blocks(self) -> Result<Self, ParserError> {
        Ok(self)
    }

    pub fn h_script_refs(self) -> Result<Self, ParserError> {
        Ok(self)
    }

    pub fn h_style_refs(self) -> Result<Self, ParserError> {
        Ok(self)
    }

    pub fn h_toc(self) -> Result<Self, ParserError> {
        Ok(self)
    }

    /// Allows userland to hook into the metrics
    /// generated and mutate as needed.
    ///
    /// **Note:** _this is only called if the Output type
    /// is HTML. In other cases this hook will be
    /// called later._
    pub fn h_metrics(self) -> Result<Self, ParserError> {
        Ok(self)
    }
}
