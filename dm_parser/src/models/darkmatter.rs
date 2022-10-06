use super::{content_type::ContentType, sentiment::Sentiment, toc::TocItem};
use core::fmt::Debug;
use lingua::Language;
use serde::{Deserialize, Serialize};

pub trait DmStage: std::fmt::Debug {
    /// The hash of the table of contents of the page/post
    type TocHash;
    /// Max nesting level of a page; set to `false` prior to being calculated
    type MaxNesting;
    type TOC;
    /// "Time to Read" calculation; set to `false` prior to being calculated
    type TTR;
    type SENT;
    type IMG;
    type Links;
}

/// Types for Darkmatter in the initial gathering period
#[derive(Serialize, Deserialize, Debug)]
pub struct DmInitial;
impl DmStage for DmInitial {
    type TocHash = bool;
    type TOC = bool;
    type MaxNesting = bool;
    type TTR = bool;
    type SENT = bool;
    type IMG = bool;
    type Links = bool;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DmWhileParsing;
impl DmStage for DmWhileParsing {
    type TocHash = Option<u64>;
    type TOC = Option<TocItem>;
    type MaxNesting = Option<i32>;
    type TTR = bool;
    type SENT = bool;
    type IMG = Vec<String>;
    type Links = Vec<String>;
}

/// Finalized Darkmatter types
#[derive(Serialize, Deserialize, Debug)]
pub struct DmFinal;
impl DmStage for DmFinal {
    type TocHash = u64;
    type TOC = TocItem;
    type MaxNesting = i32;
    type TTR = u8;
    type SENT = Option<Sentiment>;
    type IMG = Vec<String>;
    type Links = Vec<String>;
}

#[derive(Serialize, Deserialize)]
pub struct Darkmatter<T>
where
    T: DmStage,
{
    /// The content-type for this document
    content_type: ContentType,
    /// The word_tokens generated by eliminating stop words for the language
    /// and then stemming. This is only available if configured for.
    word_tokens: Vec<String>,
    /// The assumed language for the content. This will come from language
    /// detection features if configured but can also come from a default
    /// language set in the configuration.
    language: Option<Language>,
    /// The very simple (and fast) estimated time to read (in minutes)
    time_to_read: u8,
    /// The complexity of the reading material based on the
    /// [Flesch Kincaid](https://en.wikipedia.org/wiki/Flesch%E2%80%93Kincaid_readability_tests)
    /// readability test.
    complexity: Option<f64>,
    /// The pos/neg sentiment detected in the document
    sentiment: Option<f64>,

    /// The maximum depth/nesting level the page goes to; this becomes available
    /// only after HTML has been parsed.
    max_nesting: T::MaxNesting,

    /// A haTocHash of the table-of-contents which indicates whether
    /// the structure of the document has changed. Available after
    /// HTML has been parsed.
    toc_hash: T::TocHash,

    /// A hierarchical representation of the table of contents
    /// defined by H1..H6. This becomes available after parsing HTML.
    toc: T::TOC,

    /// A list of all the image references on the page along with validity
    /// checking of the links (if configured to do so).
    images: T::IMG,

    /// a list of all the link references on the page along with the validity
    /// of the link (if configured to do so).
    links: T::Links,
}

impl<T> std::fmt::Debug for Darkmatter<T>
where
    T: DmStage,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Darkmatter")
            .field("content_type", &self.content_type)
            .field("word_tokens", &self.word_tokens)
            .field("language", &self.language)
            .field("time_to_read", &self.time_to_read)
            .field("complexity", &self.complexity)
            .field("sentiment", &self.sentiment)
            .field("max_nesting", &"max_nesting")
            .field("toc_hash", &"toc_hash")
            .field("toc", &"toc")
            .field("images", &"images")
            .field("links", &"links")
            .finish()
    }
}

const DEFAULT_IMAGE_REFS: usize = 5;
const DEFAULT_LINK_REFS: usize = 8;

impl Default for Darkmatter<DmWhileParsing> {
    fn default() -> Self {
        Darkmatter {
            content_type: ContentType::default(),
            word_tokens: vec![],
            language: None,
            time_to_read: 0,
            sentiment: None,
            complexity: None,
            max_nesting: None,
            toc_hash: None,
            toc: None,
            images: Vec::with_capacity(DEFAULT_IMAGE_REFS),
            links: Vec::with_capacity(DEFAULT_LINK_REFS),
        }
    }
}
