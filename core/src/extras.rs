//! Unofficial extras support.

// TODO: hide this behind feature flag?

use std::{iter::Map, slice::Iter};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum UserRole {
    SquadLeader = 0,
    Lieutenant = 1,
    Member = 2,
    Invited = 3,
    Applied = 4,
    None = 5,

    /// Internal only
    Invalid = 6,
}

#[derive(Debug)]
pub struct UserInfoOwned {
    /// Account name, without leading ':'.
    pub account_name: Option<String>,

    /// Unix timestamp when the user joined the squad.
    ///
    /// `0` if time could not be determined.
    pub join_time: u64,

    /// Role in squad, or [`UserRole::None`] if the user was removed from the squad.
    pub role: UserRole,

    /// Subgroup the user is in.
    ///
    /// `0` when no subgroup could be found, which is either the first subgroup or no subgroup.
    pub subgroup: u8,

    /// Whether this player is ready or not (in a squad ready check).
    ///
    /// ### Remarks
    /// `role` set to [`UserRole::SquadLeader`] and `ready_status == true` implies that a ready check was just started.
    /// Similarly, `role` set to [`UserRole::SquadLeader`] and `ready_status == false` implies that a ready check either finished or was cancelled.
    /// If everyone in the squad had an event sent with `ready_status == true` then that means that the ready check finished successfully.
    /// After which there will be events sent for each user where their `ready_status == false`.
    pub ready_status: bool,
}

impl From<UserInfo<'_>> for UserInfoOwned {
    fn from(user: UserInfo<'_>) -> Self {
        Self {
            account_name: user.account_name.map(|x| x.to_string()),
            join_time: user.join_time,
            role: user.role,
            subgroup: user.subgroup,
            ready_status: user.ready_status,
        }
    }
}

#[derive(Debug)]
pub struct UserInfo<'a> {
    /// Account name, without leading ':'.
    pub account_name: Option<&'a str>,

    /// Unix timestamp when the user joined the squad.
    ///
    /// `0` if time could not be determined.
    pub join_time: u64,

    /// Role in squad, or [`UserRole::None`] if the user was removed from the squad.
    pub role: UserRole,

    /// Subgroup the user is in.
    ///
    /// `0` when no subgroup could be found, which is either the first subgroup or no subgroup.
    pub subgroup: u8,

    /// Whether this player is ready or not (in a squad ready check).
    ///
    /// ### Remarks
    /// `role` set to [`UserRole::SquadLeader`] and `ready_status == true` implies that a ready check was just started.
    /// Similarly, `role` set to [`UserRole::SquadLeader`] and `ready_status == false` implies that a ready check either finished or was cancelled.
    /// If everyone in the squad had an event sent with `ready_status == true` then that means that the ready check finished successfully.
    /// After which there will be events sent for each user where their `ready_status == false`.
    pub ready_status: bool,
}

#[repr(C)]
pub struct RawUserInfo {
    pub account_name: *const u8,
    pub join_time: u64,
    pub role: UserRole,
    pub subgroup: u8,
    pub ready_status: bool,
    pub _unused1: u8,
    pub _unused2: u32,
}

#[repr(C)]
pub struct RawExtrasAddonInfo {
    /// Version of the api, gets incremented whenever a function signature or behavior changes in a breaking way.
    /// Current version is 1.
    pub api_version: u32,

    /// padding
    pub _unused: u32,

    /// String version of unofficial_extras addon, gets changed on every release.
    /// The string is valid for the lifetime of the unofficial_extras dll.
    pub string_version: *const u8,

    /// The account name of the logged in player, including leading `:`.
    /// The string is only valid for the duration of the init call.
    pub self_account_name: *const u8,
}

pub type RawSquadUpdateCallbackSignature = unsafe extern "C" fn(*const RawUserInfo, u64);

pub type ExtrasSquadUpdateCallback = fn(UserInfoIter);

pub type UserInfoIter<'a> = Map<Iter<'a, RawUserInfo>, UserConvert>;

pub type UserConvert = for<'r> fn(&'r RawUserInfo) -> UserInfo;

#[repr(C)]
pub struct RawExtrasSubscriberInfo {
    /// Name of the addon subscribing to the changes.
    /// Must be valid for the lifetime of the subcribing addon.
    /// Set to `nullptr` if initialization fails.
    pub subscriber_name: *const u8,

    /// Called whenever anything in the squad changes.
    /// Only the users that changed are sent.
    /// If a user is removed from the squad, it will be sent with `role` set to [`UserRole::None`]
    pub squad_update_callback: Option<RawSquadUpdateCallbackSignature>,
}

/// This function must be exported by subscriber addons as `arcdps_unofficial_extras_subscriber_init`.
/// It's called once at startup.
/// Can be called before or after ArcDPS calls mod_init.
/// Set `subscriber_name` to `nullptr` if initialization fails.
pub type RawExtrasSubscriberInitSignature =
    unsafe extern "C" fn(&RawExtrasAddonInfo, &mut RawExtrasSubscriberInfo);

/// Called at startup of unofficial extras. Can be called before or after ArcDPS init func.
/// Provides the account name and the version of the unofficial extras addon as parameters.
pub type ExtrasInitFunc = fn(Option<&str>, Option<&'static str>);
