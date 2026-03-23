use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSHeadPart;
use crate::offsets::offsets_vtable::VTABLE_BGSHeadPart;
use crate::re::bgs_color_form::BGSColorForm;
use crate::re::bgs_list_form::BGSListForm;
use crate::re::bgs_texture_set::BGSTextureSet;
use crate::re::bst_array::BSTArray;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_form::TESForm;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::re::tes_model_tri::TESModelTri;
use crate::relocation::{RttiType, VariantID};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeadPartType {
    Misc = 0,
    Face = 1,
    Eyes = 2,
    Hair = 3,
    FacialHair = 4,
    Scar = 5,
    Eyebrows = 6,
    Total = 7,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct HeadPartFlags: u8 {
        const NONE = 0;
        const PLAYABLE = 1 << 0;
        const MALE = 1 << 1;
        const FEMALE = 1 << 2;
        const IS_EXTRA_PART = 1 << 3;
        const USE_SOLID_TINT = 1 << 4;
    }
}

pub const HEAD_PART_MORPH_TOTAL: usize = 3;

#[repr(C)]
pub struct BGSHeadPart {
    pub base: TESForm,                                     // 0x000
    pub full_name: TESFullName,                            // 0x020
    pub model_texture_swap: TESModelTextureSwap,           // 0x030
    pub flags: HeadPartFlags,                              // 0x068 - DATA
    pub pad069: u8,                                        // 0x069
    pub pad06a: u16,                                       // 0x06A
    pub part_type: HeadPartType,                           // 0x06C - PNAM
    pub extra_parts: BSTArray<*mut BGSHeadPart>,           // 0x070
    pub texture_set: *mut BGSTextureSet,                   // 0x088 - TNAM
    pub morphs: [TESModelTri; HEAD_PART_MORPH_TOTAL],      // 0x090
    pub color: *mut BGSColorForm,                          // 0x108 - CNAM
    pub valid_races: *mut BGSListForm,                     // 0x110 - RNAM
    pub form_editor_id: BSFixedString,                     // 0x118 - EDID
}

const _: () = assert!(core::mem::size_of::<BGSHeadPart>() == 0x120);
const _: () = assert!(core::mem::offset_of!(BGSHeadPart, full_name) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSHeadPart, model_texture_swap) == 0x30);

impl RttiType for BGSHeadPart {
    const RTTI: VariantID = RTTI_BGSHeadPart;
}

impl FormCastable for BGSHeadPart {
    const TARGET_FORM_TYPE: FormType = FormType::HeadPart;
}

inherit!(BGSHeadPart : TESForm);
inherit!(BGSHeadPart => TESFullName, full_name);
inherit!(BGSHeadPart => TESModelTextureSwap, model_texture_swap);

impl BGSHeadPart {
    pub const RTTI: VariantID = RTTI_BGSHeadPart;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSHeadPart;
    pub const FORMTYPE: FormType = FormType::HeadPart;

    // override (TESForm)
    // void InitializeData() override;                    // 04
    // void ClearData() override;                         // 05
    // bool Load(TESFile* a_mod) override;                // 06
    // void InitItemImpl() override;                      // 13
    // const char* GetFormEditorID() const override;      // 32
    // bool SetFormEditorID(const char* a_str) override;  // 33

    #[inline]
    pub fn is_extra_part(&self) -> bool {
        self.flags.contains(HeadPartFlags::IS_EXTRA_PART)
    }
}

pub trait BGSHeadPartExt {
    fn is_extra_part(&self) -> bool;
}

impl<T: AsRef<BGSHeadPart>> BGSHeadPartExt for T {
    fn is_extra_part(&self) -> bool {
        self.as_ref().is_extra_part()
    }
}
