use crate::error::Result;
use std::fs::{File, OpenOptions};

/// General file information.
#[derive(Debug)]
pub struct FileInfo<'a> {
    /// Path of the file.
    pub path: &'a str,
    /// Bytes of the file.
    pub bytes: &'a [u8],
    /// Whether if the file is read only.
    pub is_read_only: bool,
}

impl<'a> FileInfo<'a> {
    /// Constructs a new instance.
    pub fn new(path: &'a str, bytes: &'a [u8]) -> Result<Self> {
        Ok(Self {
            path,
            bytes,
            is_read_only: false,
        })
    }

    /// Opens the file (with R/W if possible) and returns it.
    pub fn open_file(&mut self) -> Result<File> {
        Ok(
            match OpenOptions::new().write(true).read(true).open(self.path) {
                Ok(v) => v,
                Err(_) => {
                    self.is_read_only = true;
                    File::open(self.path)?
                }
            },
        )
    }
}
