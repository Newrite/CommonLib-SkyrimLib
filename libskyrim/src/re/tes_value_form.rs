use crate::offsets::offsets_rtti::RTTI_TESValueForm;
use crate::offsets::offsets_vtable::VTABLE_TESValueForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::magic_item::MagicItem;
use crate::re::tes_form::TESForm;
use crate::relocation::{RttiType, VariantID, skyrim_cast};
use core_util::inherit;

/// C++ `RE::TESValueForm`
#[repr(C)]
pub struct TESValueForm {
    pub base: BaseFormComponent, // 00
    pub value: i32,              // 08
    pub pad0c: u32,              // 0C
}

const _: () = assert!(core::mem::size_of::<TESValueForm>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TESValueForm, value) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESValueForm, pad0c) == 0x0C);

impl RttiType for TESValueForm {
    const RTTI: VariantID = RTTI_TESValueForm;
}

impl TESValueForm {
    pub const RTTI: VariantID = RTTI_TESValueForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESValueForm;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_INITIALIZE_DATA_COMPONENT: usize = 0x01;
        pub fn initialize_data_component(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_CLEAR_DATA_COMPONENT: usize = 0x02;
        pub fn clear_data_component(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_COPY_COMPONENT: usize = 0x03;
        pub fn copy_component(&mut self, rhs: *mut BaseFormComponent)
    }

    #[inline(always)]
    pub const fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_form_value(form: *const TESForm) -> i32 {
        if form.is_null() {
            return -1;
        }

        unsafe {
            let value_form = skyrim_cast::<TESForm, TESValueForm>(form as *mut TESForm);
            if !value_form.is_null() {
                return (*value_form).value;
            }

            let magic_item = skyrim_cast::<TESForm, MagicItem>(form as *mut TESForm);
            if !magic_item.is_null() {
                return (*magic_item).calculate_magicka_cost(core::ptr::null_mut()) as i32;
            }
        }
        -1
    }
}

pub trait TESValueFormExt {
    fn dtor(&mut self);
    fn initialize_data_component(&mut self);
    fn clear_data_component(&mut self);
    fn copy_component(&mut self, rhs: *mut BaseFormComponent);
    fn get_value(&self) -> i32;
}

impl<T: AsRef<TESValueForm> + AsMut<TESValueForm>> TESValueFormExt for T {
    fn dtor(&mut self) {
        TESValueForm::dtor(self.as_mut())
    }

    fn initialize_data_component(&mut self) {
        TESValueForm::initialize_data_component(self.as_mut())
    }

    fn clear_data_component(&mut self) {
        TESValueForm::clear_data_component(self.as_mut())
    }

    fn copy_component(&mut self, rhs: *mut BaseFormComponent) {
        TESValueForm::copy_component(self.as_mut(), rhs)
    }

    fn get_value(&self) -> i32 {
        self.as_ref().get_value()
    }
}

inherit!(TESValueForm : BaseFormComponent);
