use crate::offsets::offsets_rtti::RTTI_TESValueForm;
use crate::offsets::offsets_vtable::VTABLE_TESValueForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::tes_form::TESForm;
use crate::relocation::{skyrim_cast, VariantID, RttiType};
use core_util::inherit;

/// C++ `RE::TESValueForm`
#[repr(C)]
pub struct TESValueForm {
    pub base: BaseFormComponent, // 00
    pub value: i32,              // 08
    pub pad0c: u32,              // 0C
}

const _: () = assert!(core::mem::size_of::<TESValueForm>() == 0x10);

impl RttiType for TESValueForm {
    const RTTI: VariantID = RTTI_TESValueForm;
}

impl TESValueForm {
    pub const RTTI: VariantID = RTTI_TESValueForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESValueForm;

    pub fn get_form_value(form: *const TESForm) -> i32 {
        if form.is_null() {
            return -1;
        }

        unsafe {
            let value_form = skyrim_cast::<TESForm, TESValueForm>(form as *mut TESForm);
            if !value_form.is_null() {
                return (*value_form).value;
            }
        }

        // TODO: MagicItem cost calculation fallback if needed
        -1
    }
}

inherit!(TESValueForm : BaseFormComponent);
