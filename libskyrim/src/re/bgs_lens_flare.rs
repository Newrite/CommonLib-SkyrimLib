use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_BGSLensFlare;
use crate::offsets::offsets_vtable::VTABLE_BGSLensFlare;
use crate::re::BSLensFlareRenderData;
use crate::re::TESForm;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct BGSLensFlare {
    pub base: TESForm,                                 // 0x00
    pub lens_flare_render_data: BSLensFlareRenderData, // 0x20
}

const _: () = assert!(core::mem::size_of::<BGSLensFlare>() == 0x40);
const _: () = assert!(core::mem::offset_of!(BGSLensFlare, lens_flare_render_data) == 0x20);

impl RttiType for BGSLensFlare {
    const RTTI: VariantID = RTTI_BGSLensFlare;
}

inherit!(BGSLensFlare : TESForm);
inherit!(BGSLensFlare => BSLensFlareRenderData, lens_flare_render_data);

impl crate::re::FormCastable for BGSLensFlare {
    const TARGET_FORM_TYPE: crate::re::FormType = crate::re::FormType::LensFlare;
}

impl BGSLensFlare {
    pub const RTTI: VariantID = RTTI_BGSLensFlare;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSLensFlare;
    pub const FORMTYPE: crate::re::FormType = crate::re::FormType::LensFlare;

    // override (TESForm)
    // void ClearData() override;           // 05
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13
}
