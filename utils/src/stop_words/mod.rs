use std::fmt::{self, Display};

use human_regex::{exactly, one_or_more, or, punctuation, whitespace, word_boundary};
use lingua::Language as DetectLang;
use stop_words::{get, LANGUAGE as AvailLang};

pub enum LanguageChoice {
    Available(AvailLang),
    Detectable(DetectLang),
    String(&'static str),
}

impl Display for LanguageChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl LanguageChoice {
    pub fn get_stop_words(&self) -> Vec<String> {
        let lang = format!("{}", self);
        get(lang)
    }
}

pub fn extract_stop_words(text: &str, lang: LanguageChoice) -> String {
    let text = text.to_ascii_lowercase();
    let punk = one_or_more(punctuation());
    let text = punk.to_regex().replace_all(&*text, "");
    let words = lang.get_stop_words();
    let stop =
        word_boundary() + exactly(1, or(&words)) + word_boundary() + one_or_more(whitespace());
    let text = stop.to_regex().replace_all(&*text, "");

    text.to_string()
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn english_words() {
    //     let prose = "The quick brown fox jumped over the lazy dog. The quick brown fox jumped over the lazy dog.";
    //     let en = AvailLang::English;
    //     let stop = extract_stop_words(
    //         prose,
    //         en
    //     );

    //     assert_eq!(stop, String::from("quick brown fox jumped over lazy dog"))
}
