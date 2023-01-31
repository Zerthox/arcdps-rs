mod agent;
mod error;
mod event;
mod log;
mod skill;
mod util;

pub use agent::*;
pub use arcdps_evtc::*;
pub use error::*;
pub use log::*;
pub use skill::*;

use std::io;

/// Interface for parsing a value from a [`Read`] input.
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
