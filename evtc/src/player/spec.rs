use super::Profession;
use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Player specialization.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(u32)]
pub enum Specialization {
    /// Unknown or invalid.
    #[default]
    Unknown = 0,

    // mesmer
    Dueling = 1,
    Domination = 10,
    Inspiration = 23,
    Illusions = 24,
    Chronomancer = 40,
    Chaos = 45,
    Mirage = 59,
    Virtuoso = 66,

    // necromancer
    DeathMagic = 2,
    BloodMagic = 19,
    Reaper = 34,
    Curses = 39,
    SoulReaping = 50,
    Spite = 53,
    Scourge = 60,
    Harbinger = 64,

    // revenant
    Invocation = 3,
    Retribution = 9,
    Corruption = 14,
    Devastation = 15,
    Salvation = 12,
    Herald = 52,
    Renegade = 63,
    Vindicator = 69,

    // warrior
    Strength = 4,
    Tactics = 11,
    Berserker = 18,
    Defense = 22,
    Arms = 36,
    Discipline = 51,
    Spellbreaker = 61,
    Bladesworn = 68,

    // ranger
    Druid = 5,
    Marksmanship = 8,
    NatureMagic = 25,
    Skirmishing = 30,
    Beastmastery = 32,
    WildernessSurvival = 33,
    Soulbeast = 55,
    Untamed = 72,

    // engineer
    Explosives = 6,
    Tools = 21,
    Alchemy = 29,
    Firearms = 38,
    Scrapper = 43,
    Inventions = 47,
    Holosmith = 57,
    Mechanist = 70,

    // thief
    Daredevil = 7,
    ShadowArts = 20,
    DeadlyArts = 28,
    CriticalStrikes = 35,
    Trickery = 44,
    Acrobatics = 54,
    Deadeye = 58,
    Specter = 71,

    // guardian
    Valor = 13,
    Radiance = 16,
    Dragonhunter = 27,
    Zeal = 42,
    Virtues = 46,
    Honor = 49,
    Firebrand = 62,
    Willbender = 65,

    // elementalist
    Water = 17,
    Earth = 26,
    Fire = 31,
    Arcane = 37,
    Air = 41,
    Tempest = 48,
    Weaver = 56,
    Catalyst = 67,
}

impl Specialization {
    /// Returns the [`Profession`] corresponding to the specialization.
    #[inline]
    pub fn profession(&self) -> Profession {
        match self {
            Self::Unknown => Profession::Unknown,

            Self::Dueling
            | Self::Domination
            | Self::Inspiration
            | Self::Illusions
            | Self::Chronomancer
            | Self::Chaos
            | Self::Mirage
            | Self::Virtuoso => Profession::Mesmer,

            Self::DeathMagic
            | Self::BloodMagic
            | Self::Reaper
            | Self::Curses
            | Self::SoulReaping
            | Self::Spite
            | Self::Scourge
            | Self::Harbinger => Profession::Necromancer,

            Self::Invocation
            | Self::Retribution
            | Self::Corruption
            | Self::Devastation
            | Self::Salvation
            | Self::Herald
            | Self::Renegade
            | Self::Vindicator => Profession::Revenant,

            Self::Strength
            | Self::Tactics
            | Self::Berserker
            | Self::Defense
            | Self::Arms
            | Self::Discipline
            | Self::Spellbreaker
            | Self::Bladesworn => Profession::Warrior,

            Self::Druid
            | Self::Marksmanship
            | Self::NatureMagic
            | Self::Skirmishing
            | Self::Beastmastery
            | Self::WildernessSurvival
            | Self::Soulbeast
            | Self::Untamed => Profession::Ranger,

            Self::Explosives
            | Self::Tools
            | Self::Alchemy
            | Self::Firearms
            | Self::Scrapper
            | Self::Inventions
            | Self::Holosmith
            | Self::Mechanist => Profession::Engineer,

            Self::Daredevil
            | Self::ShadowArts
            | Self::DeadlyArts
            | Self::CriticalStrikes
            | Self::Trickery
            | Self::Acrobatics
            | Self::Deadeye
            | Self::Specter => Profession::Thief,

            Self::Valor
            | Self::Radiance
            | Self::Dragonhunter
            | Self::Zeal
            | Self::Virtues
            | Self::Honor
            | Self::Firebrand
            | Self::Willbender => Profession::Guardian,

            Self::Water
            | Self::Earth
            | Self::Fire
            | Self::Arcane
            | Self::Air
            | Self::Tempest
            | Self::Weaver
            | Self::Catalyst => Profession::Elementalist,
        }
    }
}
