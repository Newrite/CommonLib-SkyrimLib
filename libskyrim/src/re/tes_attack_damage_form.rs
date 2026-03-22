use core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_TESAttackDamageForm;
use crate::offsets::offsets_vtable::VTABLE_TESAttackDamageForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

#[repr(C)]
pub struct TESAttackDamageForm {
    pub base: BaseFormComponent, // 00
    pub attack_damage: u16,      // 08
    pub pad0a: u16,              // 0A
    pub pad0c: u32,              // 0C
}

const _: () = assert!(core::mem::size_of::<TESAttackDamageForm>() == 0x10);

impl RttiType for TESAttackDamageForm {
    const RTTI: VariantID = RTTI_TESAttackDamageForm;
}

inherit!(TESAttackDamageForm : BaseFormComponent);

impl TESAttackDamageForm {
    pub const RTTI: VariantID = RTTI_TESAttackDamageForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESAttackDamageForm;

    virtual_method! {
        pub const GET_ATTACK_DAMAGE: usize = 0x04;
        pub fn get_attack_damage(this: &TESAttackDamageForm) -> u16
    }
}