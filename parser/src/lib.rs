use content::toc::TocItem;

pub mod content;
pub mod meta;

pub struct LangDetectConfig {
    languages: Vec<String>,
}

pub enum UseLangDetection {
    No(),
    SwitchedOnDefaults(bool),
    Configured(),
}

pub struct Options {
    /// whether to have language detection run over the
    /// content of the Markdown.
    use_lang_detection: UseLangDetection,
}

pub struct Frontmatter {
    title: Option<String>,
    description: Option<String>,
}

pub struct Darkmatter {
    /// The uniqueness hash for full content of the page
    hash: String,
    /// A hash of the table-of-contents which indicates whether
    /// the structure of the document has changed
    structure_hash: String,
    /// The maximum depth/nesting level the page goes to
    max_nesting: usize,
    /// Results from language detection
    // language: Language,
    /// The estimated time to read (in minutes)
    time_to_read: Option<u8>,
    /// The Table of Contents of the page
    toc: TocItem,
}

/// Transforms the raw text content into both:
///
/// 1. Meta data (frontmatter + darkmatter)
/// 2. HTML (in SFC format or raw HTML)
pub fn transform() {
    todo!();
}
