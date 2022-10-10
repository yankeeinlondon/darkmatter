extern crate rust_stemmers;

use std::collections::{HashMap, HashSet};

use lingua::Language;
use rust_stemmers::{Algorithm, Stemmer};

use super::StemmingError;

const STEM_ALGO_COUNT: usize = 17;

thread_local! {
    static STEM_LOOKUP: [(&'static str, Algorithm); STEM_ALGO_COUNT] = [
        ("arabic", Algorithm::Arabic),
        ("danish", Algorithm::Danish),
        ("dutch", Algorithm::Dutch),
        ("english", Algorithm::English),
        ("french", Algorithm::French),
        ("german", Algorithm::German),
        ("greek", Algorithm::Greek),
        ("hungarian", Algorithm::Hungarian),
        ("italian", Algorithm::Italian),
        ("nynorsk", Algorithm::Norwegian),
        ("portuguese", Algorithm::Portuguese),
        ("romanian", Algorithm::Romanian),
        ("russian", Algorithm::Russian),
        ("spanish", Algorithm::Spanish),
        ("swedish", Algorithm::Swedish),
        ("tamil", Algorithm::Tamil),
        ("turkish", Algorithm::Turkish),
    ];
}

pub struct StemAlgo(String, Algorithm);

impl StemAlgo {
    pub fn new(name: &str, algo: Algorithm) -> StemAlgo {
        StemAlgo(name.to_string(), algo)
    }

    pub fn name(&self) -> String {
        self.0.to_string()
    }

    pub fn algorithm(&self) -> &Algorithm {
        &self.1
    }
}

impl TryFrom<&Language> for StemAlgo {
    type Error = StemmingError;

    fn try_from(value: &Language) -> Result<StemAlgo, StemmingError> {
        STEM_LOOKUP.with(|lookup| {
            let mut found: Option<StemAlgo> = None;
            for l in lookup.iter() {
                let lc = format!("{}", value).to_lowercase();
                if lc.eq(l.0) {
                    let algo = l.1;
                    found = Some(StemAlgo::new(&lc, algo));
                    break;
                }
            }

            match found {
                Some(found) => Ok(found),
                None => Err(StemmingError::InvalidIso(value.to_string())),
            }
        })
    }
}

pub struct StemmedResults {
    /// the reduced/stemmed
    tokens: HashSet<String>,
    /// given a stemmed token, allows lookup of the antecedents
    reverse_lookup: HashMap<String, Vec<String>>,
}

impl Default for StemmedResults {
    fn default() -> Self {
        StemmedResults::new()
    }
}

impl StemmedResults {
    pub fn new() -> Self {
        StemmedResults {
            tokens: HashSet::new(),
            reverse_lookup: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        StemmedResults {
            tokens: HashSet::with_capacity(capacity),
            reverse_lookup: HashMap::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, token: &str) -> bool {
        self.tokens.contains(token)
    }

    pub fn insert(&mut self, original: &str, maps_to: String) {
        self.tokens.insert(maps_to.clone());
        if self.reverse_lookup.get(&maps_to).is_none() {
            self.reverse_lookup.insert(maps_to.clone(), vec![]);
        }

        self.reverse_lookup
            .get_mut(&maps_to)
            .unwrap()
            .push(original.to_string());
    }

    /// returns all the tokens compiled with the stemmer
    /// and clears the internal cache.
    pub fn take_tokens(&mut self) -> Vec<String> {
        self.tokens.drain().collect()
    }

    /// provides a means to determine what the antecedents were
    /// for a given token.
    pub fn reverse_lookup(&self, word: &str) -> Option<&Vec<String>> {
        self.reverse_lookup.get(word)
    }
}

/// Provides stemming on the text corpus to reduce
/// words with shared meaning to a single token
pub fn stem(
    tokens: &Vec<String>,
    lang: &Language,
) -> Result<StemmedResults, StemmingError> {
    let algo = StemAlgo::try_from(lang)?;
    let stemmer = Stemmer::create(algo.1);
    let mut results = StemmedResults::new();

    for token in tokens {
        let stemmed = stemmer.stem(token).to_string();
        results.insert(token, stemmed);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::words::Words;

    use super::*;

    #[test]
    fn simple_stem_test() {
        let mut outcome =
            stem(&vec!["fruitlessly".to_string()], &Language::English).unwrap();
        assert_eq!(outcome.tokens.len(), 1);
        let tokens = outcome.take_tokens();
        let token = tokens.get(0);
        assert_eq!(token.is_some(), true);
        if let Some(token) = token {
            assert_eq!(token, "fruitless");
        }
    }

    #[test]
    fn stem_a_full_corpus() {
        let path = Path::new("src/stemming/en.txt");
        let words = Words::from_path(path, Some(12))
            .expect("Couldn't read corpus.txt!");
        let results = stem(&words.as_tokens(), &Language::English)
            .expect("failed to stem the text loaded from file");

        assert_eq!(words.len() > results.len(), true);
        assert_eq!(results.contains("draft"), true);
        assert_eq!(results.contains("drafting"), false);
        assert_eq!(results.contains("drafted"), false);
        let draft = results.reverse_lookup.get("draft").unwrap();
        assert_eq!(draft.len(), 2);
    }
}
