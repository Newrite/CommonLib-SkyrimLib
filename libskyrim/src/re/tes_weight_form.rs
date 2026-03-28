use crate::offsets::offsets_rtti::RTTI_TESWeightForm;
use crate::offsets::offsets_vtable::VTABLE_TESWeightForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};
use core_util::inherit;

/// C++ `RE::TESWeightForm`
#[repr(C)]
pub struct TESWeightForm {
    pub base: BaseFormComponent, // 00
    pub weight: f32,             // 08
    pub pad0c: u32,              // 0C
}

const _: () = assert!(core::mem::size_of::<TESWeightForm>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TESWeightForm, weight) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESWeightForm, pad0c) == 0x0C);

impl RttiType for TESWeightForm {
    const RTTI: VariantID = RTTI_TESWeightForm;
}

impl TESWeightForm {
    pub const RTTI: VariantID = RTTI_TESWeightForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESWeightForm;

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
    pub const fn get_weight(&self) -> f32 {
        self.weight
    }
}

pub trait TESWeightFormExt {
    fn dtor(&mut self);
    fn initialize_data_component(&mut self);
    fn clear_data_component(&mut self);
    fn copy_component(&mut self, rhs: *mut BaseFormComponent);
    fn get_weight(&self) -> f32;
}

impl<T: AsRef<TESWeightForm> + AsMut<TESWeightForm>> TESWeightFormExt for T {
    fn dtor(&mut self) {
        TESWeightForm::dtor(self.as_mut())
    }

    fn initialize_data_component(&mut self) {
        TESWeightForm::initialize_data_component(self.as_mut())
    }

    fn clear_data_component(&mut self) {
        TESWeightForm::clear_data_component(self.as_mut())
    }

    fn copy_component(&mut self, rhs: *mut BaseFormComponent) {
        TESWeightForm::copy_component(self.as_mut(), rhs)
    }

    fn get_weight(&self) -> f32 {
        TESWeightForm::get_weight(self.as_ref())
    }
}

inherit!(TESWeightForm : BaseFormComponent);
