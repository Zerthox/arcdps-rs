use super::{
    keybinds::KeybindChange, ExtrasAddonInfo, ExtrasSubscriberInfo, RawExtrasAddonInfo,
    RawUserInfo, UserInfoIter,
};
use crate::Language;

/// This function must be exported by subscriber addons as `arcdps_unofficial_extras_subscriber_init`.
/// It's called once at startup.
/// Can be called before or after ArcDPS calls mod_init.
/// Set `subscriber_name` to `nullptr` if initialization fails.
pub type RawExtrasSubscriberInit =
    unsafe extern "C" fn(&RawExtrasAddonInfo, &mut ExtrasSubscriberInfo);

/// Callback for subscriber initialization.
///
/// Can be called before or after ArcDPS init function.
/// Receives information about the Unofficial Extras addon and the current player account name as parameters.
pub type ExtrasInitFunc = fn(ExtrasAddonInfo, Option<&'static str>);

pub type RawSquadUpdateCallback = unsafe extern "C" fn(*const RawUserInfo, u64);

/// Callback for squad information.
///
/// Called whenever anything in the squad changes.
/// Only the users that changed are sent.
/// If a user is removed, their `role` will be set to [`UserRole::None`](super::UserRole::None).
pub type ExtrasSquadUpdateCallback = fn(UserInfoIter);

// TODO: support other callbacks

pub type RawLanguageChangedCallback = unsafe extern "C" fn(Language);

pub type RawKeyBindChangedCallback = unsafe extern "C" fn(KeybindChange);
