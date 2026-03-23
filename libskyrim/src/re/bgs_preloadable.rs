use crate::offsets::offsets_rtti::RTTI_BGSPreloadable;
use crate::offsets::offsets_vtable::VTABLE_BGSPreloadable;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

#[repr(C)]
pub struct BGSPreloadable {
    pub base: BaseFormComponent, // 00
}

const _: () = assert!(core::mem::size_of::<BGSPreloadable>() == 0x8);

impl RttiType for BGSPreloadable {
    const RTTI: VariantID = RTTI_BGSPreloadable;
}

inherit!(BGSPreloadable : BaseFormComponent);

impl BGSPreloadable {
    pub const RTTI: VariantID = RTTI_BGSPreloadable;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSPreloadable;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03

    virtual_method! {
        pub const UNK_04: usize = 0x04;
        pub fn unk_04()
    }
}

pub trait BGSPreloadableExt {
    fn unk_04(&self);
}

impl<T: AsRef<BGSPreloadable>> BGSPreloadableExt for T {
    fn unk_04(&self) {
        self.as_ref().unk_04()
    }
}
