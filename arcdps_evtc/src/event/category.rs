use crate::{Activation, BuffRemove, CombatEvent, StateChange};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Possible [`CombatEvent`] categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
pub enum EventCategory {
    /// State change event. See variants of [`StateChange`] for details.
    StateChange,

    /// Activation (cast) event.
    ///
    /// `is_activation` contains [`Activation`] (except [`Activation::None`]).
    ///
    /// For [`Activation::Normal`] and [`Activation::Quickness`]:
    /// `value` contains the duration at which all "significant" effects associated with the cast have happened (for example damage hits).
    /// `buff_dmg` contains the duration at which control is expected to be returned to the character (for example aftercasts).
    ///
    /// For [`Activation::CancelFire`] and [`Activation::CancelCancel`]:
    /// `value` contains the time spent in animation.
    /// `buff_dmg` contains the duration of the scaled (as if not affected) time spent.
    ///
    /// `dst_agent` contains x/y of target of skill effect.
    /// `overstack_value` contains z of target of skill effect.
    ///
    /// All durations and times are given in milliseconds.
    ///
    /// For skill data see [`SkillInfo`] and [`SkillTiming`].
    Activation,

    /// Buff removed.
    ///
    /// `is_buffremove` contains [`BuffRemove`] (except [`BuffRemove::None`]).
    /// `skill_id` contains the buff id.
    /// `buff` will be non-zero.
    ///
    /// `src_agent` is agent that had buff removed.
    /// `dst_agent` is the agent that caused the buff to be removed.
    ///
    /// `value` contains the remaining time on the removed buff calculated as duration.
    /// `buff_dmg` contains the remaining time on the removed buff calculated as intensity
    /// *Warning: this can overflow on [`BuffRemove::All`], use as sum check only!*
    ///
    /// For [`BuffRemove::All`] `result` contains the number of stacks removed
    ///
    /// For [`BuffRemove::Single`] `pad61` to `pad64` contains the buff instance id of buff removed.
    ///
    /// For buff data see [`BuffInfo`] and [`BuffFormula`].
    BuffRemove,

    /// Buff applied.
    ///
    /// `skill_id` contains the buff id.
    /// `buff` will be non-zero.
    ///
    /// `value` contains the duration in milliseconds applied.
    /// `pad61` to `pad64` contains the buff instance id of the buff applied.
    /// `is_shields` contains the stack active status.
    ///
    /// If `is_offcycle == 0`, `overstack_value` contains the duration of the existing buff stack that is expected to be replaced.
    /// If `is_offcycle != 0`, `overstack_value` contains the new duration of the existing buff stack and `value` contains the duration change (no new buff stack added).
    ///
    /// For buff data see [`BuffInfo`] and [`BuffFormula`].
    BuffApply,

    /// Buff damage.
    ///
    /// `skill_id` contains the buff id.
    /// `buff` will be non-zero.
    ///
    /// `buff_dmg` contains the damage dealt in Arc's damage simulation.
    /// If `is_offcycle == 0`, damage is accumulated by tick (for example Bleeding tick).
    /// If `is_offcycle != 0`, damage is accumulated reactively (for example Confusion damage on skill use).
    /// `result` contains `0` if expected to hit, `1` for invulnerability by buff and `2`/`3`/`4` for invulnerability by skill.
    ///
    /// For buff data see [`BuffInfo`] and [`BuffFormula`].
    BuffDamage,

    /// Direct (strike) damage.
    ///
    /// `value` contains the combined shield (barrier) and health damage dealt.
    /// `overstack_value` contains the shield (barrier) damage dealt.
    /// `is_offcycle == 1` if target is currently downed.
    /// `result` contains [`Strike`](crate::Strike).
    DirectDamage,
}

impl From<&CombatEvent> for EventCategory {
    #[inline]
    fn from(event: &CombatEvent) -> Self {
        if event.is_statechange != StateChange::None {
            EventCategory::StateChange
        } else if event.is_activation != Activation::None {
            EventCategory::Activation
        } else if event.is_buffremove != BuffRemove::None {
            EventCategory::BuffRemove
        } else if event.buff != 0 {
            if event.buff_dmg == 0 {
                EventCategory::BuffApply
            } else {
                EventCategory::BuffDamage
            }
        } else {
            EventCategory::DirectDamage
        }
    }
}
