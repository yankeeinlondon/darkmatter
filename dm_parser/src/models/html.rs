use pulldown_cmark::{html::push_html, Options as ParserOptions, Parser};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::context::BaseContext;

#[derive(Error, Debug)]
pub enum HtmlError {}

fn get_parser_options(ctx: &BaseContext) -> ParserOptions {
    let mut options = ParserOptions::empty();
    let config = &ctx.config.features.markdown;
    if config.footnotes {
        options.insert(ParserOptions::ENABLE_FOOTNOTES);
    }
    if config.heading_attributes {
        options.insert(ParserOptions::ENABLE_HEADING_ATTRIBUTES);
    }
    if config.smart_punctuation {
        options.insert(ParserOptions::ENABLE_SMART_PUNCTUATION);
    }
    if config.strikethrough {
        options.insert(ParserOptions::ENABLE_STRIKETHROUGH);
    }
    if config.tables {
        options.insert(ParserOptions::ENABLE_TABLES);
    }
    if config.tasklists {
        options.insert(ParserOptions::ENABLE_TASKLISTS);
    }

    options
}

/// a string which represents HTML content
#[derive(Debug, Serialize, Deserialize)]
pub struct HtmlContent {
    html: String,
    hash_initial: u64,
}

impl From<&BaseContext> for HtmlContent {
    /// Generate HTML from a base context
    fn from(ctx: &BaseContext) -> Self {
        let parser_options = get_parser_options(ctx);
        let md = ctx.markdown.content();
        let parser = Parser::new_ext(&md, parser_options);
        let mut html = String::new();
        push_html(&mut html, parser);
        let hash_initial = dm_utils::hash(&html, None);

        HtmlContent { html, hash_initial }
    }
}

impl HtmlContent {
    /// Create structure directly from HTML string content
    pub fn new(content: &str) -> Self {
        let html = content.to_string();
        let hash_initial = dm_utils::hash(&html, None);

        HtmlContent { html, hash_initial }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::Config,
        models::{
            frontmatter::Frontmatter,
            markdown::{MarkdownContent, MarkdownContentRaw},
        },
    };

    use super::*;

    #[test]
    fn html_from_ctx() {
        let markdown = MarkdownContentRaw::new(
            "Hello world, this is a ~~complicated~~ *very simple* example.",
        );
        let expected_html =
            "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";

        let config = Config::default();
        let (markdown, frontmatter) = markdown.parse(&config).unwrap();
        let ctx = BaseContext::new("test", "test", &frontmatter, &markdown, &None, &config);

        let html = HtmlContent::from(&ctx).html;

        assert_eq!(expected_html, &html);
    }
}
