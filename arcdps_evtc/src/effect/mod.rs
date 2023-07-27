mod guid;
mod old;

pub use self::guid::*;
pub use self::old::*;

use crate::{CombatEvent, Position, StateChange};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Effect information from a [`CombatEvent`] with [`StateChange::Effect`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Effect {
    /// Id of the effect.
    ///
    /// Use to map to a GUID using [`StateChange::IdToGUID`] events.
    pub effect_id: u32,

    /// Owner of the effect.
    pub owner: u64,

    /// Location of the effect.
    pub location: EffectLocation,

    /// Duration of the effect in milliseconds.
    pub duration: u32,

    /// Trackable id for effect end.
    pub tracking_id: u32,

    /// Effect orientation.
    pub orientation: EffectOrientation,
}

impl Effect {
    /// Extracts effect information from an [`StateChange::Effect`] event.
    ///
    /// # Safety
    /// This operation is safe when the [`CombatEvent`] is a valid effect event.
    #[inline]
    pub unsafe fn from_event(event: &CombatEvent) -> Self {
        let effect_id = event.skill_id;
        let duration: u32 = unsafe {
            transmute([
                event.affinity.into(),
                event.buff,
                event.result,
                event.is_activation.into(),
            ])
        };
        let tracking_id: u32 = unsafe {
            transmute([
                event.is_buff_remove.into(),
                event.is_ninety,
                event.is_fifty,
                event.is_moving,
            ])
        };
        let orientation: [i16; 3] = unsafe {
            transmute([
                event.is_shields,
                event.is_off_cycle,
                event.pad61,
                event.pad62,
                event.pad63,
                event.pad64,
            ])
        };

        Self {
            effect_id,
            owner: event.src_agent,
            location: EffectLocation::from_event(event),
            duration,
            tracking_id,
            orientation: orientation.into(),
        }
    }

    /// Checks whether this is the end of an effect.
    #[inline]
    pub fn is_end(&self) -> bool {
        self.effect_id == 0
    }
}

impl TryFrom<&CombatEvent> for Effect {
    type Error = ();

    #[inline]
    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        match event.is_statechange {
            StateChange::Effect => Ok(unsafe { Self::from_event(event) }),
            _ => Err(()),
        }
    }
}

/// Location of an effect.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EffectLocation {
    Agent(u64),
    Position(Position),
}

impl EffectLocation {
    /// Extracts an effect location from an effect [`CombatEvent`].
    ///
    /// # Safety
    /// This operation is safe when the [`CombatEvent`] is a valid effect event.
    #[inline]
    pub unsafe fn from_event(event: &CombatEvent) -> Self {
        if event.dst_agent != 0 {
            Self::Agent(event.dst_agent)
        } else {
            let pos: [f32; 3] = transmute((event.value, event.buff_dmg, event.overstack_value));
            Self::Position(pos.into())
        }
    }
}

/// Orientation of an effect.
///
/// Values represent rotation along each axis multiplied by `1000` or [`i16::MIN`]/[`i16::MAX`] if out of range.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EffectOrientation {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl EffectOrientation {
    /// Ratio between [`i16`] and [`f32`] representation.
    pub const RATIO: f32 = 1000.0;

    /// Maximum value in [`f32`] representation.
    ///
    /// For [`i16`] representation use [`i16::MAX`].
    pub const MAX: f32 = i16::MAX as f32 / Self::RATIO;

    /// Minimum value in [`f32`] representation.
    ///
    /// For [`i16`] representation use [`i16::MIN`].
    pub const MIN: f32 = i16::MIN as f32 / Self::RATIO;

    /// Creates a new effect orientation from radians in [`i16`] representation.
    #[inline]
    pub const fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    /// Creates a new effect orientation from radians in [`f32`] representation.
    #[inline]
    pub fn from_floats(x: f32, y: f32, z: f32) -> Self {
        Self::new(Self::to_int(x), Self::to_int(y), Self::to_int(z))
    }

    /// Converts int to float.
    #[inline]
    pub fn to_float(int: i16) -> f32 {
        int as f32 / Self::RATIO
    }

    /// Converts int to float.
    #[inline]
    pub fn to_int(float: f32) -> i16 {
        (float * Self::RATIO).round() as i16
    }

    /// Converts the orientation to a [`Position`].
    #[inline]
    pub fn as_position(&self) -> Position {
        Position::new(
            Self::to_float(self.x),
            Self::to_float(self.y),
            Self::to_float(self.z),
        )
    }
}

impl From<[i16; 3]> for EffectOrientation {
    #[inline]
    fn from(value: [i16; 3]) -> Self {
        let [x, y, z] = value;
        Self::new(x, y, z)
    }
}

impl From<EffectOrientation> for [i16; 3] {
    #[inline]
    fn from(orientation: EffectOrientation) -> Self {
        [orientation.x, orientation.y, orientation.z]
    }
}

impl From<EffectOrientation> for Position {
    #[inline]
    fn from(orientation: EffectOrientation) -> Self {
        orientation.as_position()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation() {
        let orient = EffectOrientation::from_floats(12.345, 6.789, 0.0);
        assert_eq!(orient, EffectOrientation::new(12345, 6789, 0));
    }

    #[test]
    fn orientation_round() {
        assert_eq!(EffectOrientation::to_int(30.9999), 31000);
    }

    #[test]
    fn orientation_saturate() {
        let orient = EffectOrientation::from_floats(12345.0, -6789.0, 0.0);
        assert_eq!(orient, EffectOrientation::new(i16::MAX, i16::MIN, 0));
    }
}
