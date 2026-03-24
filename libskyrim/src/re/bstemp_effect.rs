use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_BSTempEffect;
use crate::offsets::offsets_rtti::RTTI_BSTempEffect;
use crate::offsets::offsets_vtable::VTABLE_BSTempEffect;
use crate::re::NiObject;
use crate::re::TESObjectCELL;
use crate::relocation::{RttiType, VariantID};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TempEffectType {
    Terrain = 0,
    WeaponBlood = 1,
    Decal = 2,
    GeometryDecal = 3,
    Particle = 4,
    Debris = 5,
    Spg = 6,
    Default = 7,
    RefDefault = 8,
    RefModel = 9,
    RefShader = 10,
    MagicSummon = 11,
}

/// C++ `RE::BSTempEffect`
#[repr(C)]
pub struct BSTempEffect {
    pub base: NiObject,           // 00
    pub lifetime: f32,            // 10
    pub pad14: u32,               // 14
    pub cell: *mut TESObjectCELL, // 18
    pub age: f32,                 // 20
    pub initialized: bool,        // 24
    pub pad25: u8,                // 25
    pub pad26: u16,               // 26
    pub effect_id: u32,           // 28
    pub pad2c: u32,               // 2C
}

const _: () = assert!(core::mem::size_of::<BSTempEffect>() == 0x30);
const _: () = assert!(core::mem::offset_of!(BSTempEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSTempEffect, lifetime) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSTempEffect, pad14) == 0x14);
const _: () = assert!(core::mem::offset_of!(BSTempEffect, cell) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSTempEffect, age) == 0x20);
const _: () = assert!(core::mem::offset_of!(BSTempEffect, initialized) == 0x24);
const _: () = assert!(core::mem::offset_of!(BSTempEffect, pad25) == 0x25);
const _: () = assert!(core::mem::offset_of!(BSTempEffect, pad26) == 0x26);
const _: () = assert!(core::mem::offset_of!(BSTempEffect, effect_id) == 0x28);
const _: () = assert!(core::mem::offset_of!(BSTempEffect, pad2c) == 0x2C);

impl RttiType for BSTempEffect {
    const RTTI: VariantID = RTTI_BSTempEffect;
}

impl crate::re::NiRef for BSTempEffect {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(BSTempEffect : NiObject);

impl BSTempEffect {
    pub const RTTI: VariantID = RTTI_BSTempEffect;
    pub const NI_RTTI: VariantID = NiRTTI_BSTempEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSTempEffect;
    pub const TYPE: TempEffectType = TempEffectType::Default;

    // override (NiObject)
    // const NiRTTI* GetRTTI() const override;             // 02

    // add
    // void               Initialize() {}                  // 25
    // void               Attach() {}                      // 26
    // void               Detach() {}                      // 27
    // bool               Update(float) = 0;               // 28
    // NiAVObject*        Get3D() const {}                 // 29
    // bool               GetManagerHandlesSaveLoad() {}   // 2A
    // bool               GetClearWhenCellIsUnloaded() {}  // 2B
    // TEMP_EFFECT_TYPE   GetType() const {}               // 2C
    // void               SaveGame(BGSSaveGameBuffer*) {}  // 2D
    // void               LoadGame(BGSLoadGameBuffer*) {}  // 2E
    // void               FinishLoadGame(BGSLoadGameBuffer*) {}  // 2F
    // bool               IsInterfaceEffect() const {}     // 30
    // void               SetInterfaceEffect(bool) {}      // 31
    // bool               GetStackable() const {}          // 32
    // bool               GetStackableMatch(BSTempEffect*) const {}  // 33
    // void               Push() {}                        // 34
    // void               Pop() {}                         // 35
}
