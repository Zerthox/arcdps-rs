//! # Macro usage
//! To see which fields are supported, have a look at [SupportedFields]

mod exported_functions;
pub mod helpers;
#[cfg(feature = "log")]
mod logging;
mod raw_structs;
mod unofficial_extras;

pub use arcdps_codegen::*;
pub use arcdps_imgui as imgui;
pub use exported_functions::*;
pub use raw_structs::*;
pub use unofficial_extras::raw_structs::*;

#[doc(hidden)]
#[inline(always)]
pub unsafe fn __init(
    arc_version: PCCHAR,
    arcdps: raw_structs::HANDLE,
    #[allow(unused)] name: &'static str,
) {
    __set_handle(arcdps);
    ARC_VERSION = helpers::get_str_from_pc_char(arc_version);
    #[cfg(feature = "log")]
    let _ = log::set_boxed_logger(Box::new(logging::ArcdpsLogger::new(name)))
        .map(|()| log::set_max_level(log::LevelFilter::Trace));
}

static mut ARC_VERSION: Option<&'static str> = None;

/// returns the loaded arcdps version
pub fn arcdps_version() -> &'static str {
    unsafe { ARC_VERSION.unwrap() }
}

/// This struct isn't used anywhere. It is a reference on what fields are
/// currently supported by the [arcdps_export!] macro.
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
    pub raw_unofficial_extras_init: Option<RawExtrasSubscriberInitSignature>,
    pub raw_unofficial_extras_squad_update: Option<RawSquadUpdateCallbackSignature>,
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
