use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_ActiveEffectReferenceEffectController;
use crate::offsets::offsets_vtable::VTABLE_ActiveEffectReferenceEffectController;
use crate::re::ActiveEffect;
use crate::re::NiPoint3;
use crate::re::ObjectRefHandle;
use crate::re::ReferenceEffectController;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ActiveEffectReferenceEffectController`
#[repr(C)]
pub struct ActiveEffectReferenceEffectController {
    pub base: ReferenceEffectController, // 08
    pub effect: *mut ActiveEffect,       // 08
    pub target: ObjectRefHandle,         // 10
    pub wind_point: NiPoint3,            // 14
}

const _: () = assert!(core::mem::size_of::<ActiveEffectReferenceEffectController>() == 0x20);
const _: () = assert!(core::mem::offset_of!(ActiveEffectReferenceEffectController, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ActiveEffectReferenceEffectController, effect) == 0x08);
const _: () = assert!(core::mem::offset_of!(ActiveEffectReferenceEffectController, target) == 0x10);
const _: () =
    assert!(core::mem::offset_of!(ActiveEffectReferenceEffectController, wind_point) == 0x14);

impl RttiType for ActiveEffectReferenceEffectController {
    const RTTI: VariantID = RTTI_ActiveEffectReferenceEffectController;
}

inherit!(ActiveEffectReferenceEffectController : ReferenceEffectController);

impl ActiveEffectReferenceEffectController {
    pub const RTTI: VariantID = RTTI_ActiveEffectReferenceEffectController;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ActiveEffectReferenceEffectController;

    // override (ReferenceEffectController)
    // void             HandleEvent(const BSFixedString& a_event) override; // 01
    // float            GetElapsedTime() override;                          // 02
    // float            GetScale() override;                                // 03
    // void             SwitchAttachedRoot(NiNode*, NiNode*) override;      // 04
    // const NiPoint3&  GetSourcePosition() override;                       // 05
    // bool             GetUseSourcePosition() override;                    // 06
    // bool             GetNoInitialFlare() override;                       // 07
    // bool             GetEffectPersists() override;                       // 08
    // bool             GetGoryVisuals() override;                          // 09
    // void             RemoveHitEffect(ReferenceEffect*) override;         // 0A
    // TESObjectREFR*   GetTargetReference() override;                      // 0B
    // BGSArtObject*    GetHitEffectArt() override;                         // 0C
    // TESEffectShader* GetHitEffectShader() override;                      // 0D
    // bool             GetManagerHandlesSaveLoad() override;               // 0E
    // bool             EffectShouldFaceTarget() override;                  // 17
    // TESObjectREFR*   GetFacingTarget() override;                         // 18
    // void             SetWindPoint(const NiPoint3& a_point) override;     // 1E
    // const NiPoint3&  GetWindPoint() override;                            // 1F
    // bool             GetAllowNo3D() override;                            // 20
    // void             SaveGame(BGSSaveGameBuffer* a_buf) override;        // 21
    // void             LoadGame(BGSLoadGameBuffer* a_buf) override;        // 22
}
