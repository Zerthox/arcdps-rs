use crate::globals::arc::ArcGlobals;

/// Checks whether export `e0` was found.
#[inline]
pub fn has_e0_config_path() -> bool {
    ArcGlobals::get().e0.is_some()
}

/// Checks whether export `e3` was found.
#[inline]
pub fn has_e3_log_file() -> bool {
    ArcGlobals::get().e3.is_some()
}

/// Checks whether export `e5` was found.
#[inline]
pub fn has_e5_colors() -> bool {
    ArcGlobals::get().e5.is_some()
}

/// Checks whether export `e6` was found.
#[inline]
pub fn has_e6_ui_settings() -> bool {
    ArcGlobals::get().e6.is_some()
}

/// Checks whether export `e7` was found.
#[inline]
pub fn has_e7_modifiers() -> bool {
    ArcGlobals::get().e7.is_some()
}

/// Checks whether export `e8` was found.
#[inline]
pub fn has_e8_log_window() -> bool {
    ArcGlobals::get().e8.is_some()
}

/// Checks whether export `e9` was found.
#[inline]
pub fn has_e9_add_event() -> bool {
    ArcGlobals::get().e9.is_some()
}

/// Checks whether export `e10` was found.
#[inline]
pub fn has_e10_add_event_combat() -> bool {
    ArcGlobals::get().e10.is_some()
}

/// Checks whether export `addextension2` was found.
#[inline]
pub fn has_add_extension() -> bool {
    ArcGlobals::get().add_extension.is_some()
}

/// Checks whether export `freeextension2` was found.
#[inline]
pub fn has_free_extension() -> bool {
    ArcGlobals::get().free_extension.is_some()
}

/// Checks whether export `listextension` was found.
#[inline]
pub fn has_list_extension() -> bool {
    ArcGlobals::get().list_extension.is_some()
}
