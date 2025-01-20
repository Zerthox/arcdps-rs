//! Unofficial Extras exports.
//!
//! Calling an export before Unofficial Extras calls `extras_init` will cause a panic.

use crate::extras::{globals::ExtrasGlobals, Control, Key, Keybind};

/// Retrieves the Unofficial Extras version as string.
#[inline]
pub fn version() -> Option<&'static str> {
    ExtrasGlobals::get().version
}

/// Checks whether the `get_key` export was found.
#[inline]
pub fn has_get_key() -> bool {
    ExtrasGlobals::get().get_key.is_some()
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
#[inline]
pub fn get_key(control: Control, secondary: bool) -> Key {
    unsafe { raw::get_key(control, secondary as u32) }.into()
}

/// Checks whether the `get_keybind` export was found.
#[inline]
pub fn has_get_keybind() -> bool {
    ExtrasGlobals::get().get_keybind.is_some()
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
#[inline]
pub fn get_keybind(control: Control) -> Keybind {
    unsafe { raw::get_keybind(control) }.into()
}

/// Raw Unofficial Extras exports.
pub mod raw {
    use crate::extras::{
        globals::ExtrasGlobals,
        keybinds::{RawKey, RawKeybind},
        Control,
    };
    use windows::Win32::Foundation::HMODULE;

    /// Returns the handle to the Unofficial Extras dll.
    pub unsafe fn handle() -> HMODULE {
        ExtrasGlobals::get().handle
    }

    /// Signature of the [`get_key`] export.
    pub type ExportGetKey = unsafe extern "C" fn(control: Control, key_index: u32) -> RawKey;

    /// Retrieves the [`RawKey`] for a given game [`Control`] from Unofficial Extras.
    /// `key_index` can be `0` or `1` for primary/secondary keybind respectively.
    #[inline]
    pub unsafe fn get_key(control: Control, key_index: u32) -> RawKey {
        ExtrasGlobals::get()
            .get_key
            .expect("failed to find extras export get_key")(control, key_index)
    }

    /// Signature of the [`get_keybind`] export.
    pub type ExportGetKeybind = unsafe extern "C" fn(control: Control) -> RawKeybind;

    /// Retrieves the [`RawKeybind`] for a given game [`Control`] from Unofficial Extras.
    #[inline]
    pub unsafe fn get_keybind(control: Control) -> RawKeybind {
        ExtrasGlobals::get()
            .get_keybind
            .expect("failed to find extras export get_key")(control)
    }
}
