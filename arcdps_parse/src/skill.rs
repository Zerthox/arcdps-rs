use crate::{
    util::{read_string_buffer, truncate_null, write_string_buffer, Endian},
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

impl Skill {
    /// Size of the skill name string.
    pub const NAME_SIZE: usize = 64;
}

impl Parse for Skill {
    type Error = ParseError;

    fn parse(input: &mut impl io::Read) -> Result<Self, Self::Error> {
        Ok(Self {
            id: input.read_u32::<Endian>()?,
            name: truncate_null(read_string_buffer::<{ Self::NAME_SIZE }>(input)?),
        })
    }
}

impl Save for Skill {
    type Error = io::Error;

    fn save(&self, output: &mut impl io::Write) -> Result<(), Self::Error> {
        output.write_u32::<Endian>(self.id)?;
        write_string_buffer::<{ Self::NAME_SIZE }>(output, &self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn skill_name() {
        const SIZE: usize = size_of::<u32>() + Skill::NAME_SIZE;

        let data: &[u8; SIZE] = b"\x07\0\0\0Skill Name\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

        let skill = Skill::parse(io::Cursor::new(data.as_slice()).get_mut())
            .expect("failed to parse skill");
        assert_eq!(7, skill.id, "incorrect skill id");
        assert_eq!("Skill Name", skill.name, "incorrect skill name");

        let mut buffer = [123u8; SIZE];
        skill
            .save(&mut buffer.as_mut_slice())
            .expect("failed to save skill");
        assert_eq!(data, &buffer, "incorrect saved data");
    }
}
