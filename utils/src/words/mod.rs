use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

pub fn words(corpus: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::with_capacity(256);
    for line in corpus.lines() {
        for word in line.split_whitespace() {
            words.push(word.to_string());
        }
    }

    words
}

pub fn words_from_file(file: &Path) -> Result<Vec<String>, Error> {
    let mut words: Vec<String> = Vec::with_capacity(512);
    let reader = BufReader::new(
        File::open(file) //
            .expect("Can not open the file provided to words_from_file()."),
    );

    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            words.push(word.to_string());
        }
    }

    Ok(words)
}
