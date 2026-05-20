//! [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) support.
//!
//! *Requires the `"extras"` feature.*

mod globals;

pub mod exports;

use crate::{extras::globals::ExtrasGlobals, util::str_from_cstr};
use std::ffi::c_char;
use unofficial_extras::callbacks::{
    RawExtrasChatMessageCallback, RawExtrasKeybindChangedCallback,
    RawExtrasLanguageChangedCallback, RawExtrasSquadChatMessageCallback,
    RawExtrasSquadUpdateCallback,
};

pub use unofficial_extras::*;

/// Subscribes to unofficial extras callbacks after checking for compatibility.
///
/// Unsupported callbacks will be skipped.
///
/// # Safety
/// Info needs to point to a valid to interpret as subscriber infos with minimum version of [`ExtrasVersion::MIN_SUB_INFO`].
/// It is passed as pointer to prevent UB by creating a reference to the wrong type.
///
/// Name needs to be null-terminated.
#[allow(clippy::too_many_arguments)]
pub unsafe fn subscribe(
    sub: *mut ExtrasSubscriberInfo,
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
        unsafe {
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
