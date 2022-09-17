pub mod errors;

use crate::hasher;
use serde::{Deserialize, Serialize};
use slugify::slugify;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::html::{
    css_for_theme_with_class_style, ClassStyle, ClassedHTMLGenerator,
};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
use tracing::instrument;

use self::errors::CodeBlockError;

#[derive(Debug, Serialize, Deserialize)]
pub struct DarkModeCache {
    light: Option<String>,
    dark: Option<String>,
}

impl DarkModeCache {
    pub fn new() -> Self {
        DarkModeCache {
            light: None,
            dark: None,
        }
    }

    pub fn get(&self, dark_mode: &bool) -> &Option<String> {
        if *dark_mode {
            &self.dark
        } else {
            &self.light
        }
    }

    pub fn cache(&mut self, content: &str, dark_mode: &bool) {
        if *dark_mode {
            self.dark = Some(content.to_string())
        } else {
            self.light = Some(content.to_string())
        }
    }
}
impl Default for DarkModeCache {
    fn default() -> Self {
        DarkModeCache::new()
    }
}

thread_local! {
    static SYNTAX: SyntaxSet = SyntaxSet::load_defaults_newlines();
    static THEMES: ThemeSet = ThemeSet::load_defaults();
}

#[derive(
    Debug, Serialize, Deserialize, Display, EnumIter, PartialEq, Clone,
)]
pub enum ThemeChoices {
    InspiredGithub,
    #[serde(rename = "Solarized (dark)")]
    SolarizedDark,
    #[serde(rename = "Solarized (light)")]
    SolarizedLight,
    #[serde(rename = "base16-eighties.dark")]
    Base16EightiesDark,
    #[serde(rename = "base16-mocha.dark")]
    Base16MochaDark,
    #[serde(rename = "base16-ocean.dark")]
    Base16OceanDark,
    #[serde(rename = "base16-ocean.light")]
    Base16OceanLight,
}

/// a `Grammar` is an acronym for what Sublime Text calls a Package and
/// what Syntect calls a Language. Ultimately it represents the language
/// parsers that can be used for code blocks.
///
/// - [Packages on SublimeHQ](https://github.com/sublimehq/Packages)
/// - [Syntax Definitions](https://www.sublimetext.com/docs/syntax.html#ver-dev)
/// - [Scope Naming](https://www.sublimetext.com/docs/scope_naming.html)
/// - [Color Schemes](https://www.sublimetext.com/docs/scope_naming.html#color-schemes)
#[derive(
    Debug, Serialize, Deserialize, Display, EnumIter, PartialEq, Clone,
)]
pub enum Grammar {
    ASP,
    ActionScript,
    AppleScript,
    BatchFile,
    Binary,
    #[serde(rename = "C#")]
    CSharp,
    #[serde(rename = "C / C++")]
    C,
    CSS,
    Clojure,
    D,
    Diff,
    Erlang,
    #[serde(rename = "Git Formats")]
    GitFormats,
    Go,
    Graphviz,
    Groovy,
    HTML,
    #[serde(rename = "HTML (TCL)")]
    HtmlTCL,
    #[serde(rename = "HTML (ASP)")]
    HtmlASP,
    #[serde(rename = "HTML (Erlang)")]
    HtmlErlang,
    #[serde(rename = "HTML (Rails)")]
    HtmlRails,
    Haskell,
    JSON,
    Java,
    JavaScript,
    LaTeX,
    Lisp,
    Lua,
    Makefile,
    Markdown,
    MultiMarkdown,
    Matlab,
    OCaml,
    #[serde(rename = "Objective-C")]
    ObjectiveC,
    PHP,
    // Pascal,
    Perl,
    Python,
    R,
    Rails,
    #[serde(rename = "Regular Expression")]
    RegExp,
    #[serde(rename = "reStructuredText")]
    RestructuredText,
    Ruby,
    #[serde(rename = "Ruby on Rails")]
    RubyOnRails,
    Rust,
    SQL,
    #[serde(rename = "SQL (Rails)")]
    SqlRails,
    Scala,
    #[serde(rename = "Bourne Again Shell (bash)")]
    Bash,
    ShellScript,
    TCL,
    Text,
    Textile,
    XML,
    YAML,
}

impl Grammar {
    /// Gets a language grammar from a string representation of the
    /// language. The matching is case insensitive and also maps some
    /// common abbreviations to the formal name.
    #[instrument(level = "info")]
    pub fn get(language: &str) -> Option<Grammar> {
        let language = language.to_string().to_lowercase();
        for grammar in Grammar::iter() {
            let lc = format!("{}", grammar).to_lowercase();
            if lc.eq(&language) {
                return Some(grammar);
            }
        }
        let mut abbr: HashMap<&str, Grammar> = HashMap::new();
        // shorthands
        abbr.insert("js", Grammar::JavaScript);
        abbr.insert("ts", Grammar::JavaScript);
        abbr.insert("typescript", Grammar::JavaScript);
        abbr.insert("cpp", Grammar::C);
        abbr.insert("c++", Grammar::C);
        abbr.insert("md", Grammar::Markdown);
        abbr.insert("mdx", Grammar::Markdown);
        // shell scripts
        abbr.insert("sh", Grammar::ShellScript);
        abbr.insert("bash", Grammar::ShellScript);
        abbr.insert("zsh", Grammar::ShellScript);
        abbr.insert("nsh", Grammar::ShellScript);
        // regular expressions
        abbr.insert("regex", Grammar::RegExp);
        abbr.insert("regexp", Grammar::RegExp);
        abbr.insert("re", Grammar::RegExp);

        // check against abbreviations
        let grammar = abbr.get(language.as_str());
        if let Some(grammar) = grammar {
            let grammar = grammar.clone();
            return Some(grammar);
        }

        None
    }

    /// outputs the grammar as a `SyntaxReference` which **syntect** expects
    #[instrument(level = "info")]
    pub fn as_syntax_ref(&self) -> SyntaxReference {
        SYNTAX.with(|s| {
            let language = format!("{}", &self);
            let syntax = s.find_syntax_by_name(&language).unwrap().clone();
            return syntax;
        })
    }
}

impl From<Grammar> for String {
    fn from(grammar: Grammar) -> String {
        format!("{}", grammar)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CssInfo {
    pub filename: String,
    pub css: String,
    pub theme: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeBlock {
    code: String,
    /// a representative filename of the code block
    pub filename: Option<String>,
    /// The language the code block is expected to be
    pub language: Option<String>,
    pub hash: u64,
    theme: (Theme, Theme),
    console_output: DarkModeCache,
    html: DarkModeCache,
}

impl CodeBlock {
    pub fn syntaxes() -> Vec<String> {
        SyntaxSet::load_defaults_newlines()
            .syntaxes()
            .into_iter()
            .map(|i| i.name.to_string())
            .collect()
    }

    pub fn new(code: &str) -> Self {
        let light = THEMES.with(|t| t.themes["base16-ocean.light"].clone());
        let dark = THEMES.with(|t| t.themes["base16-ocean.dark"].clone());

        CodeBlock {
            code: code.to_string(),
            theme: (light, dark),
            filename: None,
            language: None,
            hash: hasher::hash(code, None),
            console_output: DarkModeCache::new(),
            html: DarkModeCache::new(),
        }
    }

    pub fn theme(&self, dark_mode: &bool) -> Theme {
        if *dark_mode {
            self.theme.1.clone()
        } else {
            self.theme.0.clone()
        }
    }

    pub fn theme_options(&self) -> Vec<String> {
        THEMES.with(|t| {
            let mut options: Vec<String> = vec![];
            t.themes.iter().for_each(|(o, _)| {
                options.push(o.clone());
            });

            options
        })
    }

    /// Creates a new `CodeBlock` along with an explicit
    /// language definition. Throws `CodeBlockError::InvalidLanguage` if
    /// language is not recognized.
    pub fn new_with_lang(
        code: &str,
        lang: &str,
    ) -> Result<Self, CodeBlockError> {
        let grammar = Grammar::get(&lang);
        match grammar {
            Some(_) => {
                let mut cb = CodeBlock::new(code);
                cb.language = Some(lang.to_string());
                Ok(cb)
            }
            None => Err(CodeBlockError::InvalidLanguage(lang.to_string())),
        }
    }

    pub fn new_with_filename(code: &str, filename: &str) -> Self {
        let mut cb = CodeBlock::new(code);
        cb.filename = Some(filename.to_string());

        cb
    }

    /// Will return the file extension if it can be discerned from
    /// the "filename".
    #[instrument()]
    pub fn file_extension(&self) -> Option<&str> {
        match &self.filename {
            Some(filename) => filename.split('.').last(),
            None => None,
        }
    }

    /// Mutates the contained code block while updating hash
    /// in the background.
    #[instrument()]
    pub fn mutate(&mut self, code: &str) {
        self.hash = hasher::hash(code, None);
        self.console_output = DarkModeCache::new();
        self.html = DarkModeCache::new();
        self.code = code.to_string();
    }

    /// provides the code in an _unformatted_ form
    #[instrument()]
    pub fn unformatted_code(&self) -> String {
        self.code.clone()
    }

    /// Returns the expected language grammar based on any/all hints available
    pub fn determine_grammar(&self) -> Option<SyntaxReference> {
        if let Some(language) = &self.language {
            if let Some(grammar) = Grammar::get(language) {
                return Some(grammar.as_syntax_ref());
            }
        };

        if let Some(filename) = &self.filename {
            let extension = filename.split('.').last();
            if let Some(extension) = extension {
                let grammar = SYNTAX.with(|s| {
                    let grammar = s.find_syntax_by_extension(extension);
                    if let Some(grammar) = grammar {
                        let grammar = grammar.clone();
                        Some(grammar)
                    } else {
                        None
                    }
                });
                return grammar;
            }
        };

        None
    }

    /// provides the code with color codes embedded which are
    /// designed for the console.
    #[instrument()]
    pub fn as_escaped_console(&mut self, dark_mode: &bool) -> String {
        if let Some(cache) = &self.console_output.get(&dark_mode) {
            return cache.to_string();
        }

        let grammar = self.determine_grammar();
        let ps = SYNTAX.with(|s| s.clone());
        let mut code_lines: String = "".to_string();
        let theme = if *dark_mode {
            &self.theme.1
        } else {
            &self.theme.0
        };
        if let Some(grammar) = grammar {
            let mut h = HighlightLines::new(&grammar, &theme);
            for line in LinesWithEndings::from(&self.code) {
                let ranges: Vec<(Style, &str)> =
                    h.highlight_line(line, &ps).unwrap();
                let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                code_lines = [code_lines, escaped].join("\n").to_string();
            }
        }
        // cache
        if *dark_mode {
            self.console_output.dark = Some(code_lines.clone());
        } else {
            self.console_output.light = Some(code_lines.clone());
        }

        code_lines
    }

    #[instrument()]
    pub fn get_css_info(
        &self,
        dark_mode: &bool,
    ) -> Result<CssInfo, CodeBlockError> {
        let theme = self.theme(dark_mode);
        let theme_name = &theme
            .name
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let theme_name = slugify!(theme_name);
        let filename = [&theme_name, ".css"].join("");
        let css = css_for_theme_with_class_style(&theme, ClassStyle::Spaced)?;

        Ok(CssInfo {
            css,
            filename,
            theme: theme_name,
        })
    }

    /// provides the code as a tokenized HTML string
    #[instrument()]
    pub fn as_html(
        &mut self,
        dark_mode: &bool,
    ) -> Result<String, CodeBlockError> {
        if let Some(html) = &self.html.get(&dark_mode) {
            return Ok(html.to_string());
        }

        let grammar = self.determine_grammar();
        let ps = SYNTAX.with(|s| s.clone());
        let mut code_lines: String = "".to_string();
        if let Some(grammar) = grammar {
            let mut html = ClassedHTMLGenerator::new_with_class_style(
                &grammar,
                &ps,
                ClassStyle::Spaced,
            );

            for line in LinesWithEndings::from(&self.code) {
                html.parse_html_for_line_which_includes_newline(line)?;
            }
            code_lines = html.finalize();
        }

        self.html.cache(&code_lines, &dark_mode);

        Ok(code_lines)
    }

    pub fn as_html_with_inline_style(
        &mut self,
        dark_mode: bool,
    ) -> Result<String, CodeBlockError> {
        let html = self.as_html(&dark_mode)?;
        let info = self.get_css_info(&dark_mode)?;

        Ok(["<style>", &info.css, "</style>", &html]
            .join("\n")
            .to_string())
    }

    pub fn as_html_with_css_ref(
        &mut self,
        dark_mode: bool,
    ) -> Result<String, CodeBlockError> {
        let html = self.as_html(&dark_mode)?;
        let info = self.get_css_info(&dark_mode)?;

        Ok(
            [
                "<link rel=\"stylesheet\" type=\"text/css\" rel=\"noopener\" target=\"_blank\" href=\"", 
                &info.filename, "\">", 
                &html
            ].join("\n").to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_grammar_get() {
        let exact = ["ASP", "Erlang", "RestructuredText"];
        let wrong_caps = ["binary", "go"];
        let shorthand = ["c++", "ts", "js", "regex", "regexp"];

        for t in exact {
            assert!(Grammar::get(t).is_some());
        }
        for t in wrong_caps {
            assert!(Grammar::get(t).is_some());
        }
        for t in shorthand {
            assert!(Grammar::get(t).is_some());
        }
    }

    #[test]
    fn code_themes_include() {
        let options = CodeBlock::new("").theme_options();
        // println!("{:?}", &options);
        assert!(&options.contains(&"base16-ocean.light".to_string()));
        assert!(&options.contains(&"base16-ocean.dark".to_string()));
    }

    #[test]
    fn code_block_extracts_extension_where_available() {
        let js = CodeBlock::new_with_filename("", "foobar.js");
        let ts = CodeBlock::new_with_filename("", "foobar.ts");
        let complex = CodeBlock::new_with_filename("", "foobar.one.two.ts");

        assert!(&js.file_extension().is_some());
        assert!(&ts.file_extension().is_some());
        assert!(&complex.file_extension().is_some());
        if let Some(js) = js.file_extension() {
            assert_eq!(js, "js");
        }
        if let Some(ts) = ts.file_extension() {
            assert_eq!(ts, "ts");
        }
        if let Some(complex) = complex.file_extension() {
            assert_eq!(complex, "ts");
        }
    }

    #[test]
    fn code_grammar_determined_on_extension() {
        let js = CodeBlock::new_with_filename("", "js");
        // let ts = CodeBlock::new_with_filename("", "ts");
        let rs = CodeBlock::new_with_filename("", "rs");
        let md = CodeBlock::new_with_filename("", "md");
        let php = CodeBlock::new_with_filename("", "php");
        let c = CodeBlock::new_with_filename("", "c");
        let cpp = CodeBlock::new_with_filename("", "cpp");
        let cc = CodeBlock::new_with_filename("", "cc");
        let css = CodeBlock::new_with_filename("", "css");
        let html = CodeBlock::new_with_filename("", "html");
        let py = CodeBlock::new_with_filename("", "py");

        assert!(js.determine_grammar().is_some());
        assert!(rs.determine_grammar().is_some());
        // assert!(ts.grammar().is_some());
        assert!(md.determine_grammar().is_some());
        assert!(php.determine_grammar().is_some());
        assert!(c.determine_grammar().is_some());
        assert!(cpp.determine_grammar().is_some());
        assert!(cc.determine_grammar().is_some());
        assert!(css.determine_grammar().is_some());
        assert!(html.determine_grammar().is_some());
        assert!(py.determine_grammar().is_some());
    }

    #[test]
    fn code_grammar_determined_on_name() {
        let javascript = CodeBlock::new_with_lang("", "JavaScript")
            .unwrap()
            .determine_grammar();
        let typescript = CodeBlock::new_with_lang("", "Typescript")
            .unwrap()
            .determine_grammar();
        let rust = CodeBlock::new_with_lang("", "rust")
            .unwrap()
            .determine_grammar();
        let php = CodeBlock::new_with_lang("", "PHP")
            .unwrap()
            .determine_grammar();
        let markdown = CodeBlock::new_with_lang("", "Markdown")
            .unwrap()
            .determine_grammar();
        let c = CodeBlock::new_with_lang("", "C")
            .unwrap()
            .determine_grammar();
        let perl = CodeBlock::new_with_lang("", "perl")
            .unwrap()
            .determine_grammar();
        let go = CodeBlock::new_with_lang("", "go")
            .unwrap()
            .determine_grammar();

        assert!(javascript.is_some());
        assert!(typescript.is_some());
        assert!(rust.is_some());
        assert!(php.is_some());
        assert!(markdown.is_some());
        assert!(c.is_some());
        assert!(go.is_some());
        assert!(perl.is_some());
    }

    #[test]
    fn code_escaped_for_console() {
        let code = "let foo: number = 42";
        let mut block = CodeBlock::new_with_lang(code, "Typescript").unwrap();
        let dark = block.as_escaped_console(&true);
        assert_eq!(dark, "\n\u{1b}[48;2;43;48;59m\u{1b}[38;2;180;142;173mlet\u{1b}[48;2;43;48;59m\u{1b}[38;2;192;197;206m \u{1b}[48;2;43;48;59m\u{1b}[38;2;191;97;106mfoo\u{1b}[48;2;43;48;59m\u{1b}[38;2;192;197;206m: \u{1b}[48;2;43;48;59m\u{1b}[38;2;191;97;106mnumber\u{1b}[48;2;43;48;59m\u{1b}[38;2;192;197;206m \u{1b}[48;2;43;48;59m\u{1b}[38;2;192;197;206m=\u{1b}[48;2;43;48;59m\u{1b}[38;2;192;197;206m \u{1b}[48;2;43;48;59m\u{1b}[38;2;208;135;112m42".to_string());
    }

    #[test]
    fn code_for_console_distinct_for_light_v_dark_mode() {
        let code = "let foo: number = 42";
        let mut block = CodeBlock::new_with_lang(code, "Typescript").unwrap();
        let light = block.as_escaped_console(&false);
        let dark = block.as_escaped_console(&true);
        let light2 = block.as_escaped_console(&false); // should use cache

        assert_ne!(light, dark);
        assert_eq!(light, light2);
    }

    #[test]
    fn code_converts_to_html() {
        let code = "let foo: number = 42";
        let mut block = CodeBlock::new_with_lang(code, "Typescript").unwrap();
        let html = block.as_html(&true).unwrap();
        assert!(html.contains("meta.link {\n color: #d08770;\n}"));
    }

    #[test]
    fn code_provides_css_for_styling() {
        let code = "let foo: number = 42";
        let block = CodeBlock::new(code);
        let css = block.get_css_info(&true).unwrap();

        assert!(css
            .css
            .contains("meta.link {\n color: #d08770;\n}"));
        assert_eq!(css.filename, "base16-ocean-dark.css")
    }
}
