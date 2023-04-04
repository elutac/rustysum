// Import the handlers module.
mod file_handler;

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
            "txt" => file_handler::process_txt(file_path),
            "pdf" => file_handler::process_pdf(file_path),
            _ => println!("Unsupported file type: {}", file_type),
        },
        None => println!("Cannot determine file type for: {:?}", file_path),
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
