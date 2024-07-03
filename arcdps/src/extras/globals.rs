//! Global instance with Unofficial Extras information.

use crate::{
    extras::exports::raw::{ExportGetKey, ExportGetKeybind},
    util::exported_proc,
};
use std::mem::transmute;
use windows::Win32::Foundation::HMODULE;

/// Global instance of Unofficial Extras handle & exported functions.
pub static mut EXTRAS_GLOBALS: ExtrasGlobals = ExtrasGlobals::empty();

/// Unofficial Extras handle & exported functions.
pub struct ExtrasGlobals {
    /// Handle to Unofficial Extras dll.
    pub handle: HMODULE,

    /// Unofficial Extras version as string.
    pub version: Option<&'static str>,

    /// Get key export.
    pub get_key: Option<ExportGetKey>,

    /// Get key bind export.
    pub get_keybind: Option<ExportGetKeybind>,
}

impl ExtrasGlobals {
    /// Creates an empty version of Unofficial Extras globals.
    const fn empty() -> Self {
        Self {
            handle: HMODULE(0),
            version: None,
            get_key: None,
            get_keybind: None,
        }
    }

    /// Initializes the Unofficial Extras globals.
    pub unsafe fn init(&mut self, handle: HMODULE, version: Option<&'static str>) {
        #![allow(clippy::missing_transmute_annotations)]
        *self = Self {
            handle,
            version,
            get_key: transmute(exported_proc(handle, "get_key\0")),
            get_keybind: transmute(exported_proc(handle, "get_key_bind\0")),
        };
    }
}
