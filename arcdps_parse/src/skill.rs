use crate::{
    util::{read_string_buffer, Endian},
    Parse, ParseError, Save,
};
use byteorder::{ReadBytesExt, WriteBytesExt};
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
        Ok(Self {
            id: input.read_u32::<Endian>()?,
            name: read_string_buffer::<64>(input)?,
        })
    }
}

impl Save for Skill {
    type Error = io::Error;

    fn save(&self, output: &mut impl io::Write) -> Result<(), Self::Error> {
        output.write_u32::<Endian>(self.id)?;
        output.write_all(self.name.as_bytes())
    }
}
