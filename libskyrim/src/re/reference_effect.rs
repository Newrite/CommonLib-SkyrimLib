use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_ReferenceEffect;
use crate::offsets::offsets_rtti::RTTI_ReferenceEffect;
use crate::offsets::offsets_vtable::VTABLE_ReferenceEffect;
use crate::re::BSTempEffect;
use crate::re::ObjectRefHandle;
use crate::re::ReferenceEffectController;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ReferenceEffect`
#[repr(C)]
pub struct ReferenceEffect {
    pub base: BSTempEffect,                         // 00
    pub controller: *mut ReferenceEffectController, // 30
    pub target: ObjectRefHandle,                    // 38
    pub aim_at_target: ObjectRefHandle,             // 3C
    pub finished: bool,                             // 40
    pub own_controller: bool,                       // 41
    pub pad42: u16,                                 // 42
    pub pad44: u32,                                 // 44
}

const _: () = assert!(core::mem::size_of::<ReferenceEffect>() == 0x48);
const _: () = assert!(core::mem::offset_of!(ReferenceEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ReferenceEffect, controller) == 0x30);
const _: () = assert!(core::mem::offset_of!(ReferenceEffect, target) == 0x38);
const _: () = assert!(core::mem::offset_of!(ReferenceEffect, aim_at_target) == 0x3C);
const _: () = assert!(core::mem::offset_of!(ReferenceEffect, finished) == 0x40);
const _: () = assert!(core::mem::offset_of!(ReferenceEffect, own_controller) == 0x41);
const _: () = assert!(core::mem::offset_of!(ReferenceEffect, pad42) == 0x42);
const _: () = assert!(core::mem::offset_of!(ReferenceEffect, pad44) == 0x44);

impl RttiType for ReferenceEffect {
    const RTTI: VariantID = RTTI_ReferenceEffect;
}

inherit!(ReferenceEffect : BSTempEffect);

impl ReferenceEffect {
    pub const RTTI: VariantID = RTTI_ReferenceEffect;
    pub const NI_RTTI: VariantID = NiRTTI_ReferenceEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ReferenceEffect;
    pub const TYPE: crate::re::TempEffectType = crate::re::TempEffectType::RefDefault;

    // override (BSTempEffect)
    // const NiRTTI* GetRTTI() const override;                   // 02
    // void          Detach() override;                          // 27
    // bool          Update(float a_arg1) override;              // 28
    // bool          GetManagerHandlesSaveLoad() const override; // 2A
    // bool          GetClearWhenCellIsUnloaded() const override;// 2B
    // TEMP_EFFECT_TYPE GetType() const override;                // 2C
    // void          SaveGame(BGSSaveGameBuffer* a_buf) override; // 2D
    // void          LoadGame(BGSLoadGameBuffer* a_buf) override; // 2E
    // void          FinishLoadGame(BGSLoadGameBuffer* a_buf) override; // 2F
}
