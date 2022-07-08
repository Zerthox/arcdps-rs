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
