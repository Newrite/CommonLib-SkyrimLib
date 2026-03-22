use crate::relocation::VariantID;
use crate::re::form_type::FormType;
use crate::re::tes_form::TESForm;
use crate::virtual_method;

/// C++ `RE::IFormFactory`
#[repr(C)]
pub struct IFormFactory {
    pub vtable: *const VariantID,
}

const _: () = assert!(core::mem::size_of::<IFormFactory>() == 0x08);

impl IFormFactory {
    virtual_method! {
        pub const VFUNC_CREATE_IMPL: usize = 0x01;
        pub fn create_impl() -> *mut TESForm
    }

    virtual_method! {
        pub const VFUNC_GET_FORM_NAME: usize = 0x02;
        pub fn get_form_name() -> *const core::ffi::c_char
    }

    virtual_method! {
        pub const VFUNC_GET_FORM_TYPE: usize = 0x03;
        pub fn get_form_type() -> FormType
    }

    virtual_method! {
        pub const VFUNC_GET_OBJECT_NAME: usize = 0x04;
        pub fn get_object_name() -> *const core::ffi::c_char
    }

    // OBJECT_TYPE and OBJECT_CATEGORY_TYPE are omitted for now as they are simple enums
    // but we can add them if needed. For now let's focus on the major ones.

    crate::relocation_variable! {
        pub fn form_factories() -> *mut *mut IFormFactory => VariantID::new(514355, 400508, 0), is_ptr
    }

    crate::relocation_variable! {
        pub fn form_factories_initialized() -> *mut bool => VariantID::new(514349, 400503, 0), is_ptr
    }

    pub fn get_form_factory_by_type(form_type: FormType) -> *mut IFormFactory {
        let initialized = Self::form_factories_initialized();
        let factories = Self::form_factories();
        
        unsafe {
            if !initialized.is_null() && *initialized && !factories.is_null() {
                let index = form_type as usize;
                // Bound check assuming standard number of Skyrim form types
                if index < 138 {
                    let factory_ptr = factories.add(index);
                    return *factory_ptr;
                }
            }
        }
        core::ptr::null_mut()
    }
}

impl AsRef<IFormFactory> for IFormFactory {
    #[inline(always)]
    fn as_ref(&self) -> &Self { self }
}

pub trait IFormFactoryExt {
    fn create_impl(&self) -> *mut TESForm;
    fn get_form_name(&self) -> *const core::ffi::c_char;
    fn get_form_type(&self) -> FormType;
    fn get_object_name(&self) -> *const core::ffi::c_char;
}

impl<T: AsRef<IFormFactory>> IFormFactoryExt for T {
    fn create_impl(&self) -> *mut TESForm {
        IFormFactory::create_impl(self.as_ref())
    }

    fn get_form_name(&self) -> *const core::ffi::c_char {
        IFormFactory::get_form_name(self.as_ref())
    }

    fn get_form_type(&self) -> FormType {
        IFormFactory::get_form_type(self.as_ref())
    }

    fn get_object_name(&self) -> *const core::ffi::c_char {
        IFormFactory::get_object_name(self.as_ref())
    }
}
