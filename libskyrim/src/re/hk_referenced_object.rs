#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkReferencedObject;
use crate::offsets::offsets_vtable::VTABLE_hkReferencedObject;
use crate::re::{hkBaseObject, hkClass, hkRefPtr, hkStatisticsCollector};
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::{relocation_func, virtual_method};

/// C++ `RE::hkReferencedObject::LockMode`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkReferencedObjectLockMode {
    None = 0,
    Auto = 1,
    Manual = 2,
}

/// C++ `RE::hkReferencedObject`
#[repr(C)]
pub struct hkReferencedObject {
    pub base: hkBaseObject,      // 00
    pub mem_size_and_flags: u16, // 08
    pub reference_count: i16,    // 0A
    pub pad0c: u32,              // 0C
}

const _: () = assert!(core::mem::size_of::<hkReferencedObject>() == 0x10);
const _: () = assert!(core::mem::offset_of!(hkReferencedObject, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkReferencedObject, mem_size_and_flags) == 0x08);
const _: () = assert!(core::mem::offset_of!(hkReferencedObject, reference_count) == 0x0A);
const _: () = assert!(core::mem::offset_of!(hkReferencedObject, pad0c) == 0x0C);

impl RttiType for hkReferencedObject {
    const RTTI: VariantID = RTTI_hkReferencedObject;
}

inherit!(hkReferencedObject : hkBaseObject, base);

impl AsRef<hkReferencedObject> for hkReferencedObject {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<hkReferencedObject> for hkReferencedObject {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl hkReferencedObject {
    pub const RTTI: VariantID = RTTI_hkReferencedObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkReferencedObject;
    pub const MEM_SIZE: i32 = 0x7FFF;

    /// Rust-side helper for header-level `make_hkref<hkReferencedObject>()`.
    #[inline(always)]
    pub fn make_ref() -> Option<hkRefPtr<Self>> {
        unsafe {
            hkRefPtr::<Self>::try_construct_with(|out: *mut hkRefPtr<Self>| {
                crate::ffi::commonlib_make_hkref_hk_referenced_object(out.cast())
            })
        }
    }

    // override (hkBaseObject)
    // ~hkReferencedObject() override = default;  // 00

    virtual_method! {
        pub const VFUNC_GET_CLASS_TYPE: usize = 0x01;
        pub fn get_class_type() -> *const hkClass
    }

    virtual_method! {
        pub const VFUNC_CALC_CONTENT_STATISTICS: usize = 0x02;
        pub fn calc_content_statistics(collector: *mut hkStatisticsCollector, class: *const hkClass)
    }

    relocation_func! {
        pub fn add_reference(&self) => RelocationID::new(56606, 57010)
    }

    #[inline(always)]
    pub fn get_allocated_size(&self) -> i32 {
        (self.mem_size_and_flags & Self::MEM_SIZE as u16) as i32
    }

    #[inline(always)]
    pub fn get_reference_count(&self) -> i32 {
        self.reference_count as i32
    }

    relocation_func! {
        pub fn remove_reference(&self) => RelocationID::new(56607, 57011)
    }
}

pub trait hkReferencedObjectExt {
    fn get_class_type(&self) -> *const hkClass;
    fn calc_content_statistics(&self, collector: *mut hkStatisticsCollector, class: *const hkClass);
    fn add_reference(&self);
    fn get_allocated_size(&self) -> i32;
    fn get_reference_count(&self) -> i32;
    fn remove_reference(&self);
}

impl<T: AsRef<hkReferencedObject> + AsMut<hkReferencedObject>> hkReferencedObjectExt for T {
    #[inline(always)]
    fn get_class_type(&self) -> *const hkClass {
        hkReferencedObject::get_class_type(self.as_ref())
    }

    #[inline(always)]
    fn calc_content_statistics(
        &self,
        collector: *mut hkStatisticsCollector,
        class: *const hkClass,
    ) {
        hkReferencedObject::calc_content_statistics(self.as_ref(), collector, class)
    }

    #[inline(always)]
    fn add_reference(&self) {
        hkReferencedObject::add_reference(self.as_ref())
    }

    #[inline(always)]
    fn get_allocated_size(&self) -> i32 {
        hkReferencedObject::get_allocated_size(self.as_ref())
    }

    #[inline(always)]
    fn get_reference_count(&self) -> i32 {
        hkReferencedObject::get_reference_count(self.as_ref())
    }

    #[inline(always)]
    fn remove_reference(&self) {
        hkReferencedObject::remove_reference(self.as_ref())
    }
}
