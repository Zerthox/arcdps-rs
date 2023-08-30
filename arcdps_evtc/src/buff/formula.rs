use crate::{extract::Extract, CombatEvent, StateChange, TryExtract};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Buff formula from a [`CombatEvent`] with [`StateChange::BuffFormula`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffFormula {
    pub kind: u32,
    pub attr1: u32,
    pub attr2: u32,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub trait_src: u32,
    pub trait_self: u32,
    pub buff_src: u32,
    pub buff_self: u32,
    pub not_npc: bool,
    pub not_player: bool,
    pub is_break: bool,
    pub value: u32,
    pub value_type: u8,
}

impl BuffFormula {
    /// Whether the buff formula is always active.
    #[inline]
    pub fn is_unconditional(&self) -> bool {
        self.trait_src == 0 && self.trait_self == 0 && self.buff_src == 0 && self.buff_self == 0
    }
}

impl Extract for BuffFormula {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        RawBuffFormula::extract(event).into()
    }
}

impl TryExtract for BuffFormula {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        RawBuffFormula::can_extract(event)
    }
}

impl From<RawBuffFormula> for BuffFormula {
    #[inline]
    fn from(raw: RawBuffFormula) -> Self {
        Self {
            kind: raw.kind as _,
            attr1: raw.attr1 as _,
            attr2: raw.attr2 as _,
            param1: raw.param1,
            param2: raw.param2,
            param3: raw.param3,
            trait_src: raw.trait_src as _,
            trait_self: raw.trait_self as _,
            buff_src: raw.buff_src as _,
            buff_self: raw.buff_self as _,
            not_npc: raw.not_npc,
            not_player: raw.not_player,
            is_break: raw.is_break,
            value: raw.value,
            value_type: raw.value_type,
        }
    }
}

/// Buff formula from a [`CombatEvent`] with [`StateChange::BuffFormula`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RawBuffFormula {
    pub kind: f32,
    pub attr1: f32,
    pub attr2: f32,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub trait_src: f32,
    pub trait_self: f32,
    pub buff_src: f32,
    pub buff_self: f32,
    pub not_npc: bool,
    pub not_player: bool,
    pub is_break: bool,
    pub value: u32,
    pub value_type: u8,
}

impl RawBuffFormula {
    /// Whether the buff formula is always active.
    #[inline]
    pub fn is_unconditional(&self) -> bool {
        self.trait_src == 0.0
            && self.trait_self == 0.0
            && self.buff_src == 0.0
            && self.buff_self == 0.0
    }
}

impl Extract for RawBuffFormula {
    #[inline]
    unsafe fn extract(event: &CombatEvent) -> Self {
        let [kind, attr1, attr2, param1, param2, param3, trait_src, trait_self]: [f32; 8] =
            transmute((
                event.time,
                event.src_agent,
                event.dst_agent,
                event.value,
                event.buff_dmg,
            ));
        let [buff_src, buff_self]: [f32; 2] = transmute((
            event.src_instance_id,
            event.dst_instance_id,
            event.src_master_instance_id,
            event.dst_master_instance_id,
        ));

        Self {
            kind,
            attr1,
            attr2,
            param1,
            param2,
            param3,
            trait_src,
            trait_self,
            buff_src,
            buff_self,
            not_npc: event.is_flanking != 0,
            not_player: event.is_shields != 0,
            is_break: event.is_offcycle != 0,
            value: event.overstack_value,
            value_type: event.pad61,
        }
    }
}

impl TryExtract for RawBuffFormula {
    #[inline]
    fn can_extract(event: &CombatEvent) -> bool {
        event.is_statechange == StateChange::BuffFormula
    }
}
