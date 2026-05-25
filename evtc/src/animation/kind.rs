use crate::CustomSkill;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, IntoStaticStr, VariantNames};

/// Animation kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, VariantNames)
)]
pub enum AnimationKind {
    /// Skill activation or unknown.
    Skill,

    /// Dodge.
    Dodge,

    /// Weapon draw.
    WeaponDraw,

    /// Weapon stow.
    WeaponStow,

    /// Channeled interaction with gadget.
    GadgetInteract,

    /// Emote.
    ///
    /// `reference_id` contains the (volatile) emote id.
    /// Map to stable GUID via [`StateChange::IdToGUID`] event.
    Emote,

    /// Bundle item pickup.
    ///
    /// `reference_id` contains the (stable) item id.
    BundlePickup,
}

impl AnimationKind {
    /// Determines the animation kind from a skill id.
    #[inline]
    pub const fn new(skill_id: u32) -> Self {
        match skill_id {
            CustomSkill::DODGE => Self::Dodge,
            CustomSkill::WEAPON_DRAW => Self::WeaponDraw,
            CustomSkill::WEAPON_STOW => Self::WeaponStow,
            CustomSkill::GADGET_INTERACT => Self::GadgetInteract,
            CustomSkill::EMOTE => Self::Emote,
            CustomSkill::PICKUP => Self::BundlePickup,
            _ => Self::Skill,
        }
    }
}
