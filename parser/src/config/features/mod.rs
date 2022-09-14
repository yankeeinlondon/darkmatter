use serde::{Deserialize, Serialize};
use serde_json::Value;

use self::{frontmatter::FrontmatterOptions, markdown::MarkdownOptions, nlp::NlpOptions};

pub mod frontmatter;
pub mod markdown;
pub mod meta;
pub mod nlp;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureOptions {
    /// Configuration of which NLP algorithms you would like to use
    nlp: Option<NlpOptions>,
    frontmatter: Option<FrontmatterOptions>,
    /// Options to extend the core parsers target of CommonMark standard
    /// with well known extensions. By default _all_ options are turned on.
    markdown: Option<MarkdownOptions>,

    links: Option<Value>,
    meta: Option<Value>,
    code: Option<Value>,
    emoji: Option<Value>,
    lists: Option<Value>,
    images: Option<Value>,
    /// Allows configuration of inlining assets into the page. This includes:
    ///
    /// - markdown
    /// - code blocks
    /// - frontmatter
    /// - css
    inline: Option<Value>,
    toc: Option<Value>,
    /// Provides configuration for enabling _column syntax_ to your markdown.
    ///
    /// ```md
    /// Allows markdown that includes column definitions like this:
    /// ::: 2 columns [40%, 60%]
    /// foo
    /// ::: next
    /// bar
    /// :::
    /// ```
    columns: Option<Value>,
    /// Allows lists to be _collapsable_ through a combination of configuration
    /// (static) and page level syntax. For instance:
    ///
    /// ```md
    /// - foo
    ///     - f1
    /// + bar
    ///     - b1
    /// * baz
    ///     - ba1
    /// ```
    ///
    /// The standard `-` symbol represents a non-collapsible item who's sub-items
    ///  will always be visible. In contrast, the `+` symbol makes the list item
    /// collapsible and sub items are expanded by default whereas the `*` symbol
    /// makes sub items default to hidden/collapsed.
    collapsible_lists: Option<Value>,
    collapsible_blocks: Option<Value>,
    /// Turns on/off the **slot** feature which allows markdown to target slots
    /// defined in a parent container (as is often found in a "layout"). Syntax looks
    /// like:
    ///
    /// ```md
    /// This is my document.
    /// :::slot sidebar { selected: "foobar" }
    /// but this goes in the sidebar
    /// ```
    ///
    /// As this example illustrates, you can pass "slot props" as well.
    enable_slots: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureConfig {
    links: Option<Value>,
    meta: Option<Value>,
    code: Option<Value>,
    emoji: Option<Value>,
    lists: Option<Value>,
    images: Option<Value>,
    nlp: Option<NlpOptions>,
    /// Allows configuration of inlining assets into the page. This includes:
    ///
    /// - markdown
    /// - code blocks
    /// - frontmatter
    /// - css
    inline: Option<Value>,
    toc: Option<Value>,
    /// Provides configuration for enabling _column syntax_ to your markdown.
    ///
    /// ```md
    /// Allows markdown that includes column definitions like this:
    /// ::: 2 columns [40%, 60%]
    /// foo
    /// ::: next
    /// bar
    /// :::
    /// ```
    columns: Option<Value>,
    collapsable_lists: Option<Value>,
    /// Turns on/off the **slot** feature which allows markdown to target slots
    /// defined in a parent container (as is often found in a "layout"). Syntax looks
    /// like:
    ///
    /// ```md
    /// This is my document.
    /// :::slot sidebar { selected: "foobar" }
    /// but this goes in the sidebar
    /// ```
    ///
    /// As this example illustrates, you can pass "slot props" as well.
    enable_slots: bool,
    md_tables: bool,
    md_tasklists: bool,
    md_smart_punctuation: bool,
    md_heading_attributes: bool,
    md_footnotes: bool,
    md_strikethrough: bool,
}
