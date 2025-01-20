//! Message information provided by Unofficial Extras.

mod npc;
mod squad;

pub use self::{npc::*, squad::*};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// A chat message.
#[derive(Clone, Copy)]
#[repr(C)]
pub union RawMessage {
    squad: *const SquadMessage,
    npc: *const NpcMessage,
}

/// A chat message.
#[derive(Debug, Clone)]
pub enum Message<'a> {
    Squad(&'a SquadMessage),
    Npc(&'a NpcMessage),
}

impl Message<'_> {
    /// Creates a new message from [`RawMessageType`] and [`RawMessage`].
    #[inline]
    pub unsafe fn new(message_type: MessageType, message: RawMessage) -> Self {
        match message_type {
            MessageType::Squad => Self::Squad(
                message
                    .squad
                    .as_ref()
                    .expect("invalid unofficial extras squad message info"),
            ),
            MessageType::Npc => Self::Npc(
                message
                    .npc
                    .as_ref()
                    .expect("invalid unofficial extras npc message info"),
            ),
        }
    }
}

/// Type of message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(C)]
pub enum MessageType {
    /// Party or squad chat message.
    Squad = 0,

    /// NPC message (selectable in ingame-chat as "NPC").
    Npc = 1,
}
