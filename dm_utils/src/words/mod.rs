pub mod errors;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
    path::Path,
};

pub use self::errors::WordsError;
/// A utility intended to convert a corpus of text into an
/// vector of words. Use `parse()` to build the struct or
/// alternatively if you want to load from a file use
/// `from_path()`
#[derive(Debug)]
pub struct Words {
    corpus: String,
    tokens: Vec<String>,
}

impl Words {
    /// Parse a corpus of text passed in as a string slice; optionally specifying
    /// the starting _capacity_ for the `Vec<String>` (aka, your best guess on
    /// the number of words you'll need).
    pub fn parse(corpus: &str, capacity: Option<usize>) -> Self {
        let capacity = match capacity {
            Some(capacity) => capacity,
            None => corpus.len() / 6,
        };
        let mut tokens: Vec<String> = Vec::with_capacity(capacity);
        for line in corpus.lines() {
            for word in line.split_whitespace() {
                tokens.push(word.to_string());
            }
        }

        Words {
            corpus: corpus.to_string(),
            tokens,
        }
    }

    /// Parse the content within a file in the filesystem; optionally specifying
    /// the starting _capacity_ for the `Vec<String>` (aka, your best guess on
    /// the number of words you'll need).
    pub fn from_path(
        path: &Path,
        capacity: Option<usize>,
    ) -> Result<Self, WordsError> {
        let capacity = capacity.unwrap_or(256);
        let file = File::open(path);
        match file {
            Ok(file) => {
                let mut tokens: Vec<String> = Vec::with_capacity(capacity);

                let reader = BufReader::new(file);
                let mut corpus = String::from("");

                for line in reader.lines() {
                    for word in line.unwrap().split_whitespace() {
                        tokens.push(word.to_string());
                        corpus = corpus.add(word)
                    }
                }

                Ok(Words { tokens, corpus })
            }
            Err(err) => Err(WordsError::UnableToLoadFile(err)),
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.len() == 0
    }

    pub fn get(&self, idx: usize) -> Option<&String> {
        self.tokens.get(idx)
    }

    // pub fn find(&self, find: &str) -> Option<String> {
    //     self.0
    // }

    pub fn as_tokens(&self) -> Vec<String> {
        self.tokens.clone()
    }

    /// The full text corpus passed into Words
    pub fn corpus(&self) -> String {
        self.corpus.clone()
    }
}
