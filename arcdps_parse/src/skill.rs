use crate::{
    util::{read_string_buffer, Endian},
    Parse, ParseError,
};
use byteorder::ReadBytesExt;
use std::io;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An EVTC skill definition.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Skill {
    /// Id of the skill.
    pub id: u32,

    /// Name of the skill.
    pub name: String,
}

impl Parse for Skill {
    type Error = ParseError;

    fn parse(input: &mut impl io::Read) -> Result<Self, Self::Error> {
        let id = input.read_u32::<Endian>()?;
        let name = read_string_buffer::<64>(input)?;

        Ok(Self { id, name })
    }
}
