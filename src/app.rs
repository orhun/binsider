use crate::{
    elf::Elf,
    error::{Error, Result},
    tracer::TraceData,
    tui::event::Event,
};
use elf::{endian::AnyEndian, ElfBytes};
use heh::app::Application as Heh;
use heh::decoder::Encoding;
use ratatui::text::Line;
use rust_strings::BytesConfig;
use std::{
    fmt::{self, Debug, Formatter},
    fs::{File, OpenOptions},
    sync::mpsc,
    thread,
};

/// Binary analyzer.
pub struct Analyzer<'a> {
    /// Path of the ELF file.
    pub path: &'a str,
    /// Bytes of the file.
    bytes: &'a [u8],
    /// Whether if the file is read only.
    pub is_read_only: bool,
    /// Elf properties.
    pub elf: Elf,
    /// Strings.
    pub strings: Option<Vec<(u64, String)>>,
    /// Min length of the strings.
    pub strings_len: usize,
    /// Heh application.
    pub heh: Heh,
    /// Tracer data.
    pub tracer: TraceData,
    /// System calls.
    pub system_calls: Vec<Line<'a>>,
}

impl Debug for Analyzer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Analyzer")
            .field("bytes", &self.bytes)
            .finish()
    }
}

impl<'a> Analyzer<'a> {
    /// Constructs a new instance.
    pub fn new(path: &'a str, bytes: &'a [u8], strings_len: usize) -> Result<Self> {
        let elf_bytes = ElfBytes::<AnyEndian>::minimal_parse(bytes)?;
        let elf = Elf::try_from(elf_bytes)?;
        let mut read_only = false;
        let file = match OpenOptions::new().write(true).read(true).open(path) {
            Ok(v) => v,
            Err(_) => {
                read_only = true;
                File::open(path)?
            }
        };
        let heh =
            Heh::new(file, Encoding::Ascii, 0).map_err(|e| Error::HexdumpError(e.to_string()))?;
        Ok(Self {
            path,
            bytes,
            is_read_only: read_only,
            elf,
            strings: None,
            strings_len,
            heh,
            tracer: TraceData::default(),
            system_calls: Vec::new(),
        })
    }

    /// Returns the sequences of printable characters.
    pub fn extract_strings(&mut self, event_sender: mpsc::Sender<Event>) {
        let config = BytesConfig::new(self.bytes.to_vec()).with_min_length(self.strings_len);
        thread::spawn(move || {
            event_sender
                .send(Event::FileStrings(
                    rust_strings::strings(&config).map_err(|e| Error::StringsError(e.to_string())),
                ))
                .expect("failed to send strings event");
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    fn get_test_bytes() -> Result<Vec<u8>> {
        let debug_binary = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join(env!("CARGO_PKG_NAME"));
        Ok(fs::read(debug_binary)?)
    }

    #[test]
    fn test_init() -> Result<()> {
        assert!(Analyzer::new("", get_test_bytes()?.as_slice(), 4).is_ok());
        Ok(())
    }

    #[test]
    fn test_extract_strings() -> Result<()> {
        let test_bytes = get_test_bytes()?;
        let mut analyzer = Analyzer::new("", test_bytes.as_slice(), 4)?;
        let (tx, rx) = mpsc::channel();
        analyzer.extract_strings(tx);
        if let Event::FileStrings(strings) = rx.recv()? {
            assert!(strings?.iter().map(|(s, _)| s).any(|v| v == ".debug_str"));
        } else {
            panic!("strings did not succeed");
        }
        Ok(())
    }
}
