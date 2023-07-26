use num_enum::{IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Breakbar (defiance bar) states.
///
/// Occurs in [`StateChange::BreakbarState`](crate::StateChange::BreakbarState) events.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u16)]
pub enum BreakbarState {
    /// Defiance bar active.
    Active = 0,

    /// Defiance bar recovering.
    Recover = 1,

    /// Defiance bar immune.
    Immune = 2,

    /// No defiance.
    None = 3,
}
