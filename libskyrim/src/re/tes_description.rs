use crate::offsets::offsets_rtti::RTTI_TESDescription;
use crate::offsets::offsets_vtable::VTABLE_TESDescription;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bgs_localized_string_dl::BGSLocalizedStringDL;
use crate::re::bs_string::BSString;
use crate::re::tes_form::TESForm;
use crate::relocation::{RelocationID, RttiType, VariantID};
use core_util::inherit;

#[repr(C)]
pub struct TESDescription {
    pub base: BaseFormComponent,                // 0x00
    pub file_offset: u32,                       // 0x08
    pub description_text: BGSLocalizedStringDL, // 0x0C
}

const _: () = assert!(core::mem::size_of::<TESDescription>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TESDescription, file_offset) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESDescription, description_text) == 0x0C);

impl RttiType for TESDescription {
    const RTTI: VariantID = RTTI_TESDescription;
}

inherit!(TESDescription : BaseFormComponent);

impl TESDescription {
    pub const RTTI: VariantID = RTTI_TESDescription;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESDescription;

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

    // RELOCATION_ID SE: 14399, AE: 14552
    crate::relocation_func! {
        pub fn get_description(this: &TESDescription, out: &mut BSString, parent: *mut TESForm, field_type: u32) => RelocationID::new(14399, 14552)
    }
}

pub trait TESDescriptionExt {
    fn dtor(&mut self);
    fn initialize_data_component(&mut self);
    fn clear_data_component(&mut self);
    fn copy_component(&mut self, rhs: *mut BaseFormComponent);
    fn get_description(&self, out: &mut BSString, parent: *mut TESForm, field_type: u32);
}

impl<T: AsRef<TESDescription> + AsMut<TESDescription>> TESDescriptionExt for T {
    fn dtor(&mut self) {
        TESDescription::dtor(self.as_mut())
    }

    fn initialize_data_component(&mut self) {
        TESDescription::initialize_data_component(self.as_mut())
    }

    fn clear_data_component(&mut self) {
        TESDescription::clear_data_component(self.as_mut())
    }

    fn copy_component(&mut self, rhs: *mut BaseFormComponent) {
        TESDescription::copy_component(self.as_mut(), rhs)
    }

    fn get_description(&self, out: &mut BSString, parent: *mut TESForm, field_type: u32) {
        TESDescription::get_description(self.as_ref(), out, parent, field_type)
    }
}
