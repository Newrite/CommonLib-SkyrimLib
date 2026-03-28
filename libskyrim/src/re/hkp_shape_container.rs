#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_hkpShapeContainer;
use crate::offsets::offsets_vtable::VTABLE_hkpShapeContainer;
use crate::re::hkp_shape::hkpShapeKey;
use crate::re::{CFilter, hkpShape, hkpShapeBuffer};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::hkpShapeContainer::ReferencePolicy`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpShapeContainerReferencePolicy {
    kIgnore = 0,
    kIncrement = 1,
}

/// C++ `RE::hkpShapeContainer`
#[repr(C)]
pub struct hkpShapeContainer {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<hkpShapeContainer>() == 0x8);
const _: () = assert!(core::mem::offset_of!(hkpShapeContainer, vtable) == 0x00);

impl RttiType for hkpShapeContainer {
    const RTTI: VariantID = RTTI_hkpShapeContainer;
}

impl AsRef<hkpShapeContainer> for hkpShapeContainer {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<hkpShapeContainer> for hkpShapeContainer {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl hkpShapeContainer {
    pub const RTTI: VariantID = RTTI_hkpShapeContainer;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpShapeContainer;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    virtual_method! {
        pub const VFUNC_GET_NUM_CHILD_SHAPES: usize = 0x01;
        pub fn get_num_child_shapes(&self) -> i32
    }

    virtual_method! {
        pub const VFUNC_GET_FIRST_KEY: usize = 0x02;
        pub fn get_first_key(&self) -> hkpShapeKey
    }

    virtual_method! {
        pub const VFUNC_GET_NEXT_KEY: usize = 0x03;
        pub fn get_next_key(&self, old_key: hkpShapeKey) -> hkpShapeKey
    }

    virtual_method! {
        pub const VFUNC_GET_COLLISION_FILTER_INFO: usize = 0x04;
        pub fn get_collision_filter_info(&self, key: hkpShapeKey) -> CFilter
    }

    virtual_method! {
        pub const VFUNC_GET_CHILD_SHAPE: usize = 0x05;
        pub fn get_child_shape(&self, key: hkpShapeKey, buffer: &mut hkpShapeBuffer) -> *const hkpShape
    }

    virtual_method! {
        pub const VFUNC_IS_WELDING_ENABLED: usize = 0x06;
        pub fn is_welding_enabled(&self) -> bool
    }
}
