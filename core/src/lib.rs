//! ArcDPS bindings for Rust.
//!
//! # Macro usage
//! To see which fields are supported, have a look at [SupportedFields].

#![allow(clippy::missing_safety_doc)]

pub mod api;
pub mod callbacks;
pub mod exports;
pub mod extras;
pub mod instance;
pub mod util;

pub use api::{evtc::*, game::*, Agent, AgentOwned, CombatEvent};
pub use arcdps_codegen::arcdps_export;
pub use arcdps_imgui as imgui;

use callbacks::*;
use extras::callbacks::*;

/// Reference on what fields are currently supported by the [`arcdps_export!`](arcdps_codegen::arcdps_export) macro.
/// This struct is not used anywhere.
pub struct SupportedFields {
    pub name: &'static str,
    pub sig: u32,
    pub init: Option<InitFunc>,
    pub release: Option<ReleaseFunc>,
    pub raw_wnd_nofilter: Option<RawWndprocCallback>,
    pub raw_imgui: Option<RawImguiCallback>,
    pub raw_options_end: Option<RawOptionsCallback>,
    pub raw_combat: Option<RawCombatCallback>,
    pub raw_wnd_filter: Option<RawWndprocCallback>,
    pub raw_options_windows: Option<RawOptionsWindowsCallback>,
    pub raw_combat_local: Option<RawCombatCallback>,
    pub raw_unofficial_extras_init: Option<RawExtrasSubscriberInit>,
    pub raw_unofficial_extras_squad_update: Option<RawSquadUpdateCallback>,
    pub wnd_nofilter: Option<WndProcCallback>,
    pub combat: Option<CombatCallback>,
    pub imgui: Option<ImguiCallback>,
    pub options_end: Option<OptionsCallback>,
    pub combat_local: Option<CombatCallback>,
    pub wnd_filter: Option<WndProcCallback>,
    pub options_windows: Option<OptionsWindowsCallback>,
    pub unofficial_extras_init: Option<ExtrasInitFunc>,
    pub unofficial_extras_squad_update: Option<ExtrasSquadUpdateCallback>,
}

/// Exports for usage in macros.
#[doc(hidden)]
pub mod __macro {
    pub use crate::{
        callbacks::*,
        extras::callbacks::*,
        imgui,
        instance::{FreeFn, MallocFn},
        util::str_from_cstr,
    };
    pub use std::os::raw::{c_char, c_void};
    pub use windows::Win32::{
        Foundation::{HINSTANCE, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP},
    };

    use crate::instance::{init_imgui, ARC_INSTANCE};

    /// Internally used function to initialize with information received from Arc.
    #[inline]
    pub unsafe fn __init(
        arc_version: *const c_char,
        arc_handle: HINSTANCE,
        imgui_ctx: *mut imgui::sys::ImGuiContext,
        malloc: Option<MallocFn>,
        free: Option<FreeFn>,
        _id3d: *mut c_void,
        _name: &'static str,
    ) {
        init_imgui(imgui_ctx, malloc, free);
        ARC_INSTANCE.init(arc_handle, str_from_cstr(arc_version));
    }

    /// Internally used function to retrieve the [`imgui::Ui`].
    #[inline]
    pub unsafe fn __ui() -> &'static imgui::Ui<'static> {
        ARC_INSTANCE.ui.as_ref().unwrap()
    }
}
