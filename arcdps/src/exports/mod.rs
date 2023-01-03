//! ArcDPS exports.
//!
//! Calling an export before ArcDPS calls `init` will cause a panic.

pub mod raw;

use crate::{
    api::{CombatEvent, Profession},
    globals::ARC_GLOBALS,
    imgui::sys::ImVec4,
};
use raw::{
    e0_config_path, e3_log_file, e5_colors, e6_ui_settings, e7_modifiers, e8_log_window,
    e9_add_event,
};
use std::{
    ffi::{CString, NulError, OsString},
    mem::MaybeUninit,
    ops::RangeInclusive,
    os::windows::prelude::*,
    path::PathBuf,
    slice,
};

/// Retrieves the ArcDPS version as string.
pub fn version() -> Option<&'static str> {
    unsafe { ARC_GLOBALS.version }
}

/// Retrieves the config path from ArcDPS.
pub fn config_path() -> Option<PathBuf> {
    let ptr = unsafe { e0_config_path() };
    if !ptr.is_null() {
        // calculate length
        let mut len = 0;
        while unsafe { *ptr.offset(len) } != 0 {
            len += 1;
        }

        // transform data
        let slice = unsafe { slice::from_raw_parts(ptr, len as usize) };
        let string = OsString::from_wide(slice);
        Some(PathBuf::from(string))
    } else {
        None
    }
}

/// Logs a message to ArcDPS' log file `arcdps.log`.
///
/// Returns an error if the passed message could not be converted to a C-compatible string.
pub fn log_to_file(message: impl Into<String>) -> Result<(), NulError> {
    let string = CString::new(message.into())?;
    unsafe { e3_log_file(string.as_ptr()) };
    Ok(())
}

/// ArcDPS core UI color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum CoreColor {
    Transparent,
    White,
    LightWhite,
    LightGrey,
    LightYellow,
    LightGreen,
    LightRed,
    LightTeal,
    MediumGrey,
    DarkGrey,
    Num,
}

/// ArcDPS color type.
pub type Color = [f32; 4];

/// Current ArcDPS color settings.
///
/// Use the associated functions to access individual colors.
///
/// This holds pointers to color information in memory until dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Colors {
    raw: [*mut ImVec4; 5],
}

impl Colors {
    /// Range of valid subgroups.
    const SUB_RANGE: RangeInclusive<usize> = 0..=15;

    /// Reads a color from the raw color array.
    ///
    /// The first index is the index of the subarray.
    /// The second index is the index of the color within the subarray.
    ///
    /// This will return [`None`] if the pointer retrieved from ArcDPS is null or was not initialized.
    ///
    /// This is unsafe since indexing the raw color array is only valid with specific indices.
    /// Incorrect indices may cause undefined behavior.
    unsafe fn read_color(&self, first_index: usize, second_index: usize) -> Option<Color> {
        let ptr = self.raw[first_index];
        if !ptr.is_null() {
            // we do not need the full length slice
            let slice = slice::from_raw_parts(ptr, second_index + 1);
            Some(slice[second_index].into())
        } else {
            None
        }
    }

    /// Returns the base color for a specific [`CoreColor`].
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    pub fn core(&self, color: CoreColor) -> Option<Color> {
        unsafe { self.read_color(0, color as usize) }
    }

    /// Returns the base color for a specific [`Profession`].
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    pub fn prof_base(&self, prof: Profession) -> Option<Color> {
        unsafe { self.read_color(1, prof as usize) }
    }

    /// Returns the highlight color for a specific [`Profession`].
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    pub fn prof_highlight(&self, prof: Profession) -> Option<Color> {
        unsafe { self.read_color(2, prof as usize) }
    }

    /// Returns the base color for a specific subgroup.
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    /// Also returns [`None`] if the subgroup is out of the game subgroup range.
    pub fn sub_base(&self, sub: usize) -> Option<Color> {
        // range check
        if Self::SUB_RANGE.contains(&sub) {
            unsafe { self.read_color(3, sub) }
        } else {
            None
        }
    }

    /// Returns the highlight color for a specific subgroup.
    ///
    /// This will return [`None`] if ArcDPS did not yield the requested color when the [`Colors`] struct was retrieved.
    /// Also returns [`None`] if the subgroup is out of the game subgroup range.
    pub fn sub_highlight(&self, sub: usize) -> Option<Color> {
        // range check
        if Self::SUB_RANGE.contains(&sub) {
            unsafe { self.read_color(4, sub) }
        } else {
            None
        }
    }
}

/// Retrieves the color settings from ArcDPS.
pub fn colors() -> Colors {
    // zeroing this is important for our null pointer checks later
    let mut colors = MaybeUninit::zeroed();
    unsafe { e5_colors(colors.as_mut_ptr()) };
    Colors {
        raw: unsafe { colors.assume_init() },
    }
}

/// Current ArcDPS UI settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UISettings {
    /// Whether the UI is hidden.
    pub hidden: bool,

    /// Whether the UI is always drawn.
    ///
    /// When `false`, the UI is hidden during loading screens & character select.
    pub draw_always: bool,

    /// Whether pressing the modifiers are required to move windows.
    pub modifiers_move_lock: bool,

    /// Whether pressing the modifiers are required to click windows.
    pub modifiers_click_lock: bool,

    /// Whether windows should close with the `ESC` key.
    pub close_with_esc: bool,
}

/// Retrieves the UI settings from ArcDPS.
pub fn ui_settings() -> UISettings {
    let raw = unsafe { e6_ui_settings() };
    UISettings {
        hidden: raw & 1 == 1,
        draw_always: (raw >> 1) & 1 == 1,
        modifiers_move_lock: (raw >> 2) & 1 == 1,
        modifiers_click_lock: (raw >> 3) & 1 == 1,
        close_with_esc: (raw >> 4) & 1 == 1,
    }
}

/// Current ArcDPS modifier keybinds as virtual key ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modifiers {
    /// Virtual key id of the first modifier used by ArcDPS.
    pub modifier1: u16,

    /// Virtual key id of the second modifier used by ArcDPS.
    pub modifier2: u16,

    /// Virtual key id of the "multi" modifier used by ArcDPS.
    pub modifier_multi: u16,
}

/// Retrieves the modifier keybinds from ArcDPS.
pub fn modifiers() -> Modifiers {
    let raw = unsafe { e7_modifiers() };
    Modifiers {
        modifier1: raw as u16,
        modifier2: (raw >> 16) as u16,
        modifier_multi: (raw >> 32) as u16,
    }
}
/// Logs a message to ArcDPS' logger window.
///
/// Text can be colored in a HTML-like way: `<c=#aaaaaa>colored text</c>`.
///
/// Returns an error if the passed message could not be converted to a C-compatible string.
pub fn log_to_window(message: impl Into<String>) -> Result<(), NulError> {
    let string = CString::new(message.into())?;
    unsafe { e8_log_window(string.as_ptr()) };
    Ok(())
}

/// Adds a [`CombatEvent`] to ArcDPS' event processing.
///
/// `is_statechange` will be set to [`StateChange::Extension`](crate::StateChange::Extension), padding will be set to contain `sig`.
/// Event will end up processed like ArcDPS events and logged to EVTC.
pub fn add_event(event: CombatEvent, sig: u32) {
    unsafe { e9_add_event(&event.into(), sig) }
}
