use pulldown_cmark::{html::push_html, Options as ParserOptions, Parser};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    errors::parser_err::ParserError,
    pipeline::{parse_html::ParseHtml, Pipeline},
};

mod traits;

#[derive(Error, Debug)]
pub enum HtmlError {}

fn get_parser_options(ctx: &Pipeline<ParseHtml>) -> ParserOptions {
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
    pub html: String,
    pub max_nesting: i32,
    pub hash_initial: u64,
}

/// Use the context provided by a Pipeline<ParseHtml> to
/// parse the HTML and produce a `HtmlContent` structure.
impl TryFrom<&Pipeline<ParseHtml>> for HtmlContent {
    type Error = ParserError;
    /// Generate HTML from the available pipeline content
    fn try_from(ctx: &Pipeline<ParseHtml>) -> Result<Self, ParserError> {
        let parser_options = get_parser_options(ctx);
        let md = ctx.markdown.content();
        let parser = Parser::new_ext(&md, parser_options);
        let mut max_nesting = 0;
        let mut level = 0;
        // let parser = parser.map(|event| match event {
        //     Event::Start(_) => {
        //         level += 1;
        //         max_nesting = std::cmp::max(max_nesting, level);
        //     }
        //     Event::End(_) => level -= 1,
        //     _ => (),
        // });

        let mut html = String::new();
        push_html(&mut html, parser);
        let hash_initial = dm_utils::hash(&html, None);

        Ok(HtmlContent {
            html,
            max_nesting,
            hash_initial,
        })
    }
}

impl HtmlContent {
    /// Create structure directly from HTML string content
    pub fn new(content: &str) -> Self {
        let html = content.to_string();
        let hash_initial = dm_utils::hash(&html, None);

        HtmlContent {
            html,
            max_nesting: 0,
            hash_initial,
        }
    }

    pub fn content(&self) -> String {
        self.html.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::Config,
        errors::parser_err::ParserError,
        models::markdown::{MarkdownContent, MarkdownContentRaw},
        pipeline::initialize::Initialize,
    };

    /// an initialized Pipeline can use `try_from` to move to parsing stage
    #[test]
    fn html_parse_try_from() {
        let init = Pipeline::new("foobar", Config::default());
        let p = Pipeline::try_from(init);

        assert!(p.is_ok());
    }

    #[test]
    fn html_parse_into() {
        let init = Initialize::new("foobar", Config::default());
        let p: Result<Pipeline<ParseHtml>, ParserError> = init.try_into();

        assert!(p.is_ok());
    }

    #[test]
    fn html_parse_base_test() {
        let markdown = MarkdownContentRaw::new(
            "Hello world, this is a ~~complicated~~ *very simple* example.",
        );
        let expected_html =
            "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
        let config = Config::default();
        let (md, fm) = markdown.parse(&config).unwrap();
        let ctx = ParseHtml::new("foobar", config, md, fm);

        ctx.parse_html();

        let html = ctx.0.html;

        assert_eq!(&ctx.0.html.content(), &expected_html);
    }

    fn html_max_nesting() {
        let (markdown, frontmatter) =
            MarkdownContent::from_file("test/fixtures/structures.md", &Config::default()).unwrap();
        let ctx = BaseContext::new(
            "test",
            "test",
            &frontmatter,
            &markdown,
            &None,
            &Config::default(),
        );
        let html = HtmlContent::from(&ctx);

        assert_eq!(html.max_nesting, 5);
    }
}
