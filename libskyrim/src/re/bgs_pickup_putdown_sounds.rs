use core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_BGSPickupPutdownSounds;
use crate::offsets::offsets_vtable::VTABLE_BGSPickupPutdownSounds;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

#[repr(C)]
pub struct BGSPickupPutdownSounds {
    pub base: BaseFormComponent, // 00
    pub pickup_sound: *mut BGSSoundDescriptorForm,  // 08 - YNAM
    pub putdown_sound: *mut BGSSoundDescriptorForm, // 10 - ZNAM
}

const _: () = assert!(core::mem::size_of::<BGSPickupPutdownSounds>() == 0x18);

impl RttiType for BGSPickupPutdownSounds {
    const RTTI: VariantID = RTTI_BGSPickupPutdownSounds;
}

inherit!(BGSPickupPutdownSounds : BaseFormComponent);

impl BGSPickupPutdownSounds {
    pub const RTTI: VariantID = RTTI_BGSPickupPutdownSounds;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSPickupPutdownSounds;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02 - { return; }
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}