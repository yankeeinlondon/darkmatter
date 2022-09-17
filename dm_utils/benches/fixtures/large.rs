use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum LineClassStrategy {
    None,
    Basic,
    Full,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeOptions {
    /// The _language_ to use for code blocks when no language
    /// choice is unknown.
    ///
    /// @default "plain"
    default_language_for_unknown: Option<String>,
    /// The _language_ to use for code blocks when no language
    /// is specified.
    ///
    /// @default "plain"
    default_language_for_unspecified: Option<String>,

    /// The _language_ to use for code blocks in an _inline_ code
    /// segment and no language has been specified (or it is not recognized).
    ///
    /// @default "markdown"
    default_language_for_inline: Option<String>,

    /// Every line in a code block will be wrapped by an HTML tag
    /// and to provide style utility we can apply a set of classes.
    ///
    /// Every line will have the `line` class applied unless the
    /// `LineClassStrategy::None` variant is chosen. The other two
    /// options break down as:
    ///
    /// - **basic** - provides `line`, `line-#`, `odd` or `even` classes
    /// - **full** - provides all the classes from "basic" but adds in classes
    /// to indicate the kind of tokens which are found on the line.
    ///
    /// Note: if you need more than this then checkout the `code_line` hook.
    line_class_strategy: Option<LineClassStrategy>,

    allow_code_block_heading: Option<bool>,
    allow_code_block_footer: Option<bool>,
    /// Allows to turn on/off the feature of highlighting lines in code;
    /// this is just a "default" as individual code blocks can explicitly
    /// ask for line numbers with the `#` modifier
    ///
    /// @default false
    line_numbers: Option<bool>,

    /// Flag indicating whether to display the language name in the upper right
    /// of the code block.
    ///
    /// @default true
    show_language: Option<bool>,

    /// Flag indicating whether a clipboard icon
    show_clipboard: Option<bool>,

    /// Allows to turn on/off the feature of _highlighting_ lines in code;
    /// lines will never be highlighted unless the page has instructions to
    /// highlight particular lines but this allows all highlights to be
    /// explicitly turned off
    highlight_lines: Option<bool>,

    /// Allows user to choose a code theme suitable for light mode
    /// in the browser.
    theme_light: Option<String>,
    /// Allows user to choose a code theme suitable for dark mode
    /// in the browser.
    theme_dark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeConfig {
    /// The _language_ to use for code blocks when no language
    /// choice is unknown.
    ///
    /// @default "plain"
    default_language_for_unknown: String,
    /// The _language_ to use for code blocks when no language
    /// is specified.
    ///
    /// @default "plain"
    default_language_for_unspecified: String,

    /// The _language_ to use for code blocks in an _inline_ code
    /// segment and no language has been specified (or it is not recognized).
    ///
    /// @default "markdown"
    default_language_for_inline: String,

    /// Every line in a code block will be wrapped by an HTML tag
    /// and to provide style utility we can apply a set of classes.
    ///
    /// Every line will have the `line` class applied unless the
    /// `LineClassStrategy::None` variant is chosen. The other two
    /// options break down as:
    ///
    /// - **basic** - provides `line`, `line-#`, `odd` or `even` classes
    /// - **full** - provides all the classes from "basic" but adds in classes
    /// to indicate the kind of tokens which are found on the line.
    ///
    /// Note: if you need more than this then checkout the `code_line` hook.
    line_class_strategy: LineClassStrategy,

    allow_code_block_heading: bool,
    allow_code_block_footer: bool,
    /// Allows to turn on/off the feature of highlighting lines in code;
    /// this is just a "default" as individual code blocks can explicitly
    /// ask for line numbers with the `#` modifier
    ///
    /// @default false
    line_numbers: bool,

    /// Flag indicating whether to display the language name in the upper right
    /// of the code block.
    ///
    /// @default true
    show_language: bool,

    /// Flag indicating whether a clipboard icon
    show_clipboard: bool,

    /// Allows to turn on/off the feature of _highlighting_ lines in code;
    /// lines will never be highlighted unless the page has instructions to
    /// highlight particular lines but this allows all highlights to be
    /// explicitly turned off
    highlight_lines: bool,

    /// Allows user to choose a code theme suitable for light mode
    /// in the browser.
    theme_light: String,
    /// Allows user to choose a code theme suitable for dark mode
    /// in the browser.
    theme_dark: String,
}

impl CodeConfig {
    pub fn default() -> Self {
        CodeConfig {
            default_language_for_unknown: String::from("plain"),
            default_language_for_unspecified: String::from("plain"),
            default_language_for_inline: String::from("markdown"),
            line_class_strategy: LineClassStrategy::Basic,
            allow_code_block_heading: true,
            allow_code_block_footer: true,
            line_numbers: false,
            show_language: true,
            show_clipboard: false,
            highlight_lines: true,
            theme_light: String::from(""),
            theme_dark: String::from(""),
        }
    }

    pub fn with_options(options: CodeOptions) -> Self {
        let mut config = CodeConfig::default();

        if let Some(default_language_for_unknown) =
            options.default_language_for_unknown
        {
            config.default_language_for_unknown = default_language_for_unknown;
        }
        if let Some(default_language_for_unspecified) =
            options.default_language_for_unspecified
        {
            config.default_language_for_unspecified =
                default_language_for_unspecified;
        }
        if let Some(default_language_for_inline) =
            options.default_language_for_inline
        {
            config.default_language_for_inline = default_language_for_inline;
        }
        if let Some(line_class_strategy) = options.line_class_strategy {
            config.line_class_strategy = line_class_strategy;
        }
        if let Some(allow_code_block_heading) = options.allow_code_block_heading
        {
            config.allow_code_block_heading = allow_code_block_heading;
        }
        if let Some(allow_code_block_footer) = options.allow_code_block_footer {
            config.allow_code_block_footer = allow_code_block_footer;
        }
        if let Some(line_numbers) = options.line_numbers {
            config.line_numbers = line_numbers;
        }
        if let Some(show_language) = options.show_language {
            config.show_language = show_language;
        }
        if let Some(show_clipboard) = options.show_clipboard {
            config.show_clipboard = show_clipboard;
        }
        if let Some(highlight_lines) = options.highlight_lines {
            config.highlight_lines = highlight_lines;
        }
        if let Some(line_numbers) = options.line_numbers {
            config.line_numbers = line_numbers;
        }
        if let Some(theme_light) = options.theme_light {
            config.theme_light = theme_light;
        }
        if let Some(theme_dark) = options.theme_dark {
            config.theme_dark = theme_dark;
        }

        config
    }
}
