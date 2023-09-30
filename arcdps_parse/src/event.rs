use crate::{util::Endian, Event, Parse, Save};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io;

impl Parse for Event {
    type Error = io::Error;

    fn parse(input: &mut impl io::Read) -> Result<Self, Self::Error> {
        Ok(Self {
            time: input.read_u64::<Endian>()?,
            src_agent: input.read_u64::<Endian>()?,
            dst_agent: input.read_u64::<Endian>()?,
            value: input.read_i32::<Endian>()?,
            buff_dmg: input.read_i32::<Endian>()?,
            overstack_value: input.read_u32::<Endian>()?,
            skill_id: input.read_u32::<Endian>()?,
            src_instance_id: input.read_u16::<Endian>()?,
            dst_instance_id: input.read_u16::<Endian>()?,
            src_master_instance_id: input.read_u16::<Endian>()?,
            dst_master_instance_id: input.read_u16::<Endian>()?,
            affinity: input.read_u8()?,
            buff: input.read_u8()?,
            result: input.read_u8()?,
            is_activation: input.read_u8()?,
            is_buffremove: input.read_u8()?,
            is_ninety: input.read_u8()?,
            is_fifty: input.read_u8()?,
            is_moving: input.read_u8()?,
            is_statechange: input.read_u8()?,
            is_flanking: input.read_u8()?,
            is_shields: input.read_u8()?,
            is_offcycle: input.read_u8()?,
            pad61: input.read_u8()?,
            pad62: input.read_u8()?,
            pad63: input.read_u8()?,
            pad64: input.read_u8()?,
        })
    }
}

impl Save for Event {
    type Error = io::Error;

    fn save(&self, output: &mut impl io::Write) -> Result<(), Self::Error> {
        output.write_u64::<Endian>(self.time)?;
        output.write_u64::<Endian>(self.src_agent)?;
        output.write_u64::<Endian>(self.dst_agent)?;
        output.write_i32::<Endian>(self.value)?;
        output.write_i32::<Endian>(self.buff_dmg)?;
        output.write_u32::<Endian>(self.overstack_value)?;
        output.write_u32::<Endian>(self.skill_id)?;
        output.write_u16::<Endian>(self.src_instance_id)?;
        output.write_u16::<Endian>(self.dst_instance_id)?;
        output.write_u16::<Endian>(self.src_master_instance_id)?;
        output.write_u16::<Endian>(self.dst_master_instance_id)?;
        output.write_u8(self.affinity)?;
        output.write_u8(self.buff)?;
        output.write_u8(self.result)?;
        output.write_u8(self.is_activation)?;
        output.write_u8(self.is_buffremove)?;
        output.write_u8(self.is_ninety)?;
        output.write_u8(self.is_fifty)?;
        output.write_u8(self.is_moving)?;
        output.write_u8(self.is_statechange)?;
        output.write_u8(self.is_flanking)?;
        output.write_u8(self.is_shields)?;
        output.write_u8(self.is_offcycle)?;
        output.write_u8(self.pad61)?;
        output.write_u8(self.pad62)?;
        output.write_u8(self.pad63)?;
        output.write_u8(self.pad64)
    }
}
