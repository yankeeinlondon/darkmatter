pub mod errors;

pub use self::errors::StopWordError;
use crate::words::Words;
use human_regex::{
    beginning_of_text, exactly, one_or_more, or, punctuation, whitespace,
    word_boundary,
};
use lingua::Language;
use tracing::instrument;

#[instrument]
fn get_stop_words_with_language(
    lang: &Language,
) -> Result<Vec<String>, StopWordError> {
    let ln = stop_words::get(lang.iso_code_639_1().to_string());
    if ln.is_empty() {
        let message = format!(
            "The language '{}'[{}] does not have any stop words associated to it (stop_words crate).",
            lang,
            lang.iso_code_639_1()
        );
        Err(StopWordError::NoStopWords(message))
    } else {
        Ok(ln)
    }
}

#[derive(Debug)]
pub struct StopWords {
    /// The ISO 639 codes for the language
    language: (String, String, String),
    /// The tokens/words remaining after extraction
    tokens: Vec<String>,
    /// The reduction in word count after extracting
    word_reduction: usize,
}

/// Utility for extracting stop words out of known languages
impl StopWords {
    /// extract stop words from a passed in corpus
    #[instrument]
    pub fn parse(corpus: &str, lang: Language) -> Result<Self, StopWordError> {
        // move to lowercase
        let corpus = corpus.to_ascii_lowercase();
        let initial_word_count = Words::parse(&corpus, None).len();

        // remove punctuation
        let punk = one_or_more(punctuation());
        let corpus = punk.to_regex().replace_all(&*corpus, "");

        // extract stop words
        let stop_words = get_stop_words_with_language(&lang)?;
        let stop = word_boundary()
            + exactly(1, or(&stop_words))
            + word_boundary()
            + one_or_more(whitespace());
        let corpus = stop.to_regex().replace_all(&*corpus, " ");

        // eliminate whitespace
        let corpus = stop.to_regex().replace_all(&*corpus, " ");
        let extra_whitespace = one_or_more(whitespace());
        let corpus = extra_whitespace.to_regex().replace_all(&*corpus, " ");
        let trimmings = beginning_of_text() + one_or_more(whitespace());
        let corpus = trimmings.to_regex().replace_all(&*corpus, "");

        let words = Words::parse(&corpus, None);

        Ok(StopWords {
            language: (
                format!("{}", &lang),
                lang.iso_code_639_1().to_string(),
                lang.iso_code_639_3().to_string(),
            ),
            tokens: words.as_tokens(),
            word_reduction: initial_word_count - words.len(),
        })
    }

    /// Get a space-separated string with all the remaining words
    /// after the stop-words have been extracted.
    #[instrument]
    pub fn as_prose(&self) -> String {
        self.tokens.join(" ")
    }

    #[instrument]
    pub fn as_tokens(&self) -> Vec<String> {
        self.tokens.clone()
    }

    /// Number of words that exist after the stemming
    /// process has completed.
    pub fn words(&self) -> usize {
        self.tokens.len()
    }

    /// The number of characters in the
    /// _prose_ export after removing stop words
    pub fn len(&self) -> usize {
        self.as_prose().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[instrument]
    pub fn language(&self) -> String {
        format!(
            "{}[{}, {}]",
            self.language.0, self.language.1, self.language.2
        )
    }

    pub fn word_reduction(&self) -> usize {
        self.word_reduction
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn english_words() {
        let prose = "The quick brown fox jumped over the lazy dog.";
        let stop = StopWords::parse(
            prose, //
            Language::English,
        )
        .expect("expected stop words to be returned");

        assert_eq!(
            stop.as_prose(),
            String::from("quick brown fox jumped lazy dog")
        )
    }

    #[test]
    fn correct_reporting_of_words_len_and_delta() {
        let prose = "The quick brown fox jumped over the lazy dog.";
        let stop = StopWords::parse(
            prose, //
            Language::English,
        )
        .expect("expected stop words to be returned");

        assert_eq!(stop.len(), 31);
        assert_eq!(stop.word_reduction(), 3);
        assert_eq!(stop.words(), 6);
    }

    #[test]
    fn spanish_words() {
        let prose = "El veloz zorro marrón saltó sobre el perezoso perro.";
        let stop = StopWords::parse(prose, Language::Spanish)
            .expect("no spanish stop-words were found!");

        assert_eq!(
            stop.as_prose(),
            String::from("veloz zorro marrón saltó perezoso perro")
        )
    }
}
