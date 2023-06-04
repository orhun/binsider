use thiserror::Error as ThisError;

/// Custom error type.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that may occur during I/O operations.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    /// Error that may occur while receiving messages from the channel.
    #[error("channel receive error: `{0}`")]
    ChannelReceiveError(#[from] std::sync::mpsc::RecvError),
    /// Error that may occur while working with ELF files.
    #[error("ELF error: `{0}`")]
    ElfError(#[from] goblin::error::Error),
    /// Error that may occur while extracting strings from binary data.
    #[error("String extraction error: `{0}`")]
    StringsError(String),
}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_error() {
        let message = "your computer is on fire!";
        let error = Error::from(IoError::new(ErrorKind::Other, message));
        assert_eq!(format!("IO error: `{message}`"), error.to_string());
        assert_eq!(
            format!("\"IO error: `{message}`\""),
            format!("{:?}", error.to_string())
        );
    }
}
