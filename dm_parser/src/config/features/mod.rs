use gray_matter::engine::Engine;
use serde::{Deserialize, Serialize};

use self::{
    code::{CodeConfig, CodeOptions},
    collapsible::{CollapsibleConfig, CollapsibleOptions},
    columns::{ColumnConfig, ColumnOptions},
    emoji::{EmojiConfig, EmojiOptions},
    frontmatter::{FrontmatterConfig, FrontmatterOptions},
    image::{ImageConfig, ImageOptions},
    inline::{InlineConfig, InlineOptions},
    link::{LinkConfig, LinkOptions},
    list::{ListConfig, ListOptions},
    markdown::{MarkdownConfig, MarkdownOptions},
    meta::{MetaConfig, MetaOptions},
    nlp::{NlpConfig, NlpOptions},
    toc::{TocConfig, TocOptions},
};

pub mod code;
pub mod collapsible;
pub mod columns;
pub mod emoji;
pub mod frontmatter;
pub mod image;
pub mod inline;
pub mod link;
pub mod list;
pub mod markdown;
pub mod meta;
pub mod nlp;
pub mod toc;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureOptions {
    /// Configuration of which NLP algorithms you would like to use
    nlp: Option<NlpOptions>,
    frontmatter: Option<FrontmatterOptions>,
    /// Options to extend the core parsers target of CommonMark standard
    /// with well known extensions. By default _all_ options are turned on.
    markdown: Option<MarkdownOptions>,

    toc: Option<TocOptions>,
    links: Option<LinkOptions>,
    meta: Option<MetaOptions>,
    code: Option<CodeOptions>,
    emoji: Option<EmojiOptions>,
    lists: Option<ListOptions>,
    images: Option<ImageOptions>,
    /// Allows configuration of inlining assets into the page. This includes:
    ///
    /// - markdown
    /// - code blocks
    /// - frontmatter
    /// - css
    inline: Option<InlineOptions>,
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
    columns: Option<ColumnOptions>,
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
    collapsible: Option<CollapsibleOptions>,
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
pub struct FeaturesConfig {
    pub links: LinkConfig,
    pub meta: MetaConfig,
    pub code: CodeConfig,
    pub emoji: EmojiConfig,
    pub lists: ListConfig,
    pub images: ImageConfig,
    pub markdown: MarkdownConfig,
    pub frontmatter: FrontmatterConfig,
    pub nlp: NlpConfig,
    /// Allows configuration of inlining assets into the page. This includes:
    ///
    /// - markdown
    /// - code blocks
    /// - frontmatter
    /// - css
    pub inline: InlineConfig,
    pub toc: TocConfig,
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
    pub columns: ColumnConfig,
    pub collapsable: CollapsibleConfig,
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
    pub enable_slots: bool,
}

impl FeaturesConfig {
    pub fn with_options(options: &FeatureOptions) -> Self {
        todo!();
    }
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        FeaturesConfig {
            markdown: MarkdownConfig::default(),
            frontmatter: FrontmatterConfig::default(),
            links: LinkConfig::default(),
            meta: MetaConfig::default(),
            code: CodeConfig::default(),
            emoji: EmojiConfig::default(),
            lists: ListConfig::default(),
            images: ImageConfig::default(),
            nlp: NlpConfig::default(),
            toc: TocConfig::default(),
            inline: InlineConfig::default(),
            columns: ColumnConfig::default(),
            collapsable: CollapsibleConfig::default(),
            enable_slots: true,
        }
    }
}
