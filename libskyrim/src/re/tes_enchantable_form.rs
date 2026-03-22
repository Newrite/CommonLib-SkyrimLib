use core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_TESEnchantableForm;
use crate::offsets::offsets_vtable::VTABLE_TESEnchantableForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::magic_system::CastingType;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

#[repr(C)]
pub struct TESEnchantableForm {
    pub base: BaseFormComponent, // 00
    pub form_enchanting: *mut core::ffi::c_void, // 08 - EITM (EnchantmentItem*)
    pub casting_type: u16, // 10
    pub amount_of_enchantment: u16, // 12 - EAMT
    pub pad14: u32, // 14
}

const _: () = assert!(core::mem::size_of::<TESEnchantableForm>() == 0x18);

impl RttiType for TESEnchantableForm {
    const RTTI: VariantID = RTTI_TESEnchantableForm;
}

inherit!(TESEnchantableForm : BaseFormComponent);

impl TESEnchantableForm {
    pub const RTTI: VariantID = RTTI_TESEnchantableForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESEnchantableForm;

    virtual_method! {
        pub const GET_CASTING_TYPE: usize = 0x04;
        pub fn get_casting_type(this: &TESEnchantableForm) -> CastingType
    }
}