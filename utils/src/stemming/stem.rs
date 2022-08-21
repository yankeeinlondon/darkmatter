extern crate rust_stemmers;

use lingua::Language;
use rust_stemmers::{Algorithm, Stemmer};
use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap};

use super::StemmingError;

const STEM_ALGO_COUNT: usize = 17;

fn find_algo<'a>(lang: &'a Language) -> Result<&'a Algorithm, StemmingError> {
    thread_local! {
        static WITH_STEMMERS: RefCell<HashMap<Language, Algorithm>> = RefCell::new(HashMap::with_capacity(STEM_ALGO_COUNT));
    }

    let hash = WITH_STEMMERS.with(|w| {
        let lookup = &mut *w.borrow_mut();
        if lookup.is_empty() {
            lookup.insert(Language::Arabic, Algorithm::Arabic);
            lookup.insert(Language::Danish, Algorithm::Danish);
            lookup.insert(Language::Dutch, Algorithm::Dutch);
            lookup.insert(Language::English, Algorithm::English);
            lookup.insert(Language::French, Algorithm::French);
            lookup.insert(Language::German, Algorithm::German);
            lookup.insert(Language::Greek, Algorithm::Greek);
            lookup.insert(Language::Hungarian, Algorithm::Hungarian);
            lookup.insert(Language::Italian, Algorithm::Italian);
            lookup.insert(Language::Nynorsk, Algorithm::Norwegian);
            lookup.insert(Language::Portuguese, Algorithm::Portuguese);
            lookup.insert(Language::Romanian, Algorithm::Romanian);
            lookup.insert(Language::Russian, Algorithm::Russian);
            lookup.insert(Language::Spanish, Algorithm::Spanish);
            lookup.insert(Language::Swedish, Algorithm::Swedish);
            lookup.insert(Language::Tamil, Algorithm::Tamil);
            lookup.insert(Language::Turkish, Algorithm::Turkish);
        }

        let mut copy: HashMap<Language, Algorithm> = HashMap::with_capacity(STEM_ALGO_COUNT);
        for (key, value) in lookup.into_iter() {
            copy.insert(key.clone(), value.clone());
        }

        copy
    });

    match &hash.get(lang) {
        Some(algo) => Ok(algo),
        None => Err(StemmingError::NoStemmerForLang(format!(
            "The language passed into the stemmer [{}] does not have a stemmer algorithm.",
            lang
        ))),
    }
}

// /// Gets an Stemmer algorithm if available for the given language
// fn get_algorithm(lang: &Language) -> Result<&Algorithm, StemmingErrors> {
//     let algo = find_algo(lang).unwrap();

//     Ok(algo)
// }

/// Provides stemming on the text corpus to reduce
/// words with the same basic meaning into a concentrated
pub fn stem(corpus: &str, lang: &Language) {
    let algo = find_algo(lang).unwrap();
}
