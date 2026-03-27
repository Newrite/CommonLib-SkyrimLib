use crate::offsets::offsets_rtti::RTTI_BSScript__IVMObjectBindInterface;
use crate::offsets::offsets_vtable::VTABLE_BSScript__IVMObjectBindInterface;
use crate::re::{BSFixedString, BSTSmartPointer, Object, VMHandle};
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct IVMObjectBindInterface {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<IVMObjectBindInterface>() == 0x8);

impl RttiType for IVMObjectBindInterface {
    const RTTI: VariantID = RTTI_BSScript__IVMObjectBindInterface;
}

impl IVMObjectBindInterface {
    pub const RTTI: VariantID = RTTI_BSScript__IVMObjectBindInterface;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSScript__IVMObjectBindInterface;

    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_GET_BOUND_HANDLE: usize = 0x01; pub fn get_bound_handle(obj_ptr: &BSTSmartPointer<Object>) -> VMHandle }
    crate::virtual_method! { pub const VFUNC_TYPE_CAN_BE_BOUND: usize = 0x02; pub fn type_can_be_bound(class_name: &BSFixedString, handle: VMHandle) -> bool }
    crate::virtual_method! { pub const VFUNC_BIND_OBJECT: usize = 0x03; pub fn bind_object(obj_ptr: &mut BSTSmartPointer<Object>, handle: VMHandle, conditional: bool) }
    crate::virtual_method! { pub const VFUNC_HANDLE_LOADED_BINDING: usize = 0x04; pub fn handle_loaded_binding(obj_ptr: &mut BSTSmartPointer<Object>, handle: VMHandle, conditional: bool) }
    crate::virtual_method! { pub const VFUNC_REMOVE_ALL_BOUND_OBJECTS: usize = 0x05; pub fn remove_all_bound_objects(handle: VMHandle) }
    crate::virtual_method! { pub const VFUNC_REMOVE_ALL_DISK_LOADED_BOUND_OBJECTS: usize = 0x06; pub fn remove_all_disk_loaded_bound_objects(handle: VMHandle) }
    crate::virtual_method! { pub const VFUNC_HANDLE_COBJECT_DELETION: usize = 0x07; pub fn handle_cobject_deletion(handle: VMHandle) }
    crate::virtual_method! { pub const VFUNC_UNBIND_OBJECT: usize = 0x08; pub fn unbind_object(obj_ptr: &BSTSmartPointer<Object>) }
    crate::virtual_method! { pub const VFUNC_CREATE_OBJECT_WITH_PROPERTIES: usize = 0x09; pub fn create_object_with_properties(class_name: &BSFixedString, num_properties: u32, obj_ptr: &mut BSTSmartPointer<Object>) -> bool }
    crate::virtual_method! { pub const VFUNC_INIT_OBJECT_PROPERTIES: usize = 0x0A; pub fn init_object_properties(obj_ptr: &mut BSTSmartPointer<Object>, property: *mut core::ffi::c_void, arg3: bool) -> bool }
}

pub trait IVMObjectBindInterfaceExt:
    AsRef<IVMObjectBindInterface> + AsMut<IVMObjectBindInterface>
{
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn get_bound_handle(&self, obj_ptr: &BSTSmartPointer<Object>) -> VMHandle {
        self.as_ref().get_bound_handle(obj_ptr)
    }

    #[inline(always)]
    fn type_can_be_bound(&mut self, class_name: &BSFixedString, handle: VMHandle) -> bool {
        self.as_mut().type_can_be_bound(class_name, handle)
    }

    #[inline(always)]
    fn bind_object(
        &mut self,
        obj_ptr: &mut BSTSmartPointer<Object>,
        handle: VMHandle,
        conditional: bool,
    ) {
        self.as_mut().bind_object(obj_ptr, handle, conditional)
    }

    #[inline(always)]
    fn handle_loaded_binding(
        &mut self,
        obj_ptr: &mut BSTSmartPointer<Object>,
        handle: VMHandle,
        conditional: bool,
    ) {
        self.as_mut()
            .handle_loaded_binding(obj_ptr, handle, conditional)
    }

    #[inline(always)]
    fn remove_all_bound_objects(&mut self, handle: VMHandle) {
        self.as_mut().remove_all_bound_objects(handle)
    }

    #[inline(always)]
    fn remove_all_disk_loaded_bound_objects(&mut self, handle: VMHandle) {
        self.as_mut().remove_all_disk_loaded_bound_objects(handle)
    }

    #[inline(always)]
    fn handle_cobject_deletion(&mut self, handle: VMHandle) {
        self.as_mut().handle_cobject_deletion(handle)
    }

    #[inline(always)]
    fn unbind_object(&mut self, obj_ptr: &BSTSmartPointer<Object>) {
        self.as_mut().unbind_object(obj_ptr)
    }

    #[inline(always)]
    fn create_object_with_properties(
        &mut self,
        class_name: &BSFixedString,
        num_properties: u32,
        obj_ptr: &mut BSTSmartPointer<Object>,
    ) -> bool {
        self.as_mut()
            .create_object_with_properties(class_name, num_properties, obj_ptr)
    }

    #[inline(always)]
    fn init_object_properties(
        &mut self,
        obj_ptr: &mut BSTSmartPointer<Object>,
        property: *mut core::ffi::c_void,
        arg3: bool,
    ) -> bool {
        self.as_mut()
            .init_object_properties(obj_ptr, property, arg3)
    }
}

impl<T> IVMObjectBindInterfaceExt for T where
    T: AsRef<IVMObjectBindInterface> + AsMut<IVMObjectBindInterface>
{
}
