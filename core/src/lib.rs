#![allow(dead_code)]
pub mod helpers;
mod raw_structs;

pub use raw_structs::*;

pub use arcdps_codegen::*;
pub use imgui;

// pub type SafeWndprocCallback = ?

pub type SafeOptionsWindowsCallback = fn(window_name: Option<&str>);
