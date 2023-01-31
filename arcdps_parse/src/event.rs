use crate::{util::Endian, CombatEvent, Parse};
use byteorder::ReadBytesExt;
use std::io;

impl Parse for CombatEvent {
    type Error = io::Error;

    fn parse(input: &mut impl io::Read) -> Result<Self, Self::Error> {
        Ok(Self {
            time: input.read_u64::<Endian>()?,
            src_agent: input.read_u64::<Endian>()? as usize,
            dst_agent: input.read_u64::<Endian>()? as usize,
            value: input.read_i32::<Endian>()?,
            buff_dmg: input.read_i32::<Endian>()?,
            overstack_value: input.read_u32::<Endian>()?,
            skill_id: input.read_u32::<Endian>()?,
            src_instance_id: input.read_u16::<Endian>()?,
            dst_instance_id: input.read_u16::<Endian>()?,
            src_master_instance_id: input.read_u16::<Endian>()?,
            dst_master_instance_id: input.read_u16::<Endian>()?,
            affinity: input.read_u8()?.into(),
            buff: input.read_u8()?,
            result: input.read_u8()?,
            is_activation: input.read_u8()?.into(),
            is_buff_remove: input.read_u8()?.into(),
            is_ninety: input.read_u8()?,
            is_fifty: input.read_u8()?,
            is_moving: input.read_u8()?,
            is_statechange: input.read_u8()?.into(),
            is_flanking: input.read_u8()?,
            is_shields: input.read_u8()?,
            is_off_cycle: input.read_u8()?,
            pad61: input.read_u8()?,
            pad62: input.read_u8()?,
            pad63: input.read_u8()?,
            pad64: input.read_u8()?,
        })
    }
}
