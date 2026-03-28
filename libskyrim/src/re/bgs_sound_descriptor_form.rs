use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSSoundDescriptorForm;
use crate::offsets::offsets_vtable::VTABLE_BGSSoundDescriptorForm;
use crate::re::bgs_sound_descriptor::BGSSoundDescriptor;
use crate::re::bsi_sound_descriptor::BSISoundDescriptor;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_form::TESForm;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSSoundDescriptorFormRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

unsafe impl bytemuck::Zeroable for BGSSoundDescriptorFormRecordFlags {}

/// C++ `RE::BGSSoundDescriptorForm`
#[repr(C)]
pub struct BGSSoundDescriptorForm {
    pub base: TESForm,                                  // 00
    pub sound_descriptor_interface: BSISoundDescriptor, // 20
    pub sound_descriptor: *mut BGSSoundDescriptor,      // 28
}

const _: () = assert!(core::mem::size_of::<BGSSoundDescriptorForm>() == 0x30);
const _: () =
    assert!(core::mem::offset_of!(BGSSoundDescriptorForm, sound_descriptor_interface) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSSoundDescriptorForm, sound_descriptor) == 0x28);

impl RttiType for BGSSoundDescriptorForm {
    const RTTI: VariantID = RTTI_BGSSoundDescriptorForm;
}

impl FormCastable for BGSSoundDescriptorForm {
    const TARGET_FORM_TYPE: FormType = FormType::SoundRecord;
}

inherit!(BGSSoundDescriptorForm : TESForm);
inherit!(BGSSoundDescriptorForm => BSISoundDescriptor, sound_descriptor_interface);

impl BGSSoundDescriptorForm {
    pub const RTTI: VariantID = RTTI_BGSSoundDescriptorForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSSoundDescriptorForm;
    pub const FORMTYPE: FormType = FormType::SoundRecord;

    // override (TESForm)
    // void ClearData() override;                         // 05
    // bool Load(TESFile* a_mod) override;                // 06
    // void InitItemImpl() override;                      // 13
    // bool SetFormEditorID(const char* a_str) override;  // 33

    virtual_method! {
        pub const GET_DESCRIPTOR_TYPE: usize = 0x3B;
        pub fn get_descriptor_type() -> u32
    }

    #[inline(always)]
    pub fn get_sound_descriptor_ref(&self) -> Option<&BGSSoundDescriptor> {
        unsafe { self.sound_descriptor.as_ref() }
    }
}
