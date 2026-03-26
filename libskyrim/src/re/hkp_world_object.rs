#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkpWorldObject;
use crate::offsets::offsets_vtable::VTABLE_hkpWorldObject;
use crate::re::hkp_collidable::hkpCollidable as HavokCollidable;
use crate::re::{
    hkArray, hkMotionState, hkMultiThreadCheck, hkReferencedObject, hkStringPtr,
    hkpLinkedCollidable, hkpProperty, hkpPropertyValue, hkpShape, hkpWorld,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::hkWorldOperation::Result`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkWorldOperationResult {
    Postponed = 0,
    Done = 1,
}

/// C++ `RE::hkpWorldObject::MultiThreadingChecks`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpWorldObjectMultiThreadingChecks {
    Enable = 0,
    Ignore = 1,
}

/// C++ `RE::hkpWorldObject::BroadPhaseType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpWorldObjectBroadPhaseType {
    Invalid = 0,
    Entity = 1,
    Phantom = 2,
    PhaseBorder = 3,
    Total = 4,
}

/// C++ `RE::hkpWorldObject`
#[repr(C)]
pub struct hkpWorldObject {
    pub base: hkReferencedObject,               // 00
    pub world: *mut hkpWorld,                   // 10
    pub user_data: u64,                         // 18
    pub collidable: hkpLinkedCollidable,        // 20
    pub multi_thread_check: hkMultiThreadCheck, // A0
    pub padac: u32,                             // AC
    pub name: hkStringPtr,                      // B0
    pub properties: hkArray<hkpProperty>,       // B8
    pub tree_data: *mut core::ffi::c_void,      // C8
}

const _: () = assert!(core::mem::size_of::<hkpWorldObject>() == 0xD0);
const _: () = assert!(core::mem::offset_of!(hkpWorldObject, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpWorldObject, world) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkpWorldObject, collidable) == 0x20);
const _: () = assert!(core::mem::offset_of!(hkpWorldObject, multi_thread_check) == 0xA0);
const _: () = assert!(core::mem::offset_of!(hkpWorldObject, name) == 0xB0);
const _: () = assert!(core::mem::offset_of!(hkpWorldObject, properties) == 0xB8);
const _: () = assert!(core::mem::offset_of!(hkpWorldObject, tree_data) == 0xC8);

impl RttiType for hkpWorldObject {
    const RTTI: VariantID = RTTI_hkpWorldObject;
}

inherit!(hkpWorldObject : hkReferencedObject, base);

impl hkpWorldObject {
    pub const RTTI: VariantID = RTTI_hkpWorldObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpWorldObject;

    crate::virtual_method! {
        pub const VFUNC_SET_SHAPE: usize = 0x03;
        pub fn set_shape(&mut self, shape: *const hkpShape) -> hkWorldOperationResult
    }

    crate::virtual_method! {
        pub const VFUNC_UPDATE_SHAPE: usize = 0x04;
        pub fn update_shape(
            &mut self,
            shape_modifier: *mut core::ffi::c_void
        ) -> hkWorldOperationResult
    }

    crate::virtual_method! {
        pub const VFUNC_GET_MOTION_STATE: usize = 0x05;
        pub fn get_motion_state(&mut self) -> *mut hkMotionState
    }

    #[inline(always)]
    pub fn get_collidable(&self) -> *const HavokCollidable {
        &self.collidable.base as *const _
    }

    #[inline(always)]
    pub fn get_collidable_rw(&mut self) -> *mut HavokCollidable {
        &mut self.collidable.base as *mut _
    }

    #[inline(always)]
    pub fn get_property(&self, key: u32) -> Option<hkpProperty> {
        if self.properties.base.data.is_null() || self.properties.base.size <= 0 {
            return None;
        }
        let props = unsafe {
            core::slice::from_raw_parts(
                self.properties.base.data,
                self.properties.base.size as usize,
            )
        };
        props.iter().copied().find(|prop| prop.key == key)
    }

    #[inline(always)]
    pub fn get_shape(&self) -> *const hkpShape {
        self.collidable.base.base.shape
    }

    #[inline(always)]
    pub fn has_property(&self, key: u32) -> bool {
        self.get_property(key).is_some()
    }

    crate::relocation_func! {
        pub fn remove_property(&mut self, key: u32) => RelocationID::new(75976, 77802)
    }

    crate::relocation_func! {
        pub fn set_property(&mut self, key: u32, value: hkpPropertyValue) => RelocationID::new(60628, 61479)
    }
}

pub trait hkpWorldObjectExt {
    fn set_shape(&mut self, shape: *const hkpShape) -> hkWorldOperationResult;
    fn update_shape(&mut self, shape_modifier: *mut core::ffi::c_void) -> hkWorldOperationResult;
    fn get_motion_state(&mut self) -> *mut hkMotionState;
    fn get_collidable(&self) -> *const HavokCollidable;
    fn get_collidable_rw(&mut self) -> *mut HavokCollidable;
    fn get_property(&self, key: u32) -> Option<hkpProperty>;
    fn get_shape(&self) -> *const hkpShape;
    fn has_property(&self, key: u32) -> bool;
    fn remove_property(&mut self, key: u32);
    fn set_property(&mut self, key: u32, value: hkpPropertyValue);
}

impl<T: AsRef<hkpWorldObject> + AsMut<hkpWorldObject>> hkpWorldObjectExt for T {
    #[inline(always)]
    fn set_shape(&mut self, shape: *const hkpShape) -> hkWorldOperationResult {
        hkpWorldObject::set_shape(self.as_mut(), shape)
    }

    #[inline(always)]
    fn update_shape(&mut self, shape_modifier: *mut core::ffi::c_void) -> hkWorldOperationResult {
        hkpWorldObject::update_shape(self.as_mut(), shape_modifier)
    }

    #[inline(always)]
    fn get_motion_state(&mut self) -> *mut hkMotionState {
        hkpWorldObject::get_motion_state(self.as_mut())
    }

    #[inline(always)]
    fn get_collidable(&self) -> *const HavokCollidable {
        hkpWorldObject::get_collidable(self.as_ref())
    }

    #[inline(always)]
    fn get_collidable_rw(&mut self) -> *mut HavokCollidable {
        hkpWorldObject::get_collidable_rw(self.as_mut())
    }

    #[inline(always)]
    fn get_property(&self, key: u32) -> Option<hkpProperty> {
        hkpWorldObject::get_property(self.as_ref(), key)
    }

    #[inline(always)]
    fn get_shape(&self) -> *const hkpShape {
        hkpWorldObject::get_shape(self.as_ref())
    }

    #[inline(always)]
    fn has_property(&self, key: u32) -> bool {
        hkpWorldObject::has_property(self.as_ref(), key)
    }

    #[inline(always)]
    fn remove_property(&mut self, key: u32) {
        hkpWorldObject::remove_property(self.as_mut(), key)
    }

    #[inline(always)]
    fn set_property(&mut self, key: u32, value: hkpPropertyValue) {
        hkpWorldObject::set_property(self.as_mut(), key, value)
    }
}
