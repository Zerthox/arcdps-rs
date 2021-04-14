//! # Heavy WIP
//! To see which fields are supported, have a look at [SupportedFields]

pub mod helpers;
mod raw_structs;

pub use raw_structs::*;

pub use arcdps_codegen::*;
pub use imgui;

// pub type SafeWndprocCallback = ?

/// This struct isn't used anywhere. It is a reference on what fields are currently supported by the [arcdps_export!] macro.
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
    // pub wnd_nofilter: Option<Expr>,
    pub combat: Option<CombatCallback>,
    pub imgui: Option<ImguiCallback>,
    pub options_end: Option<OptionsCallback>,
    pub combat_local: Option<CombatCallback>,
    // pub wnd_filter: Option<Expr>,
    pub options_windows: Option<OptionsWindowsCallback>,
}
