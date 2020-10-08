use std::io::Error as IOError;

/// Enumerates possible `file_system` module
/// errors
#[derive(Debug)]
pub enum Error {
    /// The path provided doesn't exists
    NoExists(String),
    /// A generic IO Error
    IOError(IOError),
}
