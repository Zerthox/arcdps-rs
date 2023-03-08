use crate::globals::ARC_GLOBALS;

/// Checks whether export `e0` was found.
#[inline]
pub fn has_e0_config_path() -> bool {
    unsafe { ARC_GLOBALS.e0 }.is_some()
}

/// Checks whether export `e3` was found.
#[inline]
pub fn has_e3_log_file() -> bool {
    unsafe { ARC_GLOBALS.e3 }.is_some()
}

/// Checks whether export `e5` was found.
#[inline]
pub fn has_e5_colors() -> bool {
    unsafe { ARC_GLOBALS.e5 }.is_some()
}

/// Checks whether export `e6` was found.
#[inline]
pub fn has_e6_ui_settings() -> bool {
    unsafe { ARC_GLOBALS.e6 }.is_some()
}

/// Checks whether export `e7` was found.
#[inline]
pub fn has_e7_modifiers() -> bool {
    unsafe { ARC_GLOBALS.e7 }.is_some()
}

/// Checks whether export `e8` was found.
#[inline]
pub fn has_e8_log_window() -> bool {
    unsafe { ARC_GLOBALS.e8 }.is_some()
}

/// Checks whether export `e9` was found.
#[inline]
pub fn has_e9_add_event() -> bool {
    unsafe { ARC_GLOBALS.e9 }.is_some()
}

/// Checks whether export `e10` was found.
#[inline]
pub fn has_e10_add_event_combat() -> bool {
    unsafe { ARC_GLOBALS.e10 }.is_some()
}

/// Checks whether export `addextension` (old) was found.
#[inline]
pub fn has_add_extension_old() -> bool {
    unsafe { ARC_GLOBALS.add_extension_old }.is_some()
}

/// Checks whether export `freeextension` (old) was found.
#[inline]
pub fn has_free_extension_old() -> bool {
    unsafe { ARC_GLOBALS.free_extension_old }.is_some()
}

/// Checks whether export `addextension2` was found.
#[inline]
pub fn has_add_extension() -> bool {
    unsafe { ARC_GLOBALS.add_extension }.is_some()
}

/// Checks whether export `freeextension2` was found.
#[inline]
pub fn has_free_extension() -> bool {
    unsafe { ARC_GLOBALS.free_extension }.is_some()
}

/// Checks whether export `listextension` was found.
#[inline]
pub fn has_list_extension() -> bool {
    unsafe { ARC_GLOBALS.list_extension }.is_some()
}
