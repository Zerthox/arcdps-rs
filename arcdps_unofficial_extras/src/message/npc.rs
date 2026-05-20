use crate::util::str_from_cstr_len;
use std::ffi::c_char;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An NPC chat message.
///
/// Strings are available for the duration of the call.
/// If you need it for longer than that, consider converting it to [`NpcMessageOwned`].
///
/// ```no_run
/// # use arcdps::extras::{NpcMessage, NpcMessageOwned};
/// # let message: &NpcMessage = todo!();
/// let owned = message.to_owned();
/// let owned: NpcMessageOwned = message.into();
/// ```
#[derive(Debug, Clone)]
#[repr(C)]
pub struct NpcMessage {
    /// Null terminated character name of the NPC that sent the message.
    ///
    /// The string is only valid for the duration of the call.
    character_name: *const c_char,
    character_name_length: u64,

    /// Null terminated string containing the content of the message that was sent.
    ///
    /// The string is only valid for the duration of the call.
    text: *const c_char,
    text_length: u64,

    /// Time since epoch in nanoseconds.
    ///
    /// This can be used to sort messages, when they are out of order.
    pub timestamp: u64,
}

impl NpcMessage {
    /// Converts the message to its owned counterpart.
    #[inline]
    pub fn to_owned(&self) -> NpcMessageOwned {
        self.into()
    }

    /// Returns the character name of the player that sent the message.
    #[inline]
    pub fn character_name(&self) -> &str {
        unsafe { str_from_cstr_len(self.character_name, self.character_name_length) }
    }

    /// Returns the character name as raw pointer.
    #[inline]
    pub fn character_name_ptr(&self) -> *const c_char {
        self.character_name
    }

    /// Returns the account name length.
    #[inline]
    pub fn character_name_len(&self) -> usize {
        self.character_name_length as _
    }

    /// Returns the text content of the message.
    #[inline]
    pub fn text(&self) -> &str {
        unsafe { str_from_cstr_len(self.text, self.text_length) }
    }

    /// Returns the text as raw pointer.
    #[inline]
    pub fn text_ptr(&self) -> *const c_char {
        self.text
    }

    /// Returns the account name length.
    #[inline]
    pub fn text_len(&self) -> usize {
        self.text_length as _
    }
}

/// [`NpcMessage`] with owned [`String`] fields.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NpcMessageOwned {
    /// Character name of the NPC that sent the message.
    pub character_name: String,

    /// Content of the message.
    pub text: String,
}

impl From<NpcMessage> for NpcMessageOwned {
    #[inline]
    fn from(msg: NpcMessage) -> Self {
        (&msg).into()
    }
}

impl From<&NpcMessage> for NpcMessageOwned {
    #[inline]
    fn from(msg: &NpcMessage) -> Self {
        Self {
            character_name: msg.character_name().to_owned(),
            text: msg.text().to_owned(),
        }
    }
}
