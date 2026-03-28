use crate::offsets::offsets_rtti::RTTI_BGSMessageIcon;
use crate::offsets::offsets_vtable::VTABLE_BGSMessageIcon;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::tes_icon::TESIcon;
use crate::relocation::{RttiType, VariantID};
use core_util::inherit;

#[repr(C)]
pub struct BGSMessageIcon {
    pub base: BaseFormComponent, // 00
    pub icon: TESIcon,           // 08
}

const _: () = assert!(core::mem::size_of::<BGSMessageIcon>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSMessageIcon, icon) == 0x08);

impl RttiType for BGSMessageIcon {
    const RTTI: VariantID = RTTI_BGSMessageIcon;
}

inherit!(BGSMessageIcon : BaseFormComponent);

impl BGSMessageIcon {
    pub const RTTI: VariantID = RTTI_BGSMessageIcon;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSMessageIcon;

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
}

pub trait BGSMessageIconExt {
    fn dtor(&mut self);
    fn initialize_data_component(&mut self);
    fn clear_data_component(&mut self);
    fn copy_component(&mut self, rhs: *mut BaseFormComponent);
}

impl<T: AsRef<BGSMessageIcon> + AsMut<BGSMessageIcon>> BGSMessageIconExt for T {
    fn dtor(&mut self) {
        BGSMessageIcon::dtor(self.as_mut())
    }

    fn initialize_data_component(&mut self) {
        BGSMessageIcon::initialize_data_component(self.as_mut())
    }

    fn clear_data_component(&mut self) {
        BGSMessageIcon::clear_data_component(self.as_mut())
    }

    fn copy_component(&mut self, rhs: *mut BaseFormComponent) {
        BGSMessageIcon::copy_component(self.as_mut(), rhs)
    }
}
