use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_RaceSexCamera;
use crate::offsets::offsets_vtable::VTABLE_RaceSexCamera;
use crate::re::TESCamera;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::RaceSexCamera`
#[repr(C)]
pub struct RaceSexCamera {
    pub base: TESCamera, // 00
    pub unk38: u64,      // 38
    pub unk40: u64,      // 40
    pub unk48: u64,      // 48
    pub unk50: u64,      // 50
}

const _: () = assert!(core::mem::size_of::<RaceSexCamera>() == 0x58);
const _: () = assert!(core::mem::offset_of!(RaceSexCamera, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(RaceSexCamera, unk38) == 0x38);

impl RttiType for RaceSexCamera {
    const RTTI: VariantID = RTTI_RaceSexCamera;
}

inherit!(RaceSexCamera : TESCamera, base);

impl RaceSexCamera {
    pub const RTTI: VariantID = RTTI_RaceSexCamera;
    pub const VTABLE: &'static [VariantID] = &VTABLE_RaceSexCamera;
}
