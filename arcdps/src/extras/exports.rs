//! Unofficial Extras exports.
//!
//! Calling an export before Unofficial Extras calls `extras_init` will cause a panic.

use crate::extras::{globals::EXTRAS_GLOBALS, Control, Key, Keybind};

/// Retrieves the Unofficial Extras version as string.
#[inline]
pub fn version() -> Option<&'static str> {
    unsafe { EXTRAS_GLOBALS.version }
}

/// Retrieves the [`Key`] for a given game [`Control`] from Unofficial Extras.
/// `secondary` determines whether the primary or secondary bind.
///
/// Returns an empty/default [`Key`] if the key is not set
/// **or** if the functionality is disabled cause of missing patterns.
///
/// # Examples
/// ```no_run
/// use arcdps::extras::{keybinds::Control, exports::get_key};
///
/// let primary = get_key(Control::Skills_EliteSkill, false);
/// let secondary = get_key(Control::Skills_EliteSkill, true);
/// ```
pub fn get_key(control: Control, secondary: bool) -> Key {
    unsafe { raw::get_key(control, secondary as u32) }.into()
}

/// Retrieves the [`Keybind`] for a given game [`Control`] from Unofficial Extras.
///
/// Returns an empty/default [`Key`] if the key is not set
/// **or** if the functionality is disabled cause of missing patterns.
///
/// # Examples
/// ```no_run
/// use arcdps::extras::{keybinds::Control, exports::get_keybind};
///
/// let keybind = get_keybind(Control::Skills_EliteSkill);
/// let primary = keybind.primary;
/// let secondary = keybind.secondary;
/// ```
pub fn get_keybind(control: Control) -> Keybind {
    unsafe { raw::get_keybind(control) }.into()
}

/// Raw Unofficial Extras exports.
pub mod raw {
    use crate::extras::{
        globals::EXTRAS_GLOBALS,
        keybinds::{RawKey, RawKeybind},
        Control,
    };

    /// Signature of the [`get_key`] export.
    pub type ExportGetKey = unsafe extern "C" fn(control: Control, key_index: u32) -> RawKey;

    /// Retrieves the [`RawKey`] for a given game [`Control`] from Unofficial Extras.
    /// `key_index` can be `0` or `1` for primary/secondary keybind respectively.
    #[inline]
    pub unsafe fn get_key(control: Control, key_index: u32) -> RawKey {
        EXTRAS_GLOBALS
            .get_key
            .expect("failed to find extras export get_key")(control, key_index)
    }

    /// Signature of the [`get_keybind`] export.
    pub type ExportGetKeybind = unsafe extern "C" fn(control: Control) -> RawKeybind;

    /// Retrieves the [`RawKeybind`] for a given game [`Control`] from Unofficial Extras.
    #[inline]
    pub unsafe fn get_keybind(control: Control) -> RawKeybind {
        EXTRAS_GLOBALS
            .get_keybind
            .expect("failed to find extras export get_key")(control)
    }
}
