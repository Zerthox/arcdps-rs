use crate::{CombatEvent, StateChange};
use std::mem::transmute;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Positional information.
///
/// This can be from a [`CombatEvent`] with [`StateChange::Position`], [`StateChange::Velocity`] or [`StateChange::Facing`].
///
/// Ingame coordinates are interpreted as 1 unit = 1 inch.
/// The z-Axis represents vertical height and **points down**,
/// meaning lower values are a higher location ingame.
///
/// Mumble coordinates are given in meters.
/// The y-Axis represents vertical height and **points up**.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Conversion from inch to meter.
const CONVERT: f32 = 0.0254;

impl Position {
    /// Creates new positional information.
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Creates a position from Mumble coordinates.
    #[inline]
    pub fn from_mumble(coords: [f32; 3]) -> Self {
        let [x, y, z] = coords;
        Self::new(x / CONVERT, z / CONVERT, -y / CONVERT)
    }

    /// Converts the position to Mumble coordinates.
    #[inline]
    pub fn to_mumble(&self) -> [f32; 3] {
        [self.x * CONVERT, -self.z * CONVERT, self.y * CONVERT]
    }
}

impl From<[f32; 3]> for Position {
    fn from(value: [f32; 3]) -> Self {
        let [x, y, z] = value;
        Self { x, y, z }
    }
}

impl From<Position> for [f32; 3] {
    fn from(pos: Position) -> Self {
        [pos.x, pos.y, pos.z]
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn mumble_conversion() {
        let pos = Position::new(3993.409, 6225.539, -549.570);

        let mumble = pos.to_mumble();
        assert_relative_eq!(mumble[0], 101.433, max_relative = 0.01);
        assert_relative_eq!(mumble[1], 13.959, max_relative = 0.01);
        assert_relative_eq!(mumble[2], 158.129, max_relative = 0.01);

        let back = Position::from_mumble(mumble);
        assert_eq!(back, pos);
    }
}
