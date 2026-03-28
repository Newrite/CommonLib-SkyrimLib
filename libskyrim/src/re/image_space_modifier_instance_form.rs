use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ImageSpaceModifierInstanceForm;
use crate::offsets::offsets_vtable::VTABLE_ImageSpaceModifierInstanceForm;
use crate::re::{ImageSpaceModifierInstance, NiAVObject, NiRef, TESImageSpaceModifier};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::ImageSpaceModifierInstanceForm`
#[repr(C)]
pub struct ImageSpaceModifierInstanceForm {
    pub base: ImageSpaceModifierInstance, // 00
    pub imod: *mut TESImageSpaceModifier, // 28
    pub unk30: u64,                       // 30
    pub unk38: f32,                       // 38
    pub unk3c: u32,                       // 3C
    pub unk40: u64,                       // 40
    pub unk48: u32,                       // 48
    pub pad4c: u32,                       // 4C
}

const _: () = assert!(core::mem::size_of::<ImageSpaceModifierInstanceForm>() == 0x50);
const _: () = assert!(core::mem::offset_of!(ImageSpaceModifierInstanceForm, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ImageSpaceModifierInstanceForm, imod) == 0x28);
const _: () = assert!(core::mem::offset_of!(ImageSpaceModifierInstanceForm, unk30) == 0x30);
const _: () = assert!(core::mem::offset_of!(ImageSpaceModifierInstanceForm, unk48) == 0x48);

inherit!(ImageSpaceModifierInstanceForm : ImageSpaceModifierInstance);

impl RttiType for ImageSpaceModifierInstanceForm {
    const RTTI: VariantID = RTTI_ImageSpaceModifierInstanceForm;
}

impl NiRef for ImageSpaceModifierInstanceForm {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

impl ImageSpaceModifierInstanceForm {
    pub const RTTI: VariantID = RTTI_ImageSpaceModifierInstanceForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ImageSpaceModifierInstanceForm;

    // override (ImageSpaceModifierInstance)
    // ~ImageSpaceModifierInstanceForm() override; // 00
    // bool Unk_25() override;                     // 25
    // void Apply() override;                     // 26
    // ImageSpaceModifierInstanceForm* IsForm();  // 27
    // void PrintInfo(char* a_dstBuf) override;   // 28

    crate::relocation_func! {
        pub fn trigger(
            imod: *mut TESImageSpaceModifier,
            strength: f32,
            target: *mut NiAVObject,
        ) -> *mut ImageSpaceModifierInstanceForm => RelocationID::new(18185, 18570)
    }

    crate::relocation_func! {
        pub fn stop(imod: *mut TESImageSpaceModifier) => RelocationID::new(18188, 18573)
    }

    crate::relocation_func! {
        pub fn stop_cross_fade(seconds: f32) => RelocationID::new(18192, 18577)
    }
}
