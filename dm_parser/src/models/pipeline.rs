use crate::{config::Config, hooks::handler_type::handler_type, error::ParserError};

use super::{context::BaseContext, darkmatter::Darkmatter, frontmatter::Frontmatter, markdown::MarkdownContent, html::HtmlContent, sfc::Sfc};

pub struct Pipeline {
    pub id: String,
    pub route: String,
    pub frontmatter: Frontmatter,
    pub darkmatter: Option<Darkmatter>,
    pub config: Config,
    pub markdown: Option<MarkdownContent>,
    pub html: Option<HtmlContent>,
    pub sfc: Option<Sfc>
};

impl Pipeline {
    /// creates a new controller with access to all the _base context_
    /// required for calling hooks.
    pub fn new(
        config: &Config,
        id: &str,
        route: &str,
        frontmatter: &Frontmatter,
        darkmatter: &Option<Darkmatter>,
    ) -> Self {
        Pipeline {
            config: *config.clone(),
            id: id.to_string(),
            route: route.to_string(),
            frontmatter: *frontmatter.clone(),
            darkmatter: if let Some(darkmatter) = darkmatter {
                Some(*darkmatter.clone())
            } else {
                None
            },
            markdown: None,
            html: None,
            sfc: None
        }
    }

    pub fn process_darkmatter(&mut self) -> Result<Pipeline, ParserError> {
        let dm = Darkmatter::analyze_content(&self);
        self.darkmatter = Some(dm);

        Ok(*self)
    }

    pub fn h_fm_default_values(&self) -> Result<Pipeline, ParserError> {
        match handler_type(self.config.hooks.frontmatter.default_values) {
            default => ()
        }

        todo!()
    }

    pub fn h_md_raw(&self) -> Result<Pipeline, ParserError> {
        todo!();
    }


    pub fn h_fm_override_values(&self) -> Result<Pipeline, ParserError> {
        todo!();
    }

    pub fn parse_html(&self) -> Result<Pipeline, ParserError> {
        todo!();
    }

    pub fn highlight_code_blocks(&self) -> Result<Pipeline, ParserError> {
        todo!();
    }

    pub fn h_code_block(&self) -> Result<Pipeline, ParserError> {
        todo!();
    }

    pub fn h_code_block_formatted(&self) -> Result<Pipeline, ParserError> {
        todo!();
    }    


    pub fn parse_sfc(&self) -> Result<Pipeline, ParserError> {
        todo!();
    }
}
