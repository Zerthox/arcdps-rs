//! Bindings & utilities for the game's 3d space.

use crate::{extract::Extract, AgentId, Event, StateChange, TryExtract};
use std::{
    mem::transmute,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Positional information for an agent.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PositionEvent {
    pub time: u64,
    pub agent: AgentId,
    pub position: Position,
}

impl Extract for PositionEvent {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        Self {
            time: event.time,
            agent: AgentId::from_src(event),
            position: Position::extract(event),
        }
    }
}

impl TryExtract for PositionEvent {
    #[inline]
    fn can_extract(event: &Event) -> bool {
        matches!(
            event.get_statechange(),
            StateChange::Position | StateChange::Velocity | StateChange::Facing
        )
    }
}

/// Positional information.
///
/// This can be from an [`Event`] with [`StateChange::Position`], [`StateChange::Velocity`] or [`StateChange::Facing`].
/// It can also occur in [`StateChange::Effect`] and [`StateChange::EffectOld`] events as effect location or orientation.
///
/// Ingame coordinates are interpreted as 1 unit = 1 inch.
/// The z-axis represents vertical height and **points down**,
/// meaning lower values are a higher location ingame.
///
/// Mumble coordinates are given in meters.
/// The y-axis represents vertical height and **points up**.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    /// Conversion from inch to meter.
    pub const INCH_TO_METER: f32 = 0.0254;

    /// Creates new positional information.
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Creates a position from Mumble coordinates.
    #[inline]
    pub fn from_mumble(coords: [f32; 3]) -> Self {
        let [x, y, z] = coords;
        Self::new(
            x / Self::INCH_TO_METER,
            z / Self::INCH_TO_METER,
            -y / Self::INCH_TO_METER,
        )
    }

    /// Converts the position to an [`array`].
    #[inline]
    pub fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    /// Converts the position to a [`tuple`].
    #[inline]
    pub fn to_tuple(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    /// Converts the position to Mumble coordinates.
    #[inline]
    pub fn to_mumble(&self) -> [f32; 3] {
        [
            self.x * Self::INCH_TO_METER,
            -self.z * Self::INCH_TO_METER,
            self.y * Self::INCH_TO_METER,
        ]
    }

    /// Returns the length of the position interpreted as vector.
    #[inline]
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Interprets the position as vector and multiplies it with the given matrix.
    #[inline]
    pub fn mat_mul(&self, matrix: [[f32; 3]; 3]) -> Self {
        let x = matrix[0][0] * self.x + matrix[0][1] * self.y + matrix[0][2] * self.z;
        let y = matrix[1][0] * self.x + matrix[1][1] * self.y + matrix[1][2] * self.z;
        let z = matrix[2][0] * self.x + matrix[2][1] * self.y + matrix[2][2] * self.z;
        Self::new(x, y, z)
    }

    /// Interprets the position as rotation angles and converts it to a rotation matrix.
    ///
    /// `x`, `y` and `z` are interpreted as angles around each axis in radians.
    #[inline]
    pub fn as_rotation_matrix(&self) -> [[f32; 3]; 3] {
        let Self {
            x: alpha,
            y: beta,
            z: gamma,
        } = self;
        [
            [
                beta.cos() * gamma.cos(),
                alpha.sin() * beta.sin() * gamma.cos() - alpha.cos() + gamma.sin(),
                alpha.cos() * beta.sin() * gamma.cos() + alpha.sin() * gamma.sin(),
            ],
            [
                beta.cos() * gamma.sin(),
                alpha.sin() * beta.sin() * gamma.sin() + alpha.cos() + gamma.cos(),
                alpha.cos() * beta.sin() * gamma.sin() - alpha.sin() * gamma.cos(),
            ],
            [
                -beta.sin(),
                alpha.sin() * beta.cos(),
                alpha.cos() * beta.cos(),
            ],
        ]
    }

    /// Interprets the position as rotation angles and rotates the given vector.
    #[inline]
    pub fn rotate(&self, vector: Self) -> Self {
        vector.mat_mul(self.as_rotation_matrix())
    }

    /// Performs a component-wise operation with another [`Position`].
    #[inline]
    fn component_wise_op(&self, other: &Position, op: impl Fn(f32, f32) -> f32) -> Self {
        Self::new(
            op(self.x, other.x),
            op(self.y, other.y),
            op(self.z, other.z),
        )
    }

    /// Performs a scalar operation with a [`Position`] and a [`f32`].
    #[inline]
    fn scalar_op(&self, scalar: f32, op: impl Fn(f32, f32) -> f32) -> Self {
        Self::new(op(self.x, scalar), op(self.y, scalar), op(self.z, scalar))
    }
}

impl From<[f32; 3]> for Position {
    #[inline]
    fn from(value: [f32; 3]) -> Self {
        let [x, y, z] = value;
        Self { x, y, z }
    }
}

impl From<Position> for [f32; 3] {
    #[inline]
    fn from(pos: Position) -> Self {
        pos.to_array()
    }
}

impl From<(f32, f32, f32)> for Position {
    #[inline]
    fn from(value: (f32, f32, f32)) -> Self {
        let (x, y, z) = value;
        Self { x, y, z }
    }
}

impl From<Position> for (f32, f32, f32) {
    #[inline]
    fn from(pos: Position) -> Self {
        pos.to_tuple()
    }
}

impl Add for &Position {
    type Output = Position;

    #[inline]
    fn add(self, rhs: &Position) -> Self::Output {
        self.component_wise_op(rhs, Add::add)
    }
}

impl AddAssign for Position {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = &*self + &rhs
    }
}

impl Sub for &Position {
    type Output = Position;

    #[inline]
    fn sub(self, rhs: &Position) -> Self::Output {
        self.component_wise_op(rhs, Sub::sub)
    }
}

impl SubAssign for Position {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = &*self - &rhs
    }
}

impl Mul<f32> for &Position {
    type Output = Position;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        self.scalar_op(rhs, Mul::mul)
    }
}

impl MulAssign<f32> for Position {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = &*self * rhs;
    }
}

impl Mul<&Position> for f32 {
    type Output = Position;

    #[inline]
    fn mul(self, rhs: &Position) -> Self::Output {
        rhs.scalar_op(self, Mul::mul)
    }
}

impl Div<f32> for &Position {
    type Output = Position;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self.scalar_op(rhs, Div::div)
    }
}

impl DivAssign<f32> for Position {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self = &*self / rhs;
    }
}

impl Div<&Position> for f32 {
    type Output = Position;

    #[inline]
    fn div(self, rhs: &Position) -> Self::Output {
        rhs.scalar_op(self, Div::div)
    }
}

impl Extract for Position {
    #[inline]
    unsafe fn extract(event: &Event) -> Self {
        let [x, y]: [f32; 2] = transmute(event.dst_agent);
        let z = f32::from_ne_bytes(event.value.to_ne_bytes());
        Self::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f32::consts::{FRAC_1_SQRT_2, PI};

    #[test]
    fn mumble_conversion() {
        let pos = Position::new(3993.409, 6225.539, -549.570);

        let mumble = pos.to_mumble();
        assert_relative_eq!(
            *mumble.as_slice(),
            [101.433, 13.959, 158.129],
            max_relative = 1e-3
        );

        let back = Position::from_mumble(mumble);
        assert_eq!(back, pos);
    }

    #[test]
    fn rotation() {
        let rotation = Position::new(0.0, 0.25 * PI, 0.5 * PI);
        let vector = Position::new(1.0, 0.0, 0.0);

        let result = rotation.rotate(vector).to_array();
        assert_relative_eq!(
            *result.as_slice(),
            [0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2],
            max_relative = 1e-7
        );
    }
}
