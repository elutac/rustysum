// Import the handler files.
mod txt_handler;
mod pdf_handler;

// Re-export public functions from handler files.
pub use txt_handler::process_txt;
pub use pdf_handler::process_pdf;
