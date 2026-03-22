use crate::virtual_method;
use crate::relocation::{VariantID, RttiType};
use crate::offsets::offsets_rtti::RTTI_BaseFormComponent;
use crate::offsets::offsets_vtable::VTABLE_BaseFormComponent;

#[repr(C)]
pub struct BaseFormComponent {
    pub vtable: *const usize,
}

const _: () = assert!(core::mem::size_of::<BaseFormComponent>() == 0x8);

impl RttiType for BaseFormComponent {
    const RTTI: VariantID = RTTI_BaseFormComponent;
}

impl BaseFormComponent {
    pub const RTTI: VariantID = RTTI_BaseFormComponent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BaseFormComponent;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_INITIALIZE_DATA_COMPONENT: usize = 0x01;
        pub fn initialize_data_component()
    }

    virtual_method! {
        pub const VFUNC_CLEAR_DATA_COMPONENT: usize = 0x02;
        pub fn clear_data_component()
    }

    virtual_method! {
        pub const VFUNC_COPY_COMPONENT: usize = 0x03;
        pub fn copy_component(rhs: *mut BaseFormComponent)
    }
}

pub trait BaseFormComponentExt {
    fn dtor(&mut self);
    fn initialize_data_component(&mut self);
    fn clear_data_component(&mut self);
    fn copy_component(&mut self, rhs: *mut BaseFormComponent);
}

impl<T: AsRef<BaseFormComponent> + AsMut<BaseFormComponent>> BaseFormComponentExt for T {
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    fn initialize_data_component(&mut self) {
        self.as_mut().initialize_data_component()
    }

    fn clear_data_component(&mut self) {
        self.as_mut().clear_data_component()
    }

    fn copy_component(&mut self, rhs: *mut BaseFormComponent) {
        self.as_mut().copy_component(rhs)
    }
}
