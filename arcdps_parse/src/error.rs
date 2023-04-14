use std::{io, string};
use thiserror::Error;

/// A possible error occurring during parsing.
#[derive(Debug, Error)]
pub enum ParseError {
    // Read related error.
    #[error(transparent)]
    IoError(#[from] io::Error),

    /// String conversion error.
    #[error(transparent)]
    FromUtf8Error(#[from] string::FromUtf8Error),

    /// Unsupported EVTC revision.
    #[error("unsupported evtc revision {0}")]
    UnsupportedRevision(u8),

    /// Data is not in EVTC format.
    #[error("not in evtc format")]
    NotEvtc,
}
