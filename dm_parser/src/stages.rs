use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    error::ParserError,
    hooks::handler_type::handler_type,
    pipeline::{Pipeline, Stage},
    source::Source,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Initialize {
    pub config: Config,
    pub id: String,
    pub route: String,
    pub source: Source,
}

impl Initialize {
    pub fn new(id: &str, config: Config) -> Self {
        let source = if id.starts_with("db::") {
            Source::Database
        } else {
            Source::File
        };
        Initialize {
            config,
            id: id.to_string(),
            route: id.to_string(),
            source,
        }
    }

    pub fn parse_raw_md(&self, md: &str) -> ParseMdFm {
        todo!()
    }
}

pub struct ParseMdFm {
    pub config: Config,
    pub id: String,
    pub route: String,
    pub markdown: Option<MarkdownContent>,
    pub frontmatter: Option<Frontmatter>,
}

impl ParseMdFm {}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Pipeline<T: Stage> {
//     pub config: Config,
//     pub id: String,
//     pub route: String,
//     pub markdown: T::MD,
//     pub frontmatter: T::FM,
//     pub darkmatter: T::DM,
//     pub html: T::HTML,
//     pub sfc: T::SFC,
// }

// impl Pipeline {
//     /// creates a new controller with access to all the _base context_
//     /// required for calling hooks.
//     pub fn new(
//         config: &Config,
//         id: &str,
//         route: &str,
//         frontmatter: &Frontmatter,
//         darkmatter: &Option<Darkmatter>,
//     ) -> Self {
//         Pipeline {
//             config: config.clone(),
//             id: id.to_string(),
//             route: route.to_string(),
//             frontmatter: frontmatter.clone(),
//             darkmatter: if let Some(darkmatter) = darkmatter {
//                 Some(darkmatter.clone())
//             } else {
//                 None
//             },
//             markdown: None,
//             html: None,
//             sfc: None,
//         }
//     }

//     pub fn process_darkmatter(mut self) -> Result<Pipeline, ParserError> {
//         let dm = Darkmatter::analyze_content(&self);
//         self.darkmatter = Some(dm);

//         Ok(self as Pipeline)
//     }

//     pub fn h_fm_default_values(&self) -> Result<Pipeline, ParserError> {
//         match handler_type(&self.config.hooks.frontmatter.default_values) {
//             _default => (),
//         }

//         todo!();
//     }

//     pub fn h_md_raw(&self) -> Result<Pipeline, ParserError> {
//         todo!();
//     }

//     pub fn h_fm_override_values(&self) -> Result<Pipeline, ParserError> {
//         todo!();
//     }

//     pub fn parse_html(&self) -> Result<Pipeline, ParserError> {
//         todo!();
//     }

//     pub fn highlight_code_blocks(&self) -> Result<Pipeline, ParserError> {
//         todo!();
//     }

//     pub fn h_code_block(&self) -> Result<Pipeline, ParserError> {
//         todo!();
//     }

//     pub fn h_code_block_formatted(&self) -> Result<Pipeline, ParserError> {
//         todo!();
//     }

//     pub fn parse_sfc(&self) -> Result<Pipeline, ParserError> {
//         todo!();
//     }
// }
