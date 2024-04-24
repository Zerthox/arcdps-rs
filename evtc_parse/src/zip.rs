use crate::{Log, Parse, ParseError};
use std::io;
use zip::{result::ZipError, ZipArchive};

/// Parses a [`Log`] from a compressed `zevtc` input.
pub fn parse_zevtc(input: impl io::Read + io::Seek) -> Result<Log, ParseError> {
    Log::parse_zevtc(input)
}

impl Log {
    /// Parses a [`Log`] from a compressed `zevtc` input.
    pub fn parse_zevtc(input: impl io::Read + io::Seek) -> Result<Log, ParseError> {
        let mut archive = ZipArchive::new(input).expect("input log file not compressed");
        let mut file = archive.by_index(0).expect("input log file empty");
        Log::parse(&mut file)
    }
}

impl From<ZipError> for ParseError {
    fn from(err: ZipError) -> Self {
        match err {
            ZipError::Io(io) => Self::IoError(io),
            _ => Self::NotEvtc,
        }
    }
}
