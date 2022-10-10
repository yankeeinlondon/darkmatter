use config::{Config, Options};
use errors::parser_err::ParserError;
use pipeline::Pipeline;
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
    Temp, // Html(Pipeline<FinalizeHtml>),
          // Sfc(Pipeline<SfcConversion>),
}

/// The key parsing/transform library which converts markdown into a
/// target output that the user specifies as part of their configuration.
#[instrument]
pub fn parse(id: &str, content: &str, options: &Options) -> Result<ParsedOutput, ParserError> {
    let pipeline = Pipeline::new(
        "foobar", //
        Config::with_options(options),
    )
    .load_content()?
    .h_raw_markdown()?;

    // let mut p2 = Pipeline::parse_raw_md(pipeline)?.h_mutate_markdown();

    // .next_stage()?;

    // .h_mutate_markdown()?
    // .h_frontmatter_defaults()?
    // .h_frontmatter_overrides()?;
    // .expand_shortcodes()?;
    // .next_stage()?
    // // Initial Darkmatter
    // .lang_detection()?
    // .tokenize()?
    // .sentiment()?
    // .complexity()?
    // .ttr()?
    // // .bloom()?
    // .next_stage()?
    // // ParseHtml
    // .h_initial_darkmatter()?
    // .parse_to_html()?
    // .wrap_html_body()
    // .next_stage()?
    // // FinalizeDarkmatter
    // .toc()?
    // .darkmatter_metrics()
    // .next_stage()?
    // // FinalizeHtml
    // .h_html_body()?
    // .h_title()?
    // .h_meta_tags()?
    // .h_script_blocks()?
    // .h_style_blocks()?
    // .h_script_refs()?
    // .h_style_refs()?
    // .h_metrics()?;

    // if &config.output == Output::SFC {
    //     pipeline = ParseSfc::from(pipeline)?;
    // }

    todo!();

    // Ok(pipeline.into())
}
