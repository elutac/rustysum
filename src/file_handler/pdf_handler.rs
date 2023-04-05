// Import necessary libraries
use pdf_extract::extract_text;
use std::io::{self, Error, ErrorKind};
use std::path::Path;

/// This function takes a file path to a PDF file as input and reads the text content of the PDF file.
/// It returns the text content of the PDF file as a vector of strings, where each string is a word from the file.
///
/// # Arguments
///
/// * `file_path` - A path to the PDF file to be processed.
///
/// # Returns
///
/// * A `Result` object containing a `Vec` of `String`s, where each `String` is a word from the PDF file
///   if the file could be successfully read and processed, otherwise an `io::Error` object.
///
pub fn process_pdf(file_path: &Path) -> Result<Vec<String>, io::Error> {
    let text = extract_text(file_path).map_err(|e| {
        Error::new(
            ErrorKind::InvalidData,
            format!("Failed to extract text from PDF: {}", e),
        )
    })?;

    let text = text.replace("\r", "").replace("\n", " ").replace("\t", " ");
    let text = text.split_whitespace().collect::<Vec<&str>>().join(" ");

    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    Ok(words)
}