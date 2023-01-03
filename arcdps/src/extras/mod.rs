//! [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) support.
//!
//! *Requires the `"extras"` feature.*

pub mod callbacks;
pub mod exports;
pub mod keybinds;
pub mod message;
pub mod user;

mod globals;

pub use keybinds::{Control, Key, KeyCode, Keybind, KeybindChange, MouseCode};
pub use user::{UserInfo, UserInfoIter, UserInfoOwned, UserRole};

use crate::util::str_from_cstr;
use callbacks::{
    RawExtrasChatMessageCallback, RawExtrasKeybindChangedCallback,
    RawExtrasLanguageChangedCallback, RawExtrasSquadUpdateCallback,
};
use globals::EXTRAS_GLOBALS;
use std::{ops::RangeInclusive, os::raw::c_char};
use windows::Win32::Foundation::HINSTANCE;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Supported Unofficial Extras API version.
const API_VERSION: u32 = 2;

/// Supported [`ExtrasSubscriberInfo`] version range.
const SUB_INFO_RANGE: RangeInclusive<u32> = 1..=2;

/// Helper to check compatibility.
#[inline]
fn check_compat(api_version: u32, sub_info_version: u32) -> bool {
    api_version == API_VERSION && SUB_INFO_RANGE.contains(&sub_info_version)
}

/// Information about the [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) addon.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExtrasAddonInfo {
    /// Version of the API.
    ///
    /// Gets incremented whenever a function signature or behavior changes in a breaking way.
    ///
    /// Current version is `2`.
    pub api_version: u32,

    /// Highest known version of the [`ExtrasSubscriberInfo`] struct.
    ///
    /// Also determines the size of the subscriber info buffer in the init call.
    /// The buffer is only guaranteed to have enough space for known [`ExtrasSubscriberInfo`] versions.
    ///
    /// Current version is `2`.
    pub max_info_version: u32,

    /// String version of the Unofficial Extras addon.
    ///
    /// Gets changed on every release.
    pub string_version: Option<&'static str>,
}

impl ExtrasAddonInfo {
    /// Checks compatibility with the Unofficial Extras addon.
    pub fn is_compatible(&self) -> bool {
        check_compat(self.api_version, self.max_info_version)
    }

    /// Whether the Unofficial Extras addon supports the chat message callback.
    pub fn supports_chat_message_callback(&self) -> bool {
        self.max_info_version >= 2
    }
}

impl From<RawExtrasAddonInfo> for ExtrasAddonInfo {
    fn from(raw: RawExtrasAddonInfo) -> Self {
        Self {
            api_version: raw.api_version,
            max_info_version: raw.max_info_version,
            string_version: unsafe { str_from_cstr(raw.string_version) },
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawExtrasAddonInfo {
    /// Version of the API.
    ///
    /// Gets incremented whenever a function signature or behavior changes in a breaking way.
    ///
    /// Current version is `2`.
    pub api_version: u32,

    /// Highest known version of the [`ExtrasSubscriberInfo`] struct.
    ///
    /// Also determines the size of the subscriber info buffer in the init call.
    /// The buffer is only guaranteed to have enough space for known [`ExtrasSubscriberInfo`] versions.
    ///
    /// Current version is `2`.
    pub max_info_version: u32,

    /// String version of the Unofficial Extras addon.
    ///
    /// Gets changed on every release.
    /// The string is valid for the entire lifetime of the Unofficial Extras DLL.
    pub string_version: *const c_char,

    /// Account name of the logged-in player, including leading `':'`.
    ///
    /// The string is only valid for the duration of the init call.
    pub self_account_name: *const c_char,

    /// The handle to the Unofficial Extras module.
    ///
    /// Use this to call the exports of the DLL.
    pub extras_handle: HINSTANCE,
}

impl RawExtrasAddonInfo {
    /// Checks compatibility with the Unofficial Extras addon.
    pub fn is_compatible(&self) -> bool {
        check_compat(self.api_version, self.max_info_version)
    }

    /// Whether the Unofficial Extras addon supports the message callback.
    pub fn supports_chat_message_callback(&self) -> bool {
        self.max_info_version >= 2
    }
}

/// Subscriber header shared across different versions.
#[derive(Debug)]
#[repr(C)]
pub struct ExtrasSubscriberInfoHeader {
    /// The version of the following info struct
    /// This has to be set to the version you want to use.
    pub info_version: u32,

    /// Unused padding.
    pub unused1: u32,
}

/// Information about a subscriber to updates from Unofficial Extras.
#[derive(Debug)]
#[repr(C)]
pub struct ExtrasSubscriberInfo {
    /// Header shared across different versions.
    pub header: ExtrasSubscriberInfoHeader,

    /// Name of the addon subscribing to the changes.
    ///
    /// Must be valid for the lifetime of the subscribing addon.
    /// Set to `nullptr` if initialization fails.
    pub subscriber_name: *const c_char,

    /// Called whenever anything in the squad changes.
    ///
    /// Only the users that changed are sent.
    /// If a user is removed from the squad, it will be sent with `role` set to [`UserRole::None`]
    pub squad_update_callback: Option<RawExtrasSquadUpdateCallback>,

    /// Called whenever the language is changed.
    ///
    /// Either by Changing it in the UI or by pressing the Right Ctrl (default) key.
    /// Will also be called directly after initialization, with the current language, to get the startup language.
    pub language_changed_callback: Option<RawExtrasLanguageChangedCallback>,

    /// Called whenever a keybind is changed.
    ///
    /// By changing it in the ingame UI, by pressing the translation shortcut or with the Presets feature of this plugin.
    /// It is called for every keybind separately.
    ///
    /// After initialization this is called for every current keybind that exists.
    /// If you want to get a single keybind, at any time you want, call the exported function.
    pub keybind_changed_callback: Option<RawExtrasKeybindChangedCallback>,

    /// Called whenever a chat message is sent in your party/squad.
    pub chat_message_callback: Option<RawExtrasChatMessageCallback>,
}

impl ExtrasSubscriberInfo {
    /// Subscribes to unofficial extras callbacks after checking for compatibility.
    ///
    /// Unsupported callbacks will be skipped.
    ///
    /// Name needs to be null-terminated.
    pub unsafe fn subscribe(
        &mut self,
        extras_addon: &RawExtrasAddonInfo,
        name: &'static str,
        squad_update: Option<RawExtrasSquadUpdateCallback>,
        language_changed: Option<RawExtrasLanguageChangedCallback>,
        keybind_changed: Option<RawExtrasKeybindChangedCallback>,
        chat_message: Option<RawExtrasChatMessageCallback>,
    ) {
        if extras_addon.is_compatible() {
            // initialize globals
            EXTRAS_GLOBALS.init(
                extras_addon.extras_handle,
                str_from_cstr(extras_addon.string_version),
            );

            // we simply use the max version
            self.header.info_version = extras_addon.max_info_version;

            self.subscriber_name = name.as_ptr() as *const c_char;
            self.squad_update_callback = squad_update;
            self.language_changed_callback = language_changed;
            self.keybind_changed_callback = keybind_changed;

            // only attempt to write message callback if supported
            if extras_addon.supports_chat_message_callback() {
                self.chat_message_callback = chat_message;
            }
        }
    }
}
