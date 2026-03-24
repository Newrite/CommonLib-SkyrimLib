use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_BGSVoiceType;
use crate::offsets::offsets_vtable::VTABLE_BGSVoiceType;
use crate::re::{BSString, FormCastable, FormType, TESForm};
use crate::relocation::{RttiType, VariantID};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSVoiceTypeFlags {
    None = 0,
    AllowDefaultDialogue = 1 << 0,
    Female = 1 << 1,
}

core_util::impl_enumset_type!(BGSVoiceTypeFlags => u8);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSVoiceTypeRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct VoiceTypeData {
    pub flags: EnumSet<BGSVoiceTypeFlags, u8>, // 00
}

const _: () = assert!(core::mem::size_of::<VoiceTypeData>() == 0x1);

#[repr(C)]
pub struct BGSVoiceType {
    pub base: TESForm,            // 00
    pub data: VoiceTypeData,      // 20
    pub pad21: u8,                // 21
    pub pad22: u16,               // 22
    pub pad24: u16,               // 24
    pub form_editor_id: BSString, // 28
}

const _: () = assert!(core::mem::size_of::<BGSVoiceType>() == 0x38);
const _: () = assert!(core::mem::offset_of!(BGSVoiceType, data) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSVoiceType, form_editor_id) == 0x28);

impl RttiType for BGSVoiceType {
    const RTTI: VariantID = RTTI_BGSVoiceType;
}

impl FormCastable for BGSVoiceType {
    const TARGET_FORM_TYPE: FormType = FormType::VoiceType;
}

inherit!(BGSVoiceType : TESForm);

impl BGSVoiceType {
    pub const RTTI: VariantID = RTTI_BGSVoiceType;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSVoiceType;
    pub const FORMTYPE: FormType = FormType::VoiceType;

    // override (TESForm)
    // bool        Load(TESFile* a_mod) override;                // 06
    // const char* GetFormEditorID() const override;             // 32
    // bool        SetFormEditorID(const char* a_str) override;  // 33

    #[inline]
    pub const fn allows_default_dialogue(&self) -> bool {
        self.data
            .flags
            .all_underlying(BGSVoiceTypeFlags::AllowDefaultDialogue as u8)
    }

    #[inline]
    pub const fn is_female(&self) -> bool {
        self.data
            .flags
            .all_underlying(BGSVoiceTypeFlags::Female as u8)
    }

    #[inline]
    pub fn get_form_editor_id_as_str(&self) -> &str {
        self.form_editor_id.as_str()
    }

    #[inline]
    pub fn set_form_editor_id_from_str(&mut self, editor_id: &str) -> bool {
        self.form_editor_id.set_str(editor_id)
    }
}
