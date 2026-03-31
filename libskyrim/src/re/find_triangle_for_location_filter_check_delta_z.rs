#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::{
    RTTI_FindTriangleForLocationFilter, RTTI_FindTriangleForLocationFilterCheckDeltaZ,
};
use crate::offsets::offsets_vtable::{
    VTABLE_FindTriangleForLocationFilter, VTABLE_FindTriangleForLocationFilterCheckDeltaZ,
};
use crate::re::{BSNavmesh, NiPoint3};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::FindTriangleForLocationFilter`
#[repr(C)]
pub struct FindTriangleForLocationFilter {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<FindTriangleForLocationFilter>() == 0x08);

impl RttiType for FindTriangleForLocationFilter {
    const RTTI: VariantID = RTTI_FindTriangleForLocationFilter;
}

impl FindTriangleForLocationFilter {
    pub const RTTI: VariantID = RTTI_FindTriangleForLocationFilter;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FindTriangleForLocationFilter;

    crate::virtual_method! {
        pub const VFUNC_IS_VALID_TRI: usize = 0x01;
        pub fn is_valid_tri(
            &mut self,
            location: *mut NiPoint3,
            nav_mesh: *mut BSNavmesh,
            tri: u16,
            point_on_tri: *mut NiPoint3
        ) -> bool
    }
}

/// C++ `RE::FindTriangleForLocationFilterCheckDeltaZ`
#[repr(C)]
pub struct FindTriangleForLocationFilterCheckDeltaZ {
    pub base: FindTriangleForLocationFilter, // 00
    pub max_dist_above: f32,                 // 08
    pub max_dist_below: f32,                 // 0C
}

const _: () = assert!(core::mem::size_of::<FindTriangleForLocationFilterCheckDeltaZ>() == 0x10);
const _: () =
    assert!(core::mem::offset_of!(FindTriangleForLocationFilterCheckDeltaZ, base) == 0x00);
const _: () = assert!(
    core::mem::offset_of!(FindTriangleForLocationFilterCheckDeltaZ, max_dist_above) == 0x08
);
const _: () = assert!(
    core::mem::offset_of!(FindTriangleForLocationFilterCheckDeltaZ, max_dist_below) == 0x0C
);

impl RttiType for FindTriangleForLocationFilterCheckDeltaZ {
    const RTTI: VariantID = RTTI_FindTriangleForLocationFilterCheckDeltaZ;
}

inherit!(FindTriangleForLocationFilterCheckDeltaZ : FindTriangleForLocationFilter, base);

impl FindTriangleForLocationFilterCheckDeltaZ {
    pub const RTTI: VariantID = RTTI_FindTriangleForLocationFilterCheckDeltaZ;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FindTriangleForLocationFilterCheckDeltaZ;

    crate::relocation_func! {
        pub fn is_valid_tri_relocated(
            &mut self,
            location: *mut NiPoint3,
            nav_mesh: *mut BSNavmesh,
            tri: u16,
            point_on_tri: *mut NiPoint3,
        ) -> bool => RelocationID::new(88004, 90391)
    }
}
