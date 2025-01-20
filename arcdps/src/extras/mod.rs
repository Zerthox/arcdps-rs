//! [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) support.
//!
//! *Requires the `"extras"` feature.*

pub mod callbacks;
pub mod exports;
pub mod keybinds;
pub mod message;
pub mod user;

mod globals;
mod version;

use crate::{util::str_from_cstr, RawExtrasSquadChatMessageCallback};
use callbacks::{
    RawExtrasChatMessageCallback, RawExtrasKeybindChangedCallback,
    RawExtrasLanguageChangedCallback, RawExtrasSquadUpdateCallback,
};
pub use keybinds::{Control, Key, KeyCode, Keybind, KeybindChange, MouseCode};
pub use message::{
    ChannelType, Message, NpcMessage, NpcMessageOwned, SquadMessage, SquadMessageOwned,
};
pub use user::{UserInfo, UserInfoIter, UserInfoOwned, UserRole};

use globals::ExtrasGlobals;
use std::os::raw::c_char;
use windows::Win32::Foundation::HMODULE;

pub use version::ExtrasVersion;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Information about the [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) addon.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExtrasAddonInfo {
    /// Version of the API.
    ///
    /// Gets incremented whenever a function signature or behavior changes in a breaking way.
    pub api_version: u32,

    /// Highest known version of the [`ExtrasSubscriberInfo`] struct.
    ///
    /// Also determines the size of the subscriber info buffer in the init call.
    /// The buffer is only guaranteed to have enough space for known [`ExtrasSubscriberInfo`] versions.
    pub max_info_version: u32,

    /// String version of the Unofficial Extras addon.
    ///
    /// Gets changed on every release.
    pub string_version: Option<&'static str>,
}

impl ExtrasAddonInfo {
    /// Returns the corresponding [`ExtrasVersion`].
    #[inline]
    pub fn version(&self) -> ExtrasVersion {
        ExtrasVersion::new(self.api_version, self.max_info_version)
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
    pub api_version: u32,

    /// Highest known version of the [`ExtrasSubscriberInfo`] struct.
    ///
    /// Also determines the size of the subscriber info buffer in the init call.
    /// The buffer is only guaranteed to have enough space for known [`ExtrasSubscriberInfo`] versions.
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
    pub extras_handle: HMODULE,
}

impl RawExtrasAddonInfo {
    /// Returns the corresponding [`ExtrasVersion`].
    #[inline]
    pub fn version(&self) -> ExtrasVersion {
        ExtrasVersion::new(self.api_version, self.max_info_version)
    }
}

/// Subscriber header shared across different versions.
#[derive(Debug)]
#[repr(C)]
pub struct ExtrasSubscriberInfoHeader {
    /// The version of the following info struct.
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
    pub squad_chat_message_callback: Option<RawExtrasSquadChatMessageCallback>,

    /// Called on different chat messages.
    pub chat_message_callback: Option<RawExtrasChatMessageCallback>,
}

impl ExtrasSubscriberInfo {
    /// Subscribes to unofficial extras callbacks after checking for compatibility.
    ///
    /// Unsupported callbacks will be skipped.
    ///
    /// # Safety
    /// Info needs to point to a valid to interpret as subscriber infos with minimum version of [`MIN_SUB_INFO_VERSION`].
    /// It is passed as pointer to prevent UB by creating a reference to the wrong type.
    ///
    /// Name needs to be null-terminated.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn subscribe(
        sub: *mut Self,
        extras_addon: &RawExtrasAddonInfo,
        name: &'static str,
        squad_update: Option<RawExtrasSquadUpdateCallback>,
        language_changed: Option<RawExtrasLanguageChangedCallback>,
        keybind_changed: Option<RawExtrasKeybindChangedCallback>,
        squad_chat_message: Option<RawExtrasSquadChatMessageCallback>,
        chat_message: Option<RawExtrasChatMessageCallback>,
    ) {
        let version = extras_addon.version();
        if let Some(sub_info_version) = version.get_version_to_use() {
            // initialize globals
            ExtrasGlobals::init(
                extras_addon.extras_handle,
                str_from_cstr(extras_addon.string_version),
            );

            (*sub).header.info_version = sub_info_version;
            (*sub).subscriber_name = name.as_ptr() as *const c_char;
            (*sub).squad_update_callback = squad_update;
            (*sub).language_changed_callback = language_changed;
            (*sub).keybind_changed_callback = keybind_changed;

            // only attempt to write additional callbacks if supported
            if version.supports_squad_chat_message() {
                (*sub).squad_chat_message_callback = squad_chat_message;
            }
            if version.supports_chat_message2() {
                (*sub).chat_message_callback = chat_message;
            }
        }
    }
}
