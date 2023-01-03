//! Keybind information provided by Unofficial Extras.

use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "strum")]
use strum::{Display, EnumCount, EnumIter, EnumVariantNames, IntoStaticStr};

/// Keybind change event.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeybindChange {
    /// Game control which got changed.
    pub control: Control,

    /// Index of the key in settings.
    pub index: i32,

    /// New key.
    pub key: Key,

    /// Whether the Shift modifier is used.
    pub mod_shift: bool,

    /// Whether the Ctrl modifier is used.
    pub mod_ctrl: bool,

    /// Whether the Alt modifier is used.
    pub mod_alt: bool,
}

impl From<RawKeybindChange> for KeybindChange {
    fn from(raw: RawKeybindChange) -> Self {
        let modifier = raw.key.modifier;
        Self {
            control: raw.control,
            index: raw.index,
            key: raw.key.into(),
            mod_shift: modifier & Modifier::Shift as i32 == 1,
            mod_ctrl: modifier & Modifier::Ctrl as i32 == 1,
            mod_alt: modifier & Modifier::Alt as i32 == 1,
        }
    }
}

/// A key used by the game.
///
/// This can be a [`MouseCode`], [`KeyCode`] or an [`Unknown`](Self::Unknown) code.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Key {
    /// An unknown type of key.
    Unknown(i32),

    /// A mouse button.
    Mouse(MouseCode),

    /// A keyboard key.
    Key(KeyCode),
}

impl From<RawKey> for Key {
    fn from(raw: RawKey) -> Self {
        match raw.device_type {
            DeviceType::Unset => Self::Unknown(raw.code),
            DeviceType::Mouse => match raw.code.try_into() {
                Ok(code) => Self::Mouse(code),
                Err(_) => Self::Unknown(raw.code),
            },
            DeviceType::Keyboard => match raw.code.try_into() {
                Ok(code) => Self::Key(code),
                Err(_) => Self::Unknown(raw.code),
            },
        }
    }
}

/// A keybind used by the game.
///
/// This contains the `primary`as well as `secondary` [`Key`] for a [`Control`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Keybind {
    /// Primary [`Key`] for this bind.
    pub primary: Key,

    /// Secondary [`Key`] for this bind.
    pub secondary: Key,
}

impl From<RawKeybind> for Keybind {
    fn from(raw: RawKeybind) -> Self {
        Self {
            primary: raw.primary.into(),
            secondary: raw.secondary.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RawKeybindChange {
    pub control: Control,
    pub index: i32,
    pub key: RawKey,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RawKey {
    pub device_type: DeviceType,
    pub code: i32,
    pub modifier: i32,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RawKeybind {
    pub primary: RawKey,
    pub secondary: RawKey,
}

/// A control (player action) used by the game.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum Control {
    // Movement tab
    Movement_MoveForward = 0,
    Movement_MoveBackward = 1,
    Movement_StrafeLeft = 2,
    Movement_StrafeRight = 3,
    Movement_TurnLeft = 4,
    Movement_TurnRight = 5,
    Movement_Dodge = 6,
    Movement_Autorun = 7,
    Movement_Walk = 8,
    Movement_Jump = 9,
    Movement_SwimUp = 10,
    Movement_SwimDown = 11,
    Movement_AboutFace = 12,

    // Skills tab
    Skills_SwapWeapons = 17,
    Skills_WeaponSkill1 = 18,
    Skills_WeaponSkill2 = 19,
    Skills_WeaponSkill3 = 20,
    Skills_WeaponSkill4 = 21,
    Skills_WeaponSkill5 = 22,
    Skills_HealingSkill = 23,
    Skills_UtilitySkill1 = 24,
    Skills_UtilitySkill2 = 25,
    Skills_UtilitySkill3 = 26,
    Skills_EliteSkill = 27,
    Skills_ProfessionSkill1 = 28,
    Skills_ProfessionSkill2 = 29,
    Skills_ProfessionSkill3 = 30,
    Skills_ProfessionSkill4 = 31,
    Skills_ProfessionSkill5 = 79,
    Skills_ProfessionSkill6 = 201,
    Skills_ProfessionSkill7 = 202,
    Skills_SpecialAction = 82,

    // Targeting tab
    Targeting_AlertTarget = 131,
    Targeting_CallTarget = 32,
    Targeting_TakeTarget = 33,
    Targeting_SetPersonalTarget = 199,
    Targeting_TakePersonalTarget = 200,
    Targeting_NearestEnemy = 34,
    Targeting_NextEnemy = 35,
    Targeting_PreviousEnemy = 36,
    Targeting_NearestAlly = 37,
    Targeting_NextAlly = 38,
    Targeting_PreviousAlly = 39,
    Targeting_LockAutotarget = 40,
    Targeting_SnapGroundTarget = 80,
    Targeting_ToggleSnapGroundTarget = 115,
    Targeting_DisableAutotargeting = 116,
    Targeting_ToggleAutotargeting = 117,
    Targeting_AllyTargetingMode = 197,
    Targeting_ToggleAllyTargetingMode = 198,

    // UI Tab
    UI_BlackLionTradingDialog = 41,
    UI_ContactsDialog = 42,
    UI_GuildDialog = 43,
    UI_HeroDialog = 44,
    UI_InventoryDialog = 45,
    UI_PetDialog = 46,
    UI_LogOut = 47,
    UI_MailDialog = 71,
    UI_OptionsDialog = 48,
    UI_PartyDialog = 49,
    UI_PvPPanel = 73,
    UI_PvPBuild = 75,
    UI_Scoreboard = 50,
    UI_InformationDialog = 51,
    UI_Show_HideChat = 70,
    UI_ChatCommand = 52,
    UI_ChatMessage = 53,
    UI_ChatReply = 54,
    UI_ShowHideUI = 55,
    UI_ShowHideSquadBroadcastChat = 85,
    UI_SquadBroadcastChatCommand = 83,
    UI_SquadBroadcastMessage = 84,

    // Camera Tab
    Camera_FreeCamera = 13,
    Camera_ZoomIn = 14,
    Camera_ZoomOut = 15,
    Camera_LookBehind = 16,
    Camera_ToggleActionCamera = 78,
    Camera_DisableActionCamera = 114,

    // Screenshot Tab
    Screenshot_Normal = 56,
    Screenshot_Stereoscopic = 57,

    // Map Tab
    Map_OpenClose = 59,
    Map_Recenter = 60,
    Map_FloorDown = 61,
    Map_FloorUp = 62,
    Map_ZoomIn = 63,
    Map_ZoomOut = 64,

    // Mounts Tab
    Mounts_MountDismount = 152,
    Mounts_MountAbility1 = 130,
    Mounts_MountAbility2 = 153,
    Mounts_Raptor = 155,
    Mounts_Springer = 156,
    Mounts_Skimmer = 157,
    Mounts_Jackal = 158,
    Mounts_Griffon = 159,
    Mounts_RollerBeetle = 161,
    Mounts_Warclaw = 169,
    Mounts_Skyscale = 170,
    Mounts_Turtle = 203,

    // Spectators Tab
    Spectators_NearestFixedCamera = 102,
    Spectators_NearestPlayer = 103,
    Spectators_RedPlayer1 = 104,
    Spectators_RedPlayer2 = 105,
    Spectators_RedPlayer3 = 106,
    Spectators_RedPlayer4 = 107,
    Spectators_RedPlayer5 = 108,
    Spectators_BluePlayer1 = 109,
    Spectators_BluePlayer2 = 110,
    Spectators_BluePlayer3 = 111,
    Spectators_BluePlayer4 = 112,
    Spectators_BluePlayer5 = 113,
    Spectators_FreeCamera = 120,
    Spectators_FreeCameraBoost = 127,
    Spectators_FreeCameraForward = 121,
    Spectators_FreeCameraBackward = 122,
    Spectators_FreeCameraLeft = 123,
    Spectators_FreeCameraRight = 124,
    Spectators_FreeCameraUp = 125,
    Spectators_FreeCameraDown = 126,

    // Squad Tab
    Squad_Location_Arrow = 86,
    Squad_Location_Circle = 87,
    Squad_Location_Heart = 88,
    Squad_Location_Square = 89,
    Squad_Location_Star = 90,
    Squad_Location_Spiral = 91,
    Squad_Location_Triangle = 92,
    Squad_Location_X = 93,
    Squad_ClearAllLocationMarkers = 119,
    Squad_Object_Arrow = 94,
    Squad_Object_Circle = 95,
    Squad_Object_Heart = 96,
    Squad_Object_Square = 97,
    Squad_Object_Star = 98,
    Squad_Object_Spiral = 99,
    Squad_Object_Triangle = 100,
    Squad_Object_X = 101,
    Squad_ClearAllObjectMarkers = 118,

    // Mastery Skills Tab
    MasterySkills_ActivateMasterySkill = 196,
    MasterySkills_StartFishing = 204,
    MasterySkills_SummonSkiff = 205,
    MasterySkills_SetJadeBotWaypoint = 206,

    // Miscellaneous Tab
    Miscellaneous_AoELoot = 74,
    Miscellaneous_Interact = 65,
    Miscellaneous_ShowEnemyNames = 66,
    Miscellaneous_ShowAllyNames = 67,
    Miscellaneous_StowDrawWeapon = 68,
    Miscellaneous_ToggleLanguage = 69,
    Miscellaneous_RangerPetCombatToggle = 76,
    Miscellaneous_ToggleFullScreen = 160,
    Miscellaneous_EquipUnequipNovelty = 162,
    Miscellaneous_ActivateChair = 163,
    Miscellaneous_ActivateMusicalInstrument = 164,
    Miscellaneous_ActivateHeldItem = 165,
    Miscellaneous_ActivateToy = 166,
    Miscellaneous_ActivateTonic = 167,

    // Templates Tab
    Templates_BuildTemplate1 = 171,
    Templates_BuildTemplate2 = 172,
    Templates_BuildTemplate3 = 173,
    Templates_BuildTemplate4 = 174,
    Templates_BuildTemplate5 = 175,
    Templates_BuildTemplate6 = 176,
    Templates_BuildTemplate7 = 177,
    Templates_BuildTemplate8 = 178,
    Templates_EquipmentTemplate1 = 182,
    Templates_EquipmentTemplate2 = 183,
    Templates_EquipmentTemplate3 = 184,
    Templates_EquipmentTemplate4 = 185,
    Templates_EquipmentTemplate5 = 186,
    Templates_EquipmentTemplate6 = 187,
    Templates_EquipmentTemplate7 = 188,
    Templates_EquipmentTemplate8 = 189,
}

/// Custom key codes.
///
/// Some of them are not usable like [`F13`](Self::F32) to [`F35`](Self::F35) or [`Print`](Self::Print).
///
/// Names are based upon US keyboard layout.
/// Site to translate it to other languages: <http://kbdlayout.info>
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(i32)]
pub enum KeyCode {
    LeftAlt = 0,
    LeftCtrl = 1,
    LeftShift = 2,
    Quote = 3,
    Hash = 4,
    CapsLock = 5,
    Colon = 6,
    Minus = 7,
    Equals = 8,
    Escape = 9,
    OpenBracket = 10,
    NumLock = 11,
    Period = 12,
    CloseBracket = 13,
    Semicolon = 14,
    Slash = 15,
    Print = 16,
    Tilde = 17,
    Backspace = 18,
    Delete = 19,
    Enter = 20,
    Space = 21,
    Tab = 22,
    End = 23,
    Home = 24,
    Insert = 25,
    Next = 26,
    Prior = 27,
    ArrowDown = 28,
    ArrowLeft = 29,
    ArrowRight = 30,
    ArrowUp = 31,
    F1 = 32,
    F2 = 33,
    F3 = 34,
    F4 = 35,
    F5 = 36,
    F6 = 37,
    F7 = 38,
    F8 = 39,
    F9 = 40,
    F10 = 41,
    F11 = 42,
    F12 = 43,
    Number0 = 48,
    Number1 = 49,
    Number2 = 50,
    Number3 = 51,
    Number4 = 52,
    Number5 = 53,
    Number6 = 54,
    Number7 = 55,
    Number8 = 56,
    Number9 = 57,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    PlusNum = 91,
    DecimalNum = 92,
    DivideNum = 93,
    MultiplyNum = 94,
    Number0Num = 95,
    Number1Num = 96,
    Number2Num = 97,
    Number3Num = 98,
    Number4Num = 99,
    Number5Num = 100,
    Number6Num = 101,
    Number7Num = 102,
    Number8Num = 103,
    Number9Num = 104,
    EnterNum = 105,
    MinusNum = 106,
    ImeKey1 = 107,
    ImeKey2 = 108,
    RightAlt = 109,
    RightCtrl = 110,
    Backslash = 111,
    F13 = 112,
    F14 = 113,
    F15 = 114,
    F16 = 115,
    F17 = 116,
    F18 = 117,
    F19 = 118,
    F20 = 119,
    F21 = 120,
    F22 = 121,
    F23 = 122,
    F24 = 123,
    F25 = 124,
    F26 = 125,
    F27 = 126,
    F28 = 127,
    F29 = 128,
    F30 = 129,
    F31 = 130,
    F32 = 131,
    F33 = 132,
    F34 = 133,
    F35 = 134,
    RightShift = 135,
    Eject = 136,
    EqualNum = 137,
    ClearNum = 138,
    LeftCmd = 139,
    Function = 140,
    RightCmd = 141,

    // additional, not used by GW2
    Scroll = 200,
    Pause = 201,
    LeftWin = 202,
    RightWin = 203,
    Menu = 204,
}

/// Custom mouse codes.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(i32)]
pub enum MouseCode {
    Mouse1 = 0,
    Mouse3 = 1,
    Mouse2 = 2,
    Mouse4 = 3,
    Mouse5 = 4,
    Mouse6 = 5,
    Mouse7 = 6,
    Mouse8 = 7,
    Mouse9 = 8,
    Mouse10 = 9,
    Mouse11 = 10,
    Mouse12 = 11,
    Mouse13 = 12,
    Mouse14 = 13,
    Mouse15 = 14,
    Mouse16 = 15,
    Mouse17 = 16,
    Mouse18 = 17,
    Mouse19 = 18,
    Mouse20 = 19,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(i32)]
pub enum DeviceType {
    #[num_enum(default)]
    Unset = 0,
    Mouse = 1,
    Keyboard = 2,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive, TryFromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(Display, EnumCount, EnumIter, IntoStaticStr, EnumVariantNames)
)]
#[repr(i32)]
pub enum Modifier {
    Shift = 1,
    Ctrl = 2,
    Alt = 4,
}
