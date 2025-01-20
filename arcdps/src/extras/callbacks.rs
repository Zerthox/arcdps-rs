//! Unofficial extras callback types.

use super::{
    keybinds::{KeybindChange, RawKeybindChange},
    message::{Message, RawMessage, RawMessageType, RawSquadMessage, SquadMessage},
    user::{UserInfo, UserInfoIter},
    ExtrasAddonInfo, ExtrasSubscriberInfo, RawExtrasAddonInfo,
};
use crate::{evtc::Language, util::abi};

pub type ExtrasInitFunc = fn(extras_info: ExtrasAddonInfo, account_name: Option<&str>);

pub type ExtrasSquadUpdateCallback = fn(updated_users: UserInfoIter);

pub type ExtrasLanguageChangedCallback = fn(language: Language);

pub type ExtrasKeybindChangedCallback = fn(changed: KeybindChange);

pub type ExtrasSquadChatMessageCallback = fn(message: SquadMessage);

pub type ExtrasChatMessageCallback = fn(message: Message);

abi! {
    pub type RawExtrasSubscriberInit = unsafe extern fn(
        extras_info: *const RawExtrasAddonInfo,
        subscriber_info: *mut ExtrasSubscriberInfo,
    );

    pub type RawExtrasSquadUpdateCallback =
        unsafe extern fn(updated_users: *const UserInfo, updated_users_count: u64);

    pub type RawExtrasLanguageChangedCallback = unsafe extern fn(language: Language);

    pub type RawExtrasKeybindChangedCallback = unsafe extern fn(changed: RawKeybindChange);

    pub type RawExtrasSquadChatMessageCallback =
        unsafe extern fn(message: *const RawSquadMessage);

    pub type RawExtrasChatMessageCallback =
        unsafe extern fn(message_type: RawMessageType, message: RawMessage);
}
