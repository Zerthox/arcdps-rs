use crate::{Affinity, AgentId, Event};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Information common to combat events.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommonEvent {
    /// Time of registering the event.
    pub time: u64,

    /// Agent that caused the event.
    pub source: AgentId,

    /// Agent the event happened to.
    pub target: AgentId,

    /// Skill id of the relevant skill (can be zero).
    pub skill_id: u32,

    /// Current affinity of `src` and `dst`.
    ///
    /// *Arc calls this "iff" for if friend/foe.*
    pub affinity: Affinity,

    /// Whether `src` is above 90% Health.
    pub is_ninety: u8,

    /// Whether `dst` is below 50% Health.
    pub is_fifty: u8,

    /// Whether `src` is moving at time of event.
    pub is_moving: u8,

    /// Whether `src` is flanking at time of event.
    ///
    /// The value lies in a range of `1` to `135` degrees where `135` is rear.
    pub is_flanking: u8,
}

impl From<&Event> for CommonEvent {
    #[inline]
    fn from(event: &Event) -> Self {
        Self {
            time: event.time,
            source: AgentId::from_src(event),
            target: AgentId::from_dst(event),
            skill_id: event.skill_id,
            affinity: event.get_affinity(),
            is_ninety: event.is_ninety,
            is_fifty: event.is_fifty,
            is_moving: event.is_moving,
            is_flanking: event.is_flanking,
        }
    }
}

/// Helper macro to implement traits for events with a [`CommonEvent`] field.
macro_rules! impl_common {
    ($ty:ty) => {
        impl ::core::convert::AsRef<$crate::event::CommonEvent> for $ty {
            #[inline]
            fn as_ref(&self) -> &$crate::event::CommonEvent {
                &self.common
            }
        }

        impl ::core::convert::AsMut<$crate::event::CommonEvent> for $ty {
            #[inline]
            fn as_mut(&mut self) -> &mut $crate::event::CommonEvent {
                &mut self.common
            }
        }

        impl ::core::convert::From<$ty> for $crate::event::CommonEvent {
            #[inline]
            fn from(value: $ty) -> Self {
                value.common
            }
        }

        impl ::core::ops::Deref for $ty {
            type Target = $crate::event::CommonEvent;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.common
            }
        }

        impl ::core::ops::DerefMut for $ty {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.common
            }
        }
    };
}

pub(crate) use impl_common;
