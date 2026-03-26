use crate::offsets::offsets_rtti::RTTI_NonActorMagicCaster;
use crate::offsets::offsets_vtable::VTABLE_NonActorMagicCaster;
use crate::re::{ActorHandle, ExtraDataType, ExtraDataTyped, ExtraMagicCaster, TESObjectREFR};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::NonActorMagicCaster`
#[repr(C)]
pub struct NonActorMagicCaster {
    pub base: ExtraMagicCaster,    // 00
    pub unk58: *mut TESObjectREFR, // 58
    pub blame_actor: ActorHandle,  // 60
    pub unk64: u32,                // 64
}

const _: () = assert!(core::mem::size_of::<NonActorMagicCaster>() == 0x68);
const _: () = assert!(core::mem::offset_of!(NonActorMagicCaster, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(NonActorMagicCaster, unk58) == 0x58);
const _: () = assert!(core::mem::offset_of!(NonActorMagicCaster, blame_actor) == 0x60);
const _: () = assert!(core::mem::offset_of!(NonActorMagicCaster, unk64) == 0x64);

impl ExtraDataTyped for NonActorMagicCaster {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::MagicCaster;
}

impl RttiType for NonActorMagicCaster {
    const RTTI: VariantID = RTTI_NonActorMagicCaster;
}

core_util::inherit!(NonActorMagicCaster : ExtraMagicCaster);

impl NonActorMagicCaster {
    pub const RTTI: VariantID = RTTI_NonActorMagicCaster;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NonActorMagicCaster;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::MagicCaster;

    // override (MagicCaster)
    // 00 ~NonActorMagicCaster
    // 01 CastSpellImmediate
    // 07 FinishCastImpl
    // 0B GetCasterStatsObject
    // 0C GetCasterAsActor
    // 0D GetCasterObjectReference
    // 0E GetMagicNode
    // 18 SaveGame
    // 19 LoadGame
}
