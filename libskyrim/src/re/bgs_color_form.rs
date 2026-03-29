use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_BGSColorForm;
use crate::offsets::offsets_vtable::VTABLE_BGSColorForm;
use crate::re::{Color, FormCastable, FormType, TESForm, TESFullName};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BGSColorForm::Flag`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSColorFormFlag {
    None = 0,
    Playable = 1 << 0,
}

core_util::impl_enumset_type!(BGSColorFormFlag => u32);

bitflags! {
    /// C++ `RE::BGSColorForm::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSColorFormRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::BGSColorForm`
#[repr(C)]
pub struct BGSColorForm {
    pub base: TESForm,                         // 00
    pub full_name: TESFullName,                // 20
    pub color: Color,                          // 30 - CNAM
    pub flags: EnumSet<BGSColorFormFlag, u32>, // 34 - FNAM
}

const _: () = assert!(core::mem::size_of::<BGSColorForm>() == 0x38);
const _: () = assert!(core::mem::offset_of!(BGSColorForm, full_name) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSColorForm, color) == 0x30);
const _: () = assert!(core::mem::offset_of!(BGSColorForm, flags) == 0x34);

impl RttiType for BGSColorForm {
    const RTTI: VariantID = RTTI_BGSColorForm;
}

impl FormCastable for BGSColorForm {
    const TARGET_FORM_TYPE: FormType = FormType::ColorForm;
}

inherit!(BGSColorForm : TESForm, base);
inherit!(BGSColorForm => TESFullName, full_name);

impl BGSColorForm {
    pub const RTTI: VariantID = RTTI_BGSColorForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSColorForm;
    pub const FORMTYPE: FormType = FormType::ColorForm;

    // override (TESForm)
    // void InitializeData() override;      // 04
    // void ClearData() override;           // 05
    // bool Load(TESFile* a_mod) override;  // 06

    #[inline(always)]
    pub fn is_playable(&self) -> bool {
        self.flags.all(BGSColorFormFlag::Playable)
    }
}
