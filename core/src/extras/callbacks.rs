use super::{
    keybinds::{KeybindChange, RawKeybindChange},
    message::ChatMessageInfo,
    ExtrasAddonInfo, ExtrasSubscriberInfo, RawExtrasAddonInfo, RawUserInfo, UserInfoIter,
};
use crate::api::Language;

pub type RawExtrasSubscriberInit = unsafe extern "C" fn(
    extras_info: &RawExtrasAddonInfo,
    subscriber_info: &mut ExtrasSubscriberInfo,
);

pub type ExtrasInitFunc = fn(extras_info: ExtrasAddonInfo, account_name: Option<&'static str>);

pub type RawExtrasSquadUpdateCallback =
    unsafe extern "C" fn(updated_users: *const RawUserInfo, updated_users_count: u64);
pub type ExtrasSquadUpdateCallback = fn(updated_users: UserInfoIter);

pub type RawExtrasLanguageChangedCallback = unsafe extern "C" fn(new_language: Language);
pub type ExtrasLanguageChangedCallback = fn(new_language: Language);

// TODO: support other callbacks
pub type RawExtrasKeyBindChangedCallback = unsafe extern "C" fn(changed: RawKeybindChange);
pub type ExtrasKeyBindChangedCallback = fn(changed: KeybindChange);

pub type RawExtrasChatMessageCallback = unsafe extern "C" fn(chat_message: *const ChatMessageInfo);
