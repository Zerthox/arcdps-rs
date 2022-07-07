use crate::util::str_from_cstr;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::os::raw::c_char;
use std::{iter::Map, slice};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u8)]
pub enum UserRole {
    SquadLeader = 0,
    Lieutenant = 1,
    Member = 2,
    Invited = 3,
    Applied = 4,
    None = 5,

    /// Internal only.
    Invalid = 6,
}

/// Information about a player related to the squad.
#[derive(Debug, Clone)]
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

impl From<&RawUserInfo> for UserInfo<'_> {
    fn from(raw: &RawUserInfo) -> Self {
        Self {
            account_name: str_from_cstr(raw.account_name),
            join_time: raw.join_time,
            role: raw.role,
            subgroup: raw.subgroup,
            ready_status: raw.ready_status,
        }
    }
}

/// Information about a user with owned [`String`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl From<&UserInfo<'_>> for UserInfoOwned {
    fn from(user: &UserInfo<'_>) -> Self {
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
#[repr(C)]
pub struct RawUserInfo {
    pub account_name: *const c_char,
    pub join_time: u64,
    pub role: UserRole,
    pub subgroup: u8,
    pub ready_status: bool,
    pub _unused1: u8,
    pub _unused2: u32,
}

pub type UserInfoIter<'a> = Map<slice::Iter<'a, RawUserInfo>, UserConvert>;

pub type UserConvert = for<'r> fn(&'r RawUserInfo) -> UserInfo<'r>;

/// Helper to convert a [`RawUserInfo`] pointer and a length to an iterator over [`UserInfo`].
pub unsafe fn to_user_info_iter<'a>(ptr: *const RawUserInfo, len: u64) -> UserInfoIter<'a> {
    std::slice::from_raw_parts(ptr, len as usize)
        .iter()
        .map(|raw| raw.into())
}
