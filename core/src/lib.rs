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

use instance::ArcInstance;
use std::os::raw::c_char;
use util::str_from_cstr;
use windows::Win32::Foundation::HINSTANCE;

/// Internally used function to initialize information about Arc.
// TODO: use bool for error
#[doc(hidden)]
pub unsafe fn __init(arc_version: *const c_char, handle: HINSTANCE, _name: &'static str) -> bool {
    ArcInstance::init(handle, str_from_cstr(arc_version))
}
