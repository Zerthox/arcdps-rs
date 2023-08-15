//! Unofficial extras callback types.

use super::{
    keybinds::{KeybindChange, RawKeybindChange},
    message::{ChatMessageInfo, RawChatMessageInfo},
    user::{RawUserInfo, UserInfoIter},
    ExtrasAddonInfo, ExtrasSubscriberInfo, RawExtrasAddonInfo,
};
use crate::{evtc::Language, util::abi};

pub type ExtrasInitFunc = fn(extras_info: ExtrasAddonInfo, account_name: Option<&str>);

pub type ExtrasSquadUpdateCallback = fn(updated_users: UserInfoIter);

pub type ExtrasLanguageChangedCallback = fn(new_language: Language);

pub type ExtrasKeybindChangedCallback = fn(changed: KeybindChange);

pub type ExtrasChatMessageCallback = fn(chat_message_info: &ChatMessageInfo);

abi! {
    pub type RawExtrasSubscriberInit = unsafe extern fn(
        extras_info: *const RawExtrasAddonInfo,
        subscriber_info: *mut ExtrasSubscriberInfo,
    );

    pub type RawExtrasSquadUpdateCallback =
        unsafe extern fn(updated_users: *const RawUserInfo, updated_users_count: u64);

    pub type RawExtrasLanguageChangedCallback = unsafe extern fn(new_language: Language);

    pub type RawExtrasKeybindChangedCallback = unsafe extern fn(changed: RawKeybindChange);

    pub type RawExtrasChatMessageCallback =
        unsafe extern fn(chat_message: *const RawChatMessageInfo);
}
