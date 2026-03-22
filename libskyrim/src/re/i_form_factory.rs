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
