//! ArcDPS bindings for Rust.
//!
//! # Macro usage
//! To see which fields are supported, have a look at [SupportedFields].

#![allow(clippy::missing_safety_doc)]

pub mod api;
pub mod callbacks;
pub mod exports;
pub mod instance;
pub mod util;

#[cfg(feature = "extras")]
pub mod extras;

pub use api::{evtc::*, game::*, Agent, AgentOwned, CombatEvent};
pub use arcdps_codegen::export;
pub use arcdps_imgui as imgui;

use callbacks::*;

#[cfg(feature = "extras")]
use extras::callbacks::*;

/// Reference on what fields are currently supported by the [`export!`] macro.
///
/// This struct is not used anywhere.
// TODO: document fields?
// TODO: add note for extras fields
pub struct SupportedFields {
    /// Name of the plugin.
    pub name: &'static str,

    /// Unique signature of the plugin.
    ///
    /// Pick a random number that is not used by other modules.
    pub sig: u32,

    /// Callback for plugin load.
    pub init: Option<InitFunc>,

    /// Callback for plugin unload.
    pub release: Option<ReleaseFunc>,

    /// Raw WndProc callback.
    pub raw_wnd_nofilter: Option<RawWndprocCallback>,

    /// Raw Imgui callback.
    pub raw_imgui: Option<RawImguiCallback>,

    /// Raw options callback.
    pub raw_options_end: Option<RawOptionsCallback>,

    /// Raw combat callback.
    pub raw_combat: Option<RawCombatCallback>,

    /// Raw filtered WndProc callback.
    pub raw_wnd_filter: Option<RawWndprocCallback>,

    /// Raw options windows callback.
    pub raw_options_windows: Option<RawOptionsWindowsCallback>,

    /// Raw local combat callback.
    pub raw_combat_local: Option<RawCombatCallback>,

    /// Callback for key presses.
    ///
    /// Returning `true` will allow ArcDPS and GW2 to receive the key press.
    /// First parameter indicates the [virtual key code](https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes).
    /// Second parameter is `true` if the key was pressed and `false` when released.
    /// Third parameter is `true` if the key was down before this event occurred, for example by holding it down.
    pub wnd_nofilter: Option<WndProcCallback>,

    /// Callback for area combat events.
    ///
    /// May be called asynchronously, use `id` to keep track of order.
    /// First event id will be `2`.
    ///
    /// At least one participant will be a party/squad member or minion of, or a buff applied by squad in the case of buff remove.
    /// Not all statechanges are present in the realtime API, see [`StateChange`](crate::StateChange) for details.
    ///
    /// No `event` and `src.elite == 0` indicates a tracking change.
    /// Player was added when `src.prof != 0`, otherwise removed.
    /// When added `dst.name` contains the account name,
    /// `dst.id` the instance id,
    /// `dst.prof` the [`Profession`](crate::Profession),
    /// `dst.elite` the elite [`Specialization`](crate::Specialization),
    /// `dst.is_self` whether the added player is self (local player),
    /// `src.team` the team,
    /// `dst.team` the subgroup.
    ///
    /// No `event` and `src.elite != 0` indicates a target change.
    /// `src.id` will contain the new target.
    pub combat: Option<CombatCallback>,

    /// Callback for standalone UI creation.
    ///
    /// Provides a [`imgui::Ui`] object that is needed to draw anything.
    /// The second parameter is `true` whenever the player is **not** in character select, loading screens or forced cameras.
    pub imgui: Option<ImguiCallback>,

    /// Callback for plugin settings UI creation.
    ///
    /// Provides a [`imgui::Ui`] object that is needed to draw anything.
    pub options_end: Option<OptionsCallback>,

    /// Callback for local combat events.
    ///
    /// Same as [`combat`](Self::combat) but for events from chat log.
    pub combat_local: Option<CombatCallback>,

    /// Callback for filtered key presses.
    ///
    /// Same as [`wnd_nofilter`](Self::wnd_nofilter) but filtered to only notify when modifiers are pressed.
    pub wnd_filter: Option<WndProcCallback>,

    /// Callback for options windows.
    ///
    /// Called for each window checkbox in ArcDPS settings.
    /// Last call will always be with [`None`].
    /// Does not draw the checkbox if returning `true`.
    pub options_windows: Option<OptionsWindowsCallback>,

    /// Raw extras init callback.
    ///
    /// *Requires the `"extras"` feature.*
    #[cfg(feature = "extras")]
    pub raw_extras_init: Option<RawExtrasSubscriberInit>,

    /// Raw extras squad update callback.
    ///
    /// *Requires the `"extras"` feature.*
    #[cfg(feature = "extras")]
    pub raw_extras_squad_update: Option<RawSquadUpdateCallback>,

    /// Initialization callback for [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases).
    ///
    /// Can be called before or after ArcDPS [`init`](Self::init).
    /// Receives information about the Unofficial Extras addon and the current player account name as parameters.
    ///
    /// *Requires the `"extras"` feature.*
    #[cfg(feature = "extras")]
    pub extras_init: Option<ExtrasInitFunc>,

    /// Squad update callback for [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases).
    ///
    /// Called whenever anything in the squad changes.
    /// Only the users that changed are sent.
    /// If a user is removed, their `role` will be set to [`UserRole::None`](crate::extras::UserRole::None).
    /// *Requires the `"extras"` feature.*
    #[cfg(feature = "extras")]
    pub extras_squad_update: Option<ExtrasSquadUpdateCallback>,
}

/// Exports for usage in macros.
#[doc(hidden)]
pub mod __macro {
    pub use crate::{
        callbacks::*,
        imgui,
        instance::{FreeFn, MallocFn},
        util::str_from_cstr,
    };
    pub use std::os::raw::{c_char, c_void};
    pub use windows::Win32::{
        Foundation::{HINSTANCE, LPARAM, WPARAM},
        UI::WindowsAndMessaging::{WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP},
    };

    #[cfg(feature = "extras")]
    pub use crate::extras::callbacks::*;

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
