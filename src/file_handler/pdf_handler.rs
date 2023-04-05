use pdf_extract::extract_text;
use std::io::{self, Error, ErrorKind};
use std::path::Path;

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