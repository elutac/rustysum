// Import necessary libraries
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// This function takes a file path as input and reads the contents of the file.
/// It returns a vector of strings, where each string is a word from the file.
///
/// # Arguments
///
/// * `file_path` - A path to the file to be processed.
///
/// # Returns
///
/// * A `Result` object containing a `Vec` of `String`s, where each `String` is a word from the file
///   if the file could be successfully read, otherwise an `io::Error` object.
///
pub fn process_txt(file_path: &Path) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut words: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line_words: Vec<String> = line.split_whitespace().map(String::from).collect();
        words.extend(line_words);
    }

    Ok(words)
}
