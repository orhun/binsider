use thiserror::Error as ThisError;

/// Custom error type.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that may occur during I/O operations.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    /// Error that may occur during locating binaries.
    #[error("Unable to locate binary: `{0}`")]
    WhichError(#[from] which::Error),
    /// Error that may occur while receiving messages from the channel.
    #[error("Channel receive error: `{0}`")]
    ChannelReceiveError(#[from] std::sync::mpsc::RecvError),
    /// Error that may occur while sending messages to the channel.
    #[error("Channel send error: `{0}`")]
    ChannelSendError(String),
    /// Error that may occur while parsing ELF files.
    #[error("ELF parse error: `{0}`")]
    ElfError(#[from] elf::parse::ParseError),
    /// Error that may occur while extracting strings from binary data.
    #[error("String extraction error: `{0}`")]
    StringsError(String),
    /// Error that may occur while running hexdump.
    #[error("Hexdump (heh) error: `{0}`")]
    HexdumpError(String),
    /// Error that may occur while tracing system calls.
    #[error("Tracing system call error: `{0}`")]
    TraceError(String),
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
