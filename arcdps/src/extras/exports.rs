//! Unofficial Extras exports.
//!
//! Calling an export before Unofficial Extras calls `init` will cause a panic.

/// Raw Unofficial Extras exports.
pub mod raw {
    use crate::extras::{
        globals::EXTRAS_GLOBALS,
        keybinds::{RawKey, RawKeybind},
        Control,
    };

    pub type ExportGetKey = unsafe extern "C" fn(control: Control, key_index: u32) -> RawKey;
    pub type ExportGetKeybind = unsafe extern "C" fn(control: Control) -> RawKeybind;

    /// Retrieves [`RawKey`] for a given [`Control`] from Unofficial Extras.
    /// `key_index` can be `0` or `1` for primary/secondary keybind respectively.
    pub unsafe fn get_key(control: Control, key_index: u32) -> RawKey {
        EXTRAS_GLOBALS
            .get_key
            .expect("failed to find extras export get_key")(control, key_index)
    }

    /// Retrieves [`RawKeybind`] for a given [`Control`] from Unofficial Extras.
    pub unsafe fn get_keybind(control: Control) -> RawKeybind {
        EXTRAS_GLOBALS
            .get_keybind
            .expect("failed to find extras export get_key")(control)
    }
}
