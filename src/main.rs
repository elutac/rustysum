// Import the handlers module.
mod file_handler;

// Import the word counter module.
mod word_counter;

// Additional imports.
use std::env;
use std::path::Path;
use std::fs;

fn main() {
    // Parse command-line arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let file_path = Path::new(&args[1]);

    // Check if the file exists.
    if let Err(_) = fs::metadata(&file_path) {
        println!("File not found: {:?}", file_path);
        return;
    }

    // Determine the file type and call the corresponding function.
    match get_file_type(&file_path) {
        Some(file_type) => match file_type.as_str() {
            "txt" => process_and_count_words(&file_path, file_handler::process_txt),
            "pdf" => process_and_count_words(&file_path, file_handler::process_pdf),
            // Add more file types here.
            _ => println!("Unsupported file type."),
        },
        None => println!("Could not determine the file type."),
    }
}

/// Determines the file type based on the file extension.
///
/// # Arguments
///
/// * `file_path` - A reference to a Path representing the file path.
///
/// # Returns
///
/// * `Option<String>` - The file extension in lowercase if found, otherwise `None`.
fn get_file_type(file_path: &Path) -> Option<String> {
    file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}

fn process_and_count_words(
    file_path: &Path,
    process_function: fn(&Path) -> Result<Vec<String>, std::io::Error>,
) {
    match process_function(file_path) {
        Ok(words) => print_word_count(&words),
        Err(e) => println!("Error processing the file: {}", e),
    }
}

fn print_word_count(words: &[String]) {
    let word_count = word_counter::count_words(words);
    println!("Total number of words: {}", word_count);
}
