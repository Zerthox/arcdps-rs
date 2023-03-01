//! Bindings for [ArcDPS](https://www.deltaconnected.com/arcdps/) plugins.
//!
//! # Usage
//! Plugins export information for ArcDPS via the [`export!`] macro.
//! To see which fields are supported by it, have a look at [`SupportedFields`].
//!
//! ```no_run
//! # mod test {
//! use std::error::Error;
//! use arcdps::{Agent, CombatEvent, StateChange};
//!
//! arcdps::export! {
//!     name: "Example Plugin",
//!     sig: 0x12345678, // change this to a random number
//!     init,
//!     combat: custom_combat_name,
//! }
//!
//! fn init() -> Result<(), Box<dyn Error>> {
//!     // may return an error to indicate load failure
//!     Ok(())
//! }
//!
//! fn custom_combat_name(
//!     event: Option<CombatEvent>,
//!     src: Option<Agent>,
//!     dst: Option<Agent>,
//!     skill_name: Option<&str>,
//!     id: u64,
//!     revision: u64,
//! ) {
//!     if let Some(event) = event {
//!         if let StateChange::EnterCombat = event.is_statechange {
//!             // source agent has entered combat
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! # Unofficial Extras
//! [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) support is hidden behind the `extras` feature flag.
//!
//! ```no_run
//! # mod test {
//! use arcdps::extras::{UserInfoIter, UserRole};
//!
//! arcdps::export! {
//!     name: "Example Plugin",
//!     sig: 123,
//!     extras_squad_update,
//! }
//!
//! fn extras_squad_update(users: UserInfoIter) {
//!     for user in users {
//!         if let UserRole::SquadLeader | UserRole::Lieutenant = user.role {
//!             // user can place markers
//!         }
//!     }
//! }
//! # }
//! ```

#![allow(clippy::missing_safety_doc)]

pub mod api;
pub mod callbacks;
pub mod exports;

#[cfg(feature = "extras")]
pub mod extras;

#[cfg(feature = "log")]
pub mod log;

mod globals;
mod panic;
mod util;

pub use api::{
    Activation, Affinity, Agent, AgentOwned, Attribute, BuffCategory, BuffCycle, BuffRemove,
    CombatEvent, CustomSkill, Language, Profession, Specialization, StateChange, Strike,
};
pub use arcdps_codegen::export;
pub use arcdps_imgui as imgui;
pub use globals::{d3d11_device, d3d_version, dxgi_swap_chain};
pub use util::strip_account_prefix;

use callbacks::*;

#[cfg(feature = "extras")]
use extras::callbacks::*;

/// Reference on what fields are currently supported by the [`export!`] macro.
///
/// This struct is not used anywhere.
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

    /// Callback for plugin unload.
    // TODO: higher level abstraction?
    pub update_url: Option<UpdateUrlFunc>,

    /// Raw WndProc callback.
    pub raw_wnd_nofilter: Option<RawWndProcCallback>,

    /// Raw ImGui callback.
    pub raw_imgui: Option<RawImguiCallback>,

    /// Raw options callback.
    pub raw_options_end: Option<RawOptionsCallback>,

    /// Raw combat callback.
    pub raw_combat: Option<RawCombatCallback>,

    /// Raw filtered WndProc callback.
    pub raw_wnd_filter: Option<RawWndProcCallback>,

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
    /// `src.team` the team and `dst.team` the subgroup.
    ///
    /// No `event` and `src.elite != 0` indicates a target change.
    /// `src.id` will contain the new target.
    pub combat: Option<CombatCallback>,

    /// Callback for standalone UI creation.
    ///
    /// Provides an [`imgui::Ui`] for drawing.
    /// The second parameter is `true` whenever the player is **not** in character select, loading screens or forced cameras.
    pub imgui: Option<ImguiCallback>,

    /// Callback for plugin settings UI creation.
    ///
    /// Provides an [`imgui::Ui`] for drawing.
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
    pub raw_extras_squad_update: Option<RawExtrasSquadUpdateCallback>,

    /// Raw extras language changed callback.
    ///
    /// *Requires the `"extras"` feature.*
    #[cfg(feature = "extras")]
    pub raw_extras_language_changed: Option<RawExtrasLanguageChangedCallback>,

    /// Raw extras keybind changed callback.
    ///
    /// *Requires the `"extras"` feature.*
    #[cfg(feature = "extras")]
    pub raw_extras_keybind_changed: Option<RawExtrasKeybindChangedCallback>,

    /// Raw extras chat message callback.
    ///
    /// *Requires the `"extras"` feature.*
    #[cfg(feature = "extras")]
    pub raw_extras_chat_message: Option<RawExtrasChatMessageCallback>,

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
    /// If a user was removed from the squad, their `role` will be set to [`UserRole::None`](crate::extras::UserRole::None).
    ///
    /// *Requires the `"extras"` feature.*
    #[cfg(feature = "extras")]
    pub extras_squad_update: Option<ExtrasSquadUpdateCallback>,

    /// Language changed callback for [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases).
    ///
    /// Called whenever the language is changed, either by changing it in the UI or by pressing the translation key (Right Ctrl by default).
    ///
    /// Will be called directly after initialization, with the current language, to get the startup language.
    ///
    /// *Requires the `"extras"` feature.*
    #[cfg(feature = "extras")]
    pub extras_language_changed: Option<ExtrasLanguageChangedCallback>,

    /// Keybind changed callback for [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases).
    ///
    /// Called whenever a keybind is changed, either by changing it in the ingame UI or with the presets feature of Unofficial Extras.
    /// It is called for every keybind separately.
    ///
    /// After initialization this is called for every current keybind that exists.
    /// If you want to get a single keybind, at any time you want, call the exported function.
    ///
    /// *Requires the `"extras"` feature.*
    #[cfg(feature = "extras")]
    pub extras_keybind_changed: Option<ExtrasKeybindChangedCallback>,

    /// Chat message callback for [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases).
    ///
    /// Called whenever a chat message is sent in your party/squad
    ///
    /// *Requires the `"extras"` feature.*
    #[cfg(feature = "extras")]
    pub extras_chat_message: Option<ExtrasChatMessageCallback>,
}

/// Exports for usage in macros.
#[doc(hidden)]
pub mod __macro {
    pub mod prelude {
        pub use crate::{
            callbacks::*,
            globals::{FreeFn, MallocFn},
        };
        pub use std::os::raw::{c_char, c_void};
        pub use windows::Win32::{
            Foundation::{HINSTANCE, HWND, LPARAM, WPARAM},
            UI::WindowsAndMessaging::{WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP},
        };

        #[cfg(feature = "extras")]
        pub use crate::extras::callbacks::*;
    }

    pub use crate::util::{str_from_cstr, str_to_wide, strip_account_prefix};

    use crate::{
        exports::{has_e3_log_file, has_e8_log_window},
        globals::{init_dxgi, init_imgui, ARC_GLOBALS, IG_UI},
        imgui,
        panic::init_panic_hook,
    };
    use prelude::*;

    #[cfg(feature = "log")]
    use crate::log::ArcDpsLogger;

    /// Internally used function to initialize with information received from Arc.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn init(
        arc_version: *const c_char,
        arc_handle: HINSTANCE,
        imgui_ctx: *mut imgui::sys::ImGuiContext,
        malloc: Option<MallocFn>,
        free: Option<FreeFn>,
        id3d: *mut c_void,
        d3d_version: u32,
        name: &'static str,
    ) {
        // arc exports have to be retrieved before panic hook & logging
        ARC_GLOBALS.init(arc_handle, str_from_cstr(arc_version));

        // only set panic hook if log file export was found
        if has_e3_log_file() {
            init_panic_hook(name);

            // only set logger if log file & window exports were found
            #[cfg(feature = "log")]
            if has_e8_log_window() {
                let result = log::set_boxed_logger(Box::new(ArcDpsLogger::new(name)));
                if result.is_ok() {
                    log::set_max_level(log::LevelFilter::Trace);
                }
            }
        }

        // initialize imgui & dxgi
        init_imgui(imgui_ctx, malloc, free);
        init_dxgi(id3d, d3d_version, name);
    }

    /// Internally used function to retrieve the [`imgui::Ui`].
    #[inline]
    pub unsafe fn ui() -> &'static imgui::Ui<'static> {
        IG_UI.as_ref().expect("imgui ui not initialized")
    }
}
