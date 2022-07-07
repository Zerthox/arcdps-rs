use crate::Language;

use super::{
    keybinds::KeybindChange, ExtrasAddonInfo, RawExtrasAddonInfo, RawExtrasSubscriberInfo,
    RawUserInfo, UserInfo,
};
use std::{iter::Map, slice};

/// This function must be exported by subscriber addons as `arcdps_unofficial_extras_subscriber_init`.
/// It's called once at startup.
/// Can be called before or after ArcDPS calls mod_init.
/// Set `subscriber_name` to `nullptr` if initialization fails.
pub type RawExtrasSubscriberInit =
    unsafe extern "C" fn(&RawExtrasAddonInfo, &mut RawExtrasSubscriberInfo);

/// Called at startup of unofficial extras.
///
/// Can be called before or after ArcDPS init func.
/// Receives information about the unofficial extras addon and the current player account name as parameters.
pub type ExtrasInitFunc = fn(ExtrasAddonInfo, Option<&'static str>);

pub type RawSquadUpdateCallback = unsafe extern "C" fn(*const RawUserInfo, u64);

pub type ExtrasSquadUpdateCallback = fn(UserInfoIter);

pub type UserInfoIter<'a> = Map<slice::Iter<'a, RawUserInfo>, UserConvert>;

pub type UserConvert = for<'r> fn(&'r RawUserInfo) -> UserInfo<'r>;

pub type RawLanguageChangedCallback = unsafe extern "C" fn(Language);

pub type RawKeyBindChangedCallback = unsafe extern "C" fn(KeybindChange);
