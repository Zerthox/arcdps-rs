use crate::{
    extract::{transmute_field, Extract},
    Event, StateChange, TryExtract,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Buff formula from an [`Event`] with [`StateChange::BuffFormula`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuffFormula {
    pub skill_id: u32,
    pub formula: u32,
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
    unsafe fn extract(event: &Event) -> Self {
        RawBuffFormula::extract(event).into()
    }
}

impl TryExtract for BuffFormula {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        RawBuffFormula::can_extract(event)
    }
}

impl From<RawBuffFormula> for BuffFormula {
    #[inline]
    fn from(raw: RawBuffFormula) -> Self {
        Self {
            skill_id: raw.skill_id,
            formula: raw.formula as _,
            attr1: raw.attr1 as _,
            attr2: raw.attr2 as _,
            param1: raw.param1,
            param2: raw.param2,
            param3: raw.param3,
            trait_src: raw.trait_src as _,
            trait_self: raw.trait_self as _,
            buff_src: raw.buff_src as _,
            buff_self: raw.buff_self as _,
            not_npc: raw.not_npc != 0,
            not_player: raw.not_player != 0,
            is_break: raw.is_break != 0,
            value: raw.value,
            value_type: raw.value_type,
        }
    }
}

/// Buff formula from an [`Event`] with [`StateChange::BuffFormula`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RawBuffFormula {
    pub skill_id: u32,
    pub formula: f32,
    pub attr1: f32,
    pub attr2: f32,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub trait_src: f32,
    pub trait_self: f32,
    pub buff_src: f32,
    pub buff_self: f32,
    pub not_npc: u8,
    pub not_player: u8,
    pub is_break: u8,
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
    unsafe fn extract(event: &Event) -> Self {
        let [kind, attr1, attr2, param1, param2, param3, trait_src, trait_self] =
            transmute_field!(event.time as [f32; 8]);
        let [buff_src, buff_self] = transmute_field!(event.src_instance_id as [f32; 2]);

        Self {
            skill_id: event.skill_id,
            formula: kind,
            attr1,
            attr2,
            param1,
            param2,
            param3,
            trait_src,
            trait_self,
            buff_src,
            buff_self,
            not_npc: event.is_flanking,
            not_player: event.is_shields,
            is_break: event.is_offcycle,
            value: event.overstack_value,
            value_type: event.pad61,
        }
    }
}

impl TryExtract for RawBuffFormula {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        event.get_statechange() == StateChange::BuffFormula
    }
}
