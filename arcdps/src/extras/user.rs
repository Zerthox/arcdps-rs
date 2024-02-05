//! User information provided by Unofficial Extras.

use crate::util::{str_from_cstr, strip_account_prefix};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{os::raw::c_char, slice};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Role of a user in the squad.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
#[repr(u8)]
pub enum UserRole {
    /// User is leader (commander tag).
    SquadLeader = 0,

    /// User is lieutenant.
    Lieutenant = 1,

    /// User is regular member.
    Member = 2,

    /// User is invited.
    Invited = 3,

    /// User has requested to join.
    Applied = 4,

    /// User has left.
    None = 5,

    /// Internal only.
    Invalid = 6,
}

/// Information about a player related to the squad.
///
/// Strings are available for the duration of the call.
/// If you need it for longer than that, consider converting it to [`UserInfoOwned`].
///
/// ```no_run
/// # use arcdps::extras::{UserInfo, UserInfoOwned};
/// # let user: UserInfo = todo!();
/// let owned = user.to_owned();
/// let owned: UserInfoOwned = user.into();
/// ```
#[derive(Debug)]
#[repr(C)]
pub struct UserInfo {
    /// Account name with leading `':'`.
    account_name: *const c_char,

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

    /// Unused space.
    pub _unused1: u8,

    /// Unused space.
    pub _unused2: u32,
}

impl UserInfo {
    /// Returns the user account name without leading `':'`.
    #[inline]
    pub fn account_name(&self) -> Option<&str> {
        unsafe { str_from_cstr(self.account_name).map(strip_account_prefix) }
    }

    /// Returns the raw pointer to the user account name.
    #[inline]
    pub fn account_name_ptr(&self) -> *const c_char {
        self.account_name
    }

    /// Converts the [`UserInfo`] to the owned version [`UserInfoOwned`].
    #[inline]
    pub fn to_owned(self) -> UserInfoOwned {
        self.into()
    }
}

/// [`UserInfo`] with an owned [`String`] name.
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

impl From<UserInfo> for UserInfoOwned {
    #[inline]
    fn from(user: UserInfo) -> Self {
        Self {
            account_name: user.account_name().map(|x| x.to_string()),
            join_time: user.join_time,
            role: user.role,
            subgroup: user.subgroup,
            ready_status: user.ready_status,
        }
    }
}

/// Iterator over changed users.
pub type UserInfoIter<'a> = slice::Iter<'a, UserInfo>;

/// Helper to generate an iterator over [`UserInfo`] structs.
#[inline]
pub unsafe fn to_user_info_iter<'a>(ptr: *const UserInfo, len: u64) -> UserInfoIter<'a> {
    slice::from_raw_parts(ptr, len as usize).iter()
}
