//! Parsing for ArcDPS EVTC logs.
//!
//! # Usage
//! A [`Log`] can be parsed from any input implementing [`Read`](io::Read).
//! ```no_run
//! use arcdps_parse::{Log, Parse};
//! use std::fs::File;
//!
//! # fn main() -> Result<(), arcdps_parse::ParseError> {
//! let mut file = File::open("log.evtc")?;
//! let log = Log::parse(&mut file)?;
//! assert_eq!(log.header.revision, 1);
//! # Ok(())
//! # }
//! ```
//!
//! Note that ArcDPS may save zipped log files with the `.zevtc` extension.
//! Reading those can be realized using for example the [zip](https://docs.rs/zip/) crate.

mod agent;
mod error;
mod event;
mod log;
mod skill;
mod util;

pub use self::agent::*;
pub use self::error::*;
pub use self::log::*;
pub use self::skill::*;
pub use arcdps_evtc::*;

use std::io;

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

/// Interface for writing a value into a [`Write`](io::Write) output.
pub trait Save: Sized {
    /// Associated error which can happen during saving.
    type Error;

    /// Saves the value to the output.
    fn save(&self, output: &mut impl io::Write) -> Result<(), Self::Error>;
}
