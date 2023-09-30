//! Parsing for ArcDPS EVTC logs.
//!
//! # Usage
//! Use the [`parse_file`] function to easily parse a [`Log`] from a file path.
//! ```no_run
//! match evtc_parse::parse_file("path/to/log.evtc") {
//!     Ok(log) => println!("Log for boss id {}", log.header.boss_id),
//!     Err(err) => eprintln!("Encountered error {}", err),
//! }
//! ```
//!
//! A [`Log`] can also be parsed from any input implementing [`Read`](io::Read).
//! ```no_run
//! use evtc_parse::{Log, Parse};
//! use std::io;
//!
//! fn parse_from_read(input: &mut impl io::Read) -> Log {
//!     Log::parse(input).expect("failed to parse")
//! }
//! ```
//!
//! Note that ArcDPS can save compressed log files with `.zevtc` as file extension.
//! Enabling the `"zevtc"` or `"zip"` feature adds support for compressed logs.

/// Extensions for log EVTC API.
#[path = "."]
mod ext {
    pub mod agent;
    pub mod event;
    pub mod skill;
}
mod error;
mod log;
mod util;

pub use self::error::*;
pub use self::log::*;
pub use evtc::*;
pub use ext::agent::*;
pub use ext::skill::*;

#[cfg(feature = "zevtc")]
mod zip;

#[cfg(feature = "zevtc")]
pub use self::zip::*;

use std::{fs::File, io, path::Path};

/// Parses a [`Log`] from a given [`Path`] to a log file.
///
/// With the `"zevtc"` or `"zip"` feature enabled this also supports compressed log files.
pub fn parse_file(path: impl AsRef<Path>) -> Result<Log, ParseError> {
    let path = path.as_ref();
    let mut file = io::BufReader::new(File::open(path)?);

    #[cfg(feature = "zevtc")]
    if let Some("zevtc" | "zip") = path.extension().and_then(|ext| ext.to_str()) {
        return parse_zevtc(file);
    }

    Log::parse(&mut file)
}

/// Interface for parsing a value from a [`Read`](io::Read) input.
pub trait Parse: Sized {
    /// Associated error which can happen during parsing.
    type Error;

    /// Parses a value of this type from the input.
    fn parse(input: &mut impl io::Read) -> Result<Self, Self::Error>;

    /// Parses multiple values of this type from the input into a [`Vec`].
    fn parse_multi(input: &mut impl io::Read, count: usize) -> Result<Vec<Self>, Self::Error> {
        (0..count).map(|_| Self::parse(input)).collect()
    }
}

/// Interface for saving a value into a [`Write`](io::Write) output.
pub trait Save: Sized {
    /// Associated error which can happen during saving.
    type Error;

    /// Saves the value to the output.
    fn save(&self, output: &mut impl io::Write) -> Result<(), Self::Error>;
}
