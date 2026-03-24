use core::ffi::c_char;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSMusicType;
use crate::offsets::offsets_vtable::VTABLE_BGSMusicType;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bsi_music_type::BSIMusicType;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_form::TESForm;
use crate::relocation::{RttiType, VariantID};

bitflags::bitflags! {
    /// C++ `RE::BGSMusicType::RecordFlags`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSMusicTypeRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::BGSMusicType`
#[repr(C)]
pub struct BGSMusicType {
    pub base: TESForm,                 // 00
    pub bsi_music_type: BSIMusicType,  // 20
    pub form_editor_id: BSFixedString, // 70 - EDID
}

const _: () = assert!(core::mem::size_of::<BGSMusicType>() == 0x78);
const _: () = assert!(core::mem::offset_of!(BGSMusicType, bsi_music_type) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSMusicType, form_editor_id) == 0x70);

impl RttiType for BGSMusicType {
    const RTTI: VariantID = RTTI_BGSMusicType;
}

impl FormCastable for BGSMusicType {
    const TARGET_FORM_TYPE: FormType = FormType::MusicType;
}

inherit!(BGSMusicType : TESForm);
inherit!(BGSMusicType => BSIMusicType, bsi_music_type);

impl BGSMusicType {
    pub const RTTI: VariantID = RTTI_BGSMusicType;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSMusicType;
    pub const FORMTYPE: FormType = FormType::MusicType;

    // override (TESForm)
    // void ClearData() override;                         // 05
    // bool Load(TESFile* a_mod) override;                // 06
    // void InitItemImpl() override;                      // 13
    // const char* GetFormEditorID() const override;      // 32
    // bool SetFormEditorID(const char* a_str) override;  // 33

    // override (BSIMusicType)
    // void DoUpdate() override;                                       // 00
    // void DoPlay() override;                                         // 01
    // void DoPause() override;                                        // 02
    // void DoFinish(bool a_arg1) override;                            // 03
    // void DoApplyDuckingAttenuation(std::uint16_t a_arg1) override;  // 04
    // void DoClearDucking() override;                                 // 05
    // void DoPrepare() override;                                      // 06

    #[inline(always)]
    pub fn get_form_editor_id_local(&self) -> *const c_char {
        self.form_editor_id.as_ptr()
    }

    #[inline(always)]
    pub fn set_form_editor_id_local(&mut self, editor_id: *const c_char) -> bool {
        self.form_editor_id = BSFixedString::new(editor_id);
        true
    }
}
