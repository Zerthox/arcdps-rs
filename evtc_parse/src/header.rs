use crate::{
    util::{read_string_buffer, write_string_buffer, Endian},
    Parse, ParseError, Save,
};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An EVTC log header.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Header {
    /// Date this log was recorded.
    pub date: String,

    /// EVTC API revision used.
    pub revision: u8,

    /// Boss id of the log target.
    ///
    /// An id of `1` indicates a WvW log.
    /// An id of `2` indicates a map log.
    pub boss_id: u16,
}

impl Header {
    /// Size of the date string.
    pub const DATE_SIZE: usize = 12;
}

impl Parse for Header {
    type Error = ParseError;

    fn parse(input: &mut impl io::Read) -> Result<Self, Self::Error> {
        let evtc = read_string_buffer::<4>(input)?;
        if evtc != "EVTC" {
            return Err(ParseError::NotEvtc);
        }

        let date = read_string_buffer::<{ Self::DATE_SIZE - 4 }>(input)?;
        let revision = input.read_u8()?;
        let boss_id = input.read_u16::<Endian>()?;

        // unused byte in arc header
        input.read_u8()?;

        Ok(Self {
            date: evtc + &date,
            revision,
            boss_id,
        })
    }
}

impl Save for Header {
    type Error = io::Error;

    fn save(&self, output: &mut impl io::Write) -> Result<(), Self::Error> {
        write_string_buffer::<{ Self::DATE_SIZE }>(output, &self.date)?;
        output.write_u8(self.revision)?;
        output.write_u16::<Endian>(self.boss_id)?;

        // unused byte
        output.write_u8(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header() {
        let header = Header {
            date: "EVTC20230328".into(),
            revision: 1,
            boss_id: 123,
        };

        let mut vec = Vec::with_capacity(64);
        header.save(&mut vec).unwrap();

        let parsed = Header::parse(&mut vec.as_slice()).unwrap();
        assert_eq!(parsed, header);
    }
}
