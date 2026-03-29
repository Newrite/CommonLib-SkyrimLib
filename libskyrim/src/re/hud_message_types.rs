#![allow(non_camel_case_types)]

use crate::rex::{EnumSet, EnumSetType};

/// Source-backed Rust storage wrapper for C++ `REX::EnumSet<RE::HUD_MESSAGE_TYPE, u32>`.
pub type HUDMessageTypeSet = EnumSet<HUD_MESSAGE_TYPE, u32>;

/// Runtime-aware C++ `RE::HUD_MESSAGE_TYPE`.
///
/// `HUDMessageTypes.h` does not define a split runtime layout. Instead, it keeps
/// `0..=4` stable, injects VR-only values at raw `5..=7`, and uses
/// `GetHUDMessageType(...)` to shift canonical SE/AE values `>= kShowSubtitle`
/// by `+3` on VR before storing them in `REX::EnumSet<HUD_MESSAGE_TYPE, u32>`
/// fields such as `HUDData::type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HUD_MESSAGE_TYPE {
    kNone,
    kNotification,
    kSetCrosshairTarget,
    kSetCrosshairTargetTextOnly,
    kSetLoadDoorInfo,
    kSetCrosshairTargetGamepad,
    kSetCrosshairTargetLeft,
    kSetCrosshairTargetRight,
    kShowSubtitle,
    kHideSubtitle,
    kShowArrowCount,
    kHeartBeat,
    kSetSubtitlesEnabled,
    kSetBlinking,
    kSetFadeOut,
    kSetPct,
    kQuestStarted,
    kQuestComplete,
    kQuestFailed,
    kObjectiveStarted,
    kObjectiveComplete,
    kObjectiveFailed,
    kSkillIncrease,
    kWordOfPowerLearned,
    kDragonSoulAbsorbed,
    kSetMode,
    kCrosshairSneak,
    kUnk25,
    kLocationDiscovery,
    kFavor,
    kValidateCrosshair,
    kShowLocationName,
    kShowHintText,
    kSetCrosshairEnabled,
    kSetDisplayInfo,
    kRefreshActivateButtonArt,
    kRefreshAll,
    kSurvival,
}

impl Default for HUD_MESSAGE_TYPE {
    #[inline(always)]
    fn default() -> Self {
        Self::kNone
    }
}

impl HUD_MESSAGE_TYPE {
    #[inline(always)]
    pub const fn is_vr_only(self) -> bool {
        matches!(
            self,
            Self::kSetCrosshairTargetGamepad
                | Self::kSetCrosshairTargetLeft
                | Self::kSetCrosshairTargetRight
        )
    }

    /// Returns the non-shifted value declared in `HUDMessageTypes.h`.
    #[inline(always)]
    pub const fn declared_underlying(self) -> u32 {
        match self {
            Self::kNone => 0,
            Self::kNotification => 1,
            Self::kSetCrosshairTarget => 2,
            Self::kSetCrosshairTargetTextOnly => 3,
            Self::kSetLoadDoorInfo => 4,
            Self::kSetCrosshairTargetGamepad => 5,
            Self::kSetCrosshairTargetLeft => 6,
            Self::kSetCrosshairTargetRight => 7,
            Self::kShowSubtitle => 5,
            Self::kHideSubtitle => 6,
            Self::kShowArrowCount => 7,
            Self::kHeartBeat => 8,
            Self::kSetSubtitlesEnabled => 9,
            Self::kSetBlinking => 11,
            Self::kSetFadeOut => 12,
            Self::kSetPct => 13,
            Self::kQuestStarted => 14,
            Self::kQuestComplete => 15,
            Self::kQuestFailed => 16,
            Self::kObjectiveStarted => 17,
            Self::kObjectiveComplete => 18,
            Self::kObjectiveFailed => 19,
            Self::kSkillIncrease => 20,
            Self::kWordOfPowerLearned => 21,
            Self::kDragonSoulAbsorbed => 22,
            Self::kSetMode => 23,
            Self::kCrosshairSneak => 24,
            Self::kUnk25 => 25,
            Self::kLocationDiscovery => 26,
            Self::kFavor => 27,
            Self::kValidateCrosshair => 28,
            Self::kShowLocationName => 29,
            Self::kShowHintText => 30,
            Self::kSetCrosshairEnabled => 31,
            Self::kSetDisplayInfo => 32,
            Self::kRefreshActivateButtonArt => 33,
            Self::kRefreshAll => 34,
            Self::kSurvival => 35,
        }
    }

    /// Returns the raw runtime storage value used by `REX::EnumSet` owners.
    #[inline(always)]
    pub fn raw(self) -> u32 {
        let value = self.declared_underlying();
        if self.is_vr_only() {
            value
        } else if crate::runtime::is_vr() && value >= Self::kShowSubtitle.declared_underlying() {
            value + 3
        } else {
            value
        }
    }

    /// C++ `RE::GetHUDMessageType(HUD_MESSAGE_TYPE seValue)`, adapted to the
    /// actual `REX::EnumSet<HUD_MESSAGE_TYPE, u32>` storage used by HUD owners.
    #[inline(always)]
    pub fn get_hud_message_type(se_value: Self) -> HUDMessageTypeSet {
        HUDMessageTypeSet::from_underlying(se_value.raw())
    }

    #[inline(always)]
    fn from_canonical_underlying(value: u32) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::kNone),
            1 => Ok(Self::kNotification),
            2 => Ok(Self::kSetCrosshairTarget),
            3 => Ok(Self::kSetCrosshairTargetTextOnly),
            4 => Ok(Self::kSetLoadDoorInfo),
            5 => Ok(Self::kShowSubtitle),
            6 => Ok(Self::kHideSubtitle),
            7 => Ok(Self::kShowArrowCount),
            8 => Ok(Self::kHeartBeat),
            9 => Ok(Self::kSetSubtitlesEnabled),
            11 => Ok(Self::kSetBlinking),
            12 => Ok(Self::kSetFadeOut),
            13 => Ok(Self::kSetPct),
            14 => Ok(Self::kQuestStarted),
            15 => Ok(Self::kQuestComplete),
            16 => Ok(Self::kQuestFailed),
            17 => Ok(Self::kObjectiveStarted),
            18 => Ok(Self::kObjectiveComplete),
            19 => Ok(Self::kObjectiveFailed),
            20 => Ok(Self::kSkillIncrease),
            21 => Ok(Self::kWordOfPowerLearned),
            22 => Ok(Self::kDragonSoulAbsorbed),
            23 => Ok(Self::kSetMode),
            24 => Ok(Self::kCrosshairSneak),
            25 => Ok(Self::kUnk25),
            26 => Ok(Self::kLocationDiscovery),
            27 => Ok(Self::kFavor),
            28 => Ok(Self::kValidateCrosshair),
            29 => Ok(Self::kShowLocationName),
            30 => Ok(Self::kShowHintText),
            31 => Ok(Self::kSetCrosshairEnabled),
            32 => Ok(Self::kSetDisplayInfo),
            33 => Ok(Self::kRefreshActivateButtonArt),
            34 => Ok(Self::kRefreshAll),
            35 => Ok(Self::kSurvival),
            _ => Err(()),
        }
    }
}

impl EnumSetType<u32> for HUD_MESSAGE_TYPE {
    #[inline(always)]
    fn to_underlying(self) -> u32 {
        self.raw()
    }
}

impl TryFrom<u32> for HUD_MESSAGE_TYPE {
    type Error = ();

    #[inline(always)]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if crate::runtime::is_vr() {
            match value {
                5 => Ok(Self::kSetCrosshairTargetGamepad),
                6 => Ok(Self::kSetCrosshairTargetLeft),
                7 => Ok(Self::kSetCrosshairTargetRight),
                8..=38 => Self::from_canonical_underlying(value - 3),
                _ => Self::from_canonical_underlying(value),
            }
        } else {
            Self::from_canonical_underlying(value)
        }
    }
}
