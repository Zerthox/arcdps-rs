//! Unofficial extras support.

// TODO: hide this behind feature flag?

pub mod callbacks;
pub mod keybinds;
mod user;

pub use user::*;

use crate::util::str_from_cstr;
use callbacks::{RawKeyBindChangedCallback, RawLanguageChangedCallback, RawSquadUpdateCallback};
use std::os::raw::c_char;
use windows::Win32::Foundation::HINSTANCE;

/// Supported extras API version.
const API_VERSION: u32 = 2;

/// Supported [`RawExtrasSubscriberInfo`] version.
const SUB_INFO_VERSION: u32 = 1;

/// Helper to check compatibility.
fn check_compat(api_version: u32, sub_info_version: u32) -> bool {
    api_version == API_VERSION && sub_info_version >= SUB_INFO_VERSION
}

#[derive(Debug)]
pub struct ExtrasAddonInfo {
    /// Version of the api, gets incremented whenever a function signature or behavior changes in a breaking way.
    /// Current version is 2.
    pub api_version: u32,

    /// Highest known version of the ExtrasSubscriberInfo struct.
    /// Also determines the size of the pSubscriberInfo buffer in the init call (the buffer is only guaranteed to have enough space for known ExtrasSubscriberInfo versions).
    /// Current version is 1.
    pub max_info_version: u32,

    /// String version of unofficial_extras addon, gets changed on every release.
    /// The string is valid for the lifetime of the unofficial_extras dll.
    pub string_version: Option<&'static str>,
}

impl ExtrasAddonInfo {
    // Checks compatibility with the extras addon.
    pub fn check_compat(&self) -> bool {
        check_compat(self.api_version, self.max_info_version)
    }
}

impl From<&RawExtrasAddonInfo> for ExtrasAddonInfo {
    fn from(raw: &RawExtrasAddonInfo) -> Self {
        Self {
            api_version: raw.api_version,
            max_info_version: raw.max_info_version,
            string_version: str_from_cstr(raw.string_version),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct RawExtrasAddonInfo {
    /// Version of the api, gets incremented whenever a function signature or behavior changes in a breaking way.
    /// Current version is 2.
    pub api_version: u32,

    /// Highest known version of the ExtrasSubscriberInfo struct.
    /// Also determines the size of the pSubscriberInfo buffer in the init call (the buffer is only guaranteed to have enough space for known ExtrasSubscriberInfo versions).
    /// Current version is 1.
    pub max_info_version: u32,

    /// String version of unofficial_extras addon, gets changed on every release.
    /// The string is valid for the lifetime of the unofficial_extras dll.
    pub string_version: *const c_char,

    /// The account name of the logged in player, including leading `:`.
    /// The string is only valid for the duration of the init call.
    pub self_account_name: *const c_char,

    /// The handle to the unofficial_extras module.
    /// Use this to call the exports of the DLL.
    pub extras_handle: HINSTANCE,
}

impl RawExtrasAddonInfo {
    /// Checks compatibility with the extras addon.
    pub fn check_compat(&self) -> bool {
        check_compat(self.api_version, self.max_info_version)
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct RawExtrasSubscriberInfoHeader {
    /// The version of the following info struct
    /// This has to be set to the version you want to use.
    pub info_version: u32,

    /// Unused padding.
    pub _unused1: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct RawExtrasSubscriberInfo {
    /// Header shared across different versions.
    pub header: RawExtrasSubscriberInfoHeader,

    /// Name of the addon subscribing to the changes.
    /// Must be valid for the lifetime of the subscribing addon.
    /// Set to `nullptr` if initialization fails.
    pub subscriber_name: *const c_char,

    /// Called whenever anything in the squad changes.
    /// Only the users that changed are sent.
    /// If a user is removed from the squad, it will be sent with `role` set to [`UserRole::None`]
    pub squad_update_callback: Option<RawSquadUpdateCallback>,

    /// Called whenever the language is changed.
    /// Either by Changing it in the UI or by pressing the Right Ctrl (default) key.
    /// Will also be called directly after initialization, with the current language, to get the startup language.
    pub language_changed_callback: Option<RawLanguageChangedCallback>,

    /// Called whenever a KeyBind is changed.
    /// By changing it in the ingame UI, by pressing the translation shortcut or with the Presets feature of this plugin.
    /// It is called for every keyBind separately.
    ///
    /// After initialization this is called for every current keybind that exists.
    /// If you want to get a single keybind, at any time you want, call the exported function.
    // TODO: expose exported function
    pub key_bind_changed_callback: Option<RawKeyBindChangedCallback>,
}

impl RawExtrasSubscriberInfo {
    /// Writes subscriber information into the struct.
    ///
    /// Name needs to be null-terminated.
    pub fn subscribe(&mut self, name: *const c_char, squad_update: Option<RawSquadUpdateCallback>) {
        self.header.info_version = SUB_INFO_VERSION;
        self.subscriber_name = name;
        self.squad_update_callback = squad_update;
    }
}
