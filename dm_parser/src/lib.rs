use std::process::Output;

use config::{Config, Options};
use errors::parser_err::ParserError;
use pipeline::{
    initialize::Initialize, parse_sfc::ParseSfc, remaining_darkmatter::RemainingDarkmatter,
    Pipeline,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

pub mod config;
pub mod errors;
pub mod hooks;
pub mod models;
pub mod pipeline;
pub mod source;

#[derive(Debug, Serialize, Deserialize)]
pub enum ParsedOutput {
    Html(Pipeline<RemainingDarkmatter>),
    Sfc(Pipeline<ParseSfc>),
}

/// The key parsing/transform library which converts markdown into a
/// target output that the user specifies as part of their configuration.
#[instrument]
pub fn parse(id: &str, content: &str, options: &Options) -> Result<ParsedOutput, ParserError> {
    let config = Config::with_options(options);
    let mut pipeline = Initialize::new(
        //
        "foobar", &config,
    )
    .parse_raw_md(content)?
    .expand_shortcodes()
    .h_mutate_markdown()
    .gather_initial_darkmatter()?
    .h_frontmatter_defaults()
    .h_frontmatter_overrides()
    .replace_static_handlebar_refs_to_frontmatter()
    .parse_md_to_html()?
    .wrap_html_body()
    .gather_remaining_darkmatter()?
    .h_html_body()
    .h_header_script_blocks()
    .h_header_title()
    .h_header_meta_tags()
    .h_header_style_blocks()
    .default_metrics()
    .h_metrics();

    if &config.output == Output::SFC {
        pipeline = ParseSfc::from(pipeline)?;
    }

    Ok(pipeline.into())
}
