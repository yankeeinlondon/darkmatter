extern crate rust_stemmers;
use lingua::Language;
use rust_stemmers::{Algorithm, Stemmer};

/// Gets an Stemmer algorithm if available for the
/// given language
fn get_algorithm(lang: &Language) -> Result<Algorithm, StemmingErrors> {
    let with_stemmers: Map<Language, Algorithm> = Map.new();

    [
        (Language::Arabic, Algorithm::Arabic),
        (Language::Danish, Algorithm::Danish),
        (Language::Dutch, Algorithm::Dutch),
        (Language::English, Algorithm::English),
        (Language::French, Algorithm::French),
    ];
}

/// Provides stemming on the text corpus to reduce
/// words with the same basic meaning into a concentrated
pub fn stem(corpus: &str, lang: &Language) {
    todo!();
}
