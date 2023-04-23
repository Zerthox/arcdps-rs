use crate::{CombatEvent, StateChange};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Positional information.
///
/// This can be from a [`CombatEvent`] with [`StateChange::Position`], [`StateChange::Velocity`] or [`StateChange::Facing`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl TryFrom<&CombatEvent> for Position {
    type Error = ();

    fn try_from(event: &CombatEvent) -> Result<Self, Self::Error> {
        #[allow(clippy::transmute_int_to_float)]
        match event.is_statechange {
            StateChange::Position | StateChange::Velocity | StateChange::Facing => {
                let [x, y]: [f32; 2] = unsafe { transmute(event.dst_agent) };
                let z = unsafe { transmute(event.value) };
                Ok(Self { x, y, z })
            }

            _ => Err(()),
        }
    }
}
