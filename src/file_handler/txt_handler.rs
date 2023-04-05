use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
