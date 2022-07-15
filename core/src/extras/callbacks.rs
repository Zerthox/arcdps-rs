use super::{
    keybinds::KeybindChange, ExtrasAddonInfo, ExtrasSubscriberInfo, RawExtrasAddonInfo,
    RawUserInfo, UserInfoIter,
};
use crate::api::Language;

pub type RawExtrasSubscriberInit =
    unsafe extern "C" fn(&RawExtrasAddonInfo, &mut ExtrasSubscriberInfo);

pub type ExtrasInitFunc = fn(ExtrasAddonInfo, Option<&'static str>);

pub type RawSquadUpdateCallback = unsafe extern "C" fn(*const RawUserInfo, u64);
pub type ExtrasSquadUpdateCallback = fn(UserInfoIter);

// TODO: support other callbacks

pub type RawLanguageChangedCallback = unsafe extern "C" fn(Language);

pub type RawKeyBindChangedCallback = unsafe extern "C" fn(KeybindChange);
