use config::{Config, Options};
use errors::parser_err::ParserError;
use pipeline::{
    stages::{f_finalize_html::FinalizeHtml, g_sfc::SfcConversion},
    Pipeline,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::source::Source;

pub mod config;
pub mod errors;
pub mod hooks;
pub mod models;
pub mod pipeline;
pub mod source;

#[derive(Debug, Serialize, Deserialize)]
pub enum ParsedOutput {
    Html(Pipeline<FinalizeHtml>),
    Sfc(Pipeline<SfcConversion>),
}

/// The key parsing/transform library which converts markdown into a
/// target output that the user specifies as part of their configuration.
#[instrument]
pub fn parse(id: &str, content: &str, options: &Options) -> Result<ParsedOutput, ParserError> {
    let config = Config::with_options(options);
    let mut pipeline = Pipeline::new("foobar", config);
    // load raw markdown
    match &pipeline.source {
        Source::File => pipeline.add_md_file(id),
        Source::Database => pipeline.add_md_db(id),
    }

    let mut pipeline = pipeline
        .h_raw_markdown() //
        .next_stage()?
        // ParseRawMd
        .h_mutate_markdown()
        .h_frontmatter_defaults()?
        .h_frontmatter_overrides()?
        .next_stage()?
        // Initial Darkmatter
        .lang_detection()?
        .tokenize()?
        .sentiment()?
        .complexity()?
        .ttr()?
        // .bloom()?
        .next_stage()?
        // ParseHtml
        .h_initial_darkmatter()?
        .parse_to_html()?
        .wrap_html_body()
        .next_stage()?
        // FinalizeDarkmatter
        .toc()?
        .darkmatter_metrics()
        .next_stage()?
        // FinalizeHtml
        .h_html_body()?
        .h_title()?
        .h_meta_tags()?
        .h_script_blocks()?
        .h_style_blocks()?
        .h_script_refs()?
        .h_style_refs()?
        .h_metrics()?;

    // if &config.output == Output::SFC {
    //     pipeline = ParseSfc::from(pipeline)?;
    // }

    todo!();

    // Ok(pipeline.into())
}
