use crate::Language;

use super::{
    keybinds::KeybindChange, RawExtrasAddonInfo, RawExtrasSubscriberInfo, RawUserInfo, UserInfo,
};
use std::{iter::Map, slice};

/// This function must be exported by subscriber addons as `arcdps_unofficial_extras_subscriber_init`.
/// It's called once at startup.
/// Can be called before or after ArcDPS calls mod_init.
/// Set `subscriber_name` to `nullptr` if initialization fails.
pub type RawExtrasSubscriberInit =
    unsafe extern "C" fn(&RawExtrasAddonInfo, &mut RawExtrasSubscriberInfo);

/// Called at startup of unofficial extras.
/// Can be called before or after ArcDPS init func.
/// Provides the account name and the version of the unofficial extras addon as parameters.
pub type ExtrasInitFunc = fn(Option<&str>, Option<&'static str>);

pub type RawSquadUpdateCallback = unsafe extern "C" fn(*const RawUserInfo, u64);

pub type ExtrasSquadUpdateCallback = fn(UserInfoIter);

pub type UserInfoIter<'a> = Map<slice::Iter<'a, RawUserInfo>, UserConvert>;

pub type UserConvert = for<'r> fn(&'r RawUserInfo) -> UserInfo;

pub type RawLanguageChangedCallback = unsafe extern "C" fn(Language);

pub type RawKeyBindChangedCallback = unsafe extern "C" fn(KeybindChange);
