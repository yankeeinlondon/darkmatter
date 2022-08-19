use pulldown_cmark::{html, Options, Parser};

pub mod toc;

/// parses Markdown content into a tokenized AST
#[inline]
pub fn parse(md: &str, options: Options) -> String {
    todo!();
}

// pub fn render(ast: )
