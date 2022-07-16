use super::{
    keybinds::KeybindChange, message::ChatMessageInfo, ExtrasAddonInfo, ExtrasSubscriberInfo,
    RawExtrasAddonInfo, RawUserInfo, UserInfoIter,
};
use crate::api::Language;

pub type RawExtrasSubscriberInit =
    unsafe extern "C" fn(&RawExtrasAddonInfo, &mut ExtrasSubscriberInfo);

pub type ExtrasInitFunc = fn(ExtrasAddonInfo, Option<&'static str>);

pub type RawExtrasSquadUpdateCallback = unsafe extern "C" fn(*const RawUserInfo, u64);
pub type ExtrasSquadUpdateCallback = fn(UserInfoIter);

pub type RawExtrasLanguageChangedCallback = unsafe extern "C" fn(Language);
pub type ExtrasLanguageChangedCallback = fn(Language);

// TODO: support other callbacks
pub type RawKeyBindChangedCallback = unsafe extern "C" fn(KeybindChange);

pub type RawChatMessageCallback = unsafe extern "C" fn(*const ChatMessageInfo);
