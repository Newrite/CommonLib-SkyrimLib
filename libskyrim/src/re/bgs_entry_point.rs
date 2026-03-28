use crate::re::{Actor, BGSEntryPointFunctionType};
use crate::relocation::RelocationID;

/// C++ `RE::BGSEntryPoint::ENTRY_POINT`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSEntryPointEntryPoint {
    CalculateWeaponDamage = 0,
    CalculateMyCriticalHitChance = 1,
    CalculateMyCriticalHitDamage = 2,
    CalculateMineExplodeChance = 3,
    AdjustLimbDamage = 4,
    AdjustBookSkillPoints = 5,
    ModRecoveredHealth = 6,
    GetShouldAttack = 7,
    ModBuyPrices = 8,
    AddLeveledListOnDeath = 9,
    GetMaxCarryWeight = 10,
    ModAddictionChance = 11,
    ModAddictionDuration = 12,
    ModPositiveChemDuration = 13,
    Activate = 14,
    IgnoreRunningDuringDetection = 15,
    IgnoreBrokenLock = 16,
    ModEnemyCriticalHitChance = 17,
    ModSneakAttackMult = 18,
    ModMaxPlaceableMines = 19,
    ModBowZoom = 20,
    ModRecoverArrowChance = 21,
    ModSkillUse = 22,
    ModTelekinesisDistance = 23,
    ModTelekinesisDamageMult = 24,
    ModTelekinesisDamage = 25,
    ModBashingDamage = 26,
    ModPowerAttackStamina = 27,
    ModPowerAttackDamage = 28,
    ModSpellMagnitude = 29,
    ModSpellDuration = 30,
    ModSecondaryValueWeight = 31,
    ModArmorWeight = 32,
    ModIncomingStagger = 33,
    ModTargetStagger = 34,
    ModAttackDamage = 35,
    ModIncomingDamage = 36,
    ModTargetDamageResistance = 37,
    ModSpellCost = 38,
    ModPercentBlocked = 39,
    ModShieldDeflectArrowChance = 40,
    ModIncomingSpellMagnitude = 41,
    ModIncomingSpellDuration = 42,
    ModPlayerIntimidation = 43,
    ModPlayerReputation = 44,
    ModFavorPoints = 45,
    ModBribeAmount = 46,
    ModDetectionLight = 47,
    ModDetectionMovement = 48,
    ModSoulGemRecharge = 49,
    SetSweepAttack = 50,
    ApplyCombatHitSpell = 51,
    ApplyBashingSpell = 52,
    ApplyReanimateSpell = 53,
    SetBooleanGraphVariable = 54,
    ModSpellCastingSoundEvent = 55,
    ModPickpocketChance = 56,
    ModDetectionSneakSkill = 57,
    ModFallingDamage = 58,
    ModLockpickSweetSpot = 59,
    ModSellPrices = 60,
    CanPickpocketEquippedItem = 61,
    ModLockpickLevelAllowed = 62,
    SetLockpickStartingArc = 63,
    SetProgressionPicking = 64,
    MakeLockpicksUnbreakable = 65,
    ModAlchemyEffectiveness = 66,
    ApplyWeaponSwingSpell = 67,
    ModCommandedActorLimit = 68,
    ApplySneakingSpell = 69,
    ModPlayerMagicSlowdown = 70,
    ModWardMagickaAbsorptionPct = 71,
    ModInitialIngredientEffectsLearned = 72,
    PurifyAlchemyIngredients = 73,
    FilterActivation = 74,
    CanDualCastSpell = 75,
    ModTemperingHealth = 76,
    ModEnchantmentPower = 77,
    ModSoulPctCapturedToWeapon = 78,
    ModSoulGemEnchanting = 79,
    ModNumberAppliedEnchantmentsAllowed = 80,
    SetActivateLabel = 81,
    ModShoutOK = 82,
    ModPoisonDoseCount = 83,
    ShouldApplyPlacedItem = 84,
    ModArmorRating = 85,
    ModLockpickingCrimeChance = 86,
    ModIngredientsHarvested = 87,
    ModSpellRangeTargetLoc = 88,
    ModPotionsCreated = 89,
    ModLockpickingKeyRewardChance = 90,
    AllowMountActor = 91,
    Total = 92,
}

/// C++ `RE::BGSEntryPoint::EntryPointParameter`
#[repr(C)]
pub struct BGSEntryPointParameter {
    pub name: *const i8, // 00
    pub non_actor: bool, // 08
    pub pad09: u8,       // 09
    pub pad0a: u16,      // 0A
    pub pad0c: u32,      // 0C
}

const _: () = assert!(core::mem::size_of::<BGSEntryPointParameter>() == 0x10);

impl BGSEntryPointParameter {
    #[inline(always)]
    pub fn get_name_as_str(&self) -> &str {
        core_util::ptr_to_str(self.name)
    }
}

/// C++ `RE::BGSEntryPoint::EntryPointParameters`
#[repr(C)]
pub struct BGSEntryPointParameters {
    pub count: u32,                        // 00
    pub pad04: u32,                        // 04
    pub data: *mut BGSEntryPointParameter, // 08
}

const _: () = assert!(core::mem::size_of::<BGSEntryPointParameters>() == 0x10);

impl BGSEntryPointParameters {
    #[inline(always)]
    pub fn as_slice(&self) -> &[BGSEntryPointParameter] {
        if self.data.is_null() || self.count == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.count as usize) }
        }
    }
}

/// C++ `RE::BGSEntryPoint::EntryPoint`
#[repr(C)]
pub struct BGSEntryPointData {
    pub name: *const i8,                          // 00
    pub parameters: BGSEntryPointParameters,      // 08
    pub function_type: BGSEntryPointFunctionType, // 18
    pub pad1c: u32,                               // 1C
}

const _: () = assert!(core::mem::size_of::<BGSEntryPointData>() == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSEntryPointData, parameters) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSEntryPointData, function_type) == 0x18);

impl BGSEntryPointData {
    #[inline(always)]
    pub fn get_name_as_str(&self) -> &str {
        core_util::ptr_to_str(self.name)
    }

    #[inline(always)]
    pub fn parameters_slice(&self) -> &[BGSEntryPointParameter] {
        self.parameters.as_slice()
    }
}

/// C++ `RE::BGSEntryPoint::ENTRY_POINTS`
pub struct BGSEntryPoints;

impl BGSEntryPoints {
    pub const TOTAL: usize = BGSEntryPointEntryPoint::Total as usize;
}

/// C++ `RE::BGSEntryPoint`
pub struct BGSEntryPoint;

impl BGSEntryPoint {
    crate::relocation_variable! {
        fn entry_points() -> *mut BGSEntryPointData
            => RelocationID::new(675707, 368994), is_ptr
    }

    #[inline(always)]
    pub fn get_entry_point(entry_point: BGSEntryPointEntryPoint) -> *mut BGSEntryPointData {
        let index = entry_point as usize;
        if index < BGSEntryPoints::TOTAL {
            unsafe { Self::entry_points().add(index) }
        } else {
            core::ptr::null_mut()
        }
    }

    #[inline(always)]
    pub fn get_entry_point_ref(
        entry_point: BGSEntryPointEntryPoint,
    ) -> Option<&'static BGSEntryPointData> {
        unsafe { Self::get_entry_point(entry_point).as_ref() }
    }

    // TODO: Add source-backed call-site-specific wrappers for `BGSEntryPoint::HandleEntryPoint`
    // when a concrete signature is needed. The vendored surface only exposes a variadic template,
    // so a universal Rust helper here would guess ABI rather than translate it honestly.
    #[allow(dead_code)]
    fn _handle_entry_point_owner_type(_: *mut Actor) {}
}
