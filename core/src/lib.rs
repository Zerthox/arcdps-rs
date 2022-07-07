//! ArcDPS bindings for Rust.
//!
//! # Macro usage
//! To see which fields are supported, have a look at [SupportedFields].

pub mod api;
pub mod callbacks;
pub mod exports;
pub mod extras;
pub mod instance;
pub mod util;

pub use arcdps_codegen::*;
pub use arcdps_imgui as imgui;

use crate::instance::ArcInstance;
use std::{ffi::CStr, os::raw::c_char};
use windows::Win32::Foundation::HINSTANCE;

/// Internally used function to initialize information about Arc.
// TODO: use bool for error
#[doc(hidden)]
pub unsafe fn __init(arc_version: *const c_char, handle: HINSTANCE, _name: &'static str) -> bool {
    ArcInstance::init(handle, CStr::from_ptr(arc_version))
}
