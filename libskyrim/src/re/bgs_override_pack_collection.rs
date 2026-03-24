use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSOverridePackCollection;
use crate::offsets::offsets_vtable::VTABLE_BGSOverridePackCollection;
use crate::re::BGSListForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct BGSOverridePackCollection {
    pub base: BaseFormComponent,                             // 00
    pub spectator_override_pack_list: *mut BGSListForm,      // 08
    pub observe_corpse_override_pack_list: *mut BGSListForm, // 10
    pub guard_warn_override_pack_list: *mut BGSListForm,     // 18
    pub enter_combat_override_pack_list: *mut BGSListForm,   // 20
}

const _: () = assert!(core::mem::size_of::<BGSOverridePackCollection>() == 0x28);
const _: () =
    assert!(core::mem::offset_of!(BGSOverridePackCollection, spectator_override_pack_list) == 0x08);
const _: () = assert!(
    core::mem::offset_of!(BGSOverridePackCollection, observe_corpse_override_pack_list) == 0x10
);
const _: () = assert!(
    core::mem::offset_of!(BGSOverridePackCollection, guard_warn_override_pack_list) == 0x18
);
const _: () = assert!(
    core::mem::offset_of!(BGSOverridePackCollection, enter_combat_override_pack_list) == 0x20
);

impl RttiType for BGSOverridePackCollection {
    const RTTI: VariantID = RTTI_BGSOverridePackCollection;
}

inherit!(BGSOverridePackCollection : BaseFormComponent);

impl BGSOverridePackCollection {
    pub const RTTI: VariantID = RTTI_BGSOverridePackCollection;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSOverridePackCollection;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03
}
