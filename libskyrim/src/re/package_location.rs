use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_PackageLocation;
use crate::offsets::offsets_vtable::VTABLE_PackageLocation;
use crate::re::{
    AIWorldLocationContext, IAIWorldLocation, IAIWorldLocationHandle, ObjectRefHandle, TESForm,
    TESObjectREFR,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::PackageLocation::Type`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageLocationType {
    None = -1,
    NearReference = 0,
    InCell = 1,
    NearPackageStartLocation = 2,
    NearEditorLocation = 3,
    ObjectID = 4,
    ObjectType = 5,
    NearLinkedReference = 6,
    AtPackagelocation = 7,
    AliasReference = 8,
    AliasLocation = 9,
    NearSelf = 12,
}

core_util::impl_enumset_type!(PackageLocationType => u8);

/// C++ `RE::PackageLocation::Data`
#[repr(C)]
#[derive(Clone, Copy)]
pub union PackageLocationData {
    pub object: *mut TESForm,
    pub ref_handle: ObjectRefHandle,
}

const _: () = assert!(core::mem::size_of::<PackageLocationData>() == 0x8);

/// C++ `RE::PackageLocation`
#[repr(C)]
pub struct PackageLocation {
    pub base: IAIWorldLocationHandle,               // 00
    pub loc_type: EnumSet<PackageLocationType, u8>, // 08
    pub pad09: u8,                                  // 09
    pub pad0a: u16,                                 // 0A
    pub rad: u32,                                   // 0C
    pub data: PackageLocationData,                  // 10
}

const _: () = assert!(core::mem::size_of::<PackageLocation>() == 0x18);
const _: () = assert!(core::mem::offset_of!(PackageLocation, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(PackageLocation, loc_type) == 0x08);
const _: () = assert!(core::mem::offset_of!(PackageLocation, rad) == 0x0C);
const _: () = assert!(core::mem::offset_of!(PackageLocation, data) == 0x10);

impl RttiType for PackageLocation {
    const RTTI: VariantID = RTTI_PackageLocation;
}

inherit!(PackageLocation : IAIWorldLocationHandle);

impl PackageLocation {
    pub const RTTI: VariantID = RTTI_PackageLocation;
    pub const VTABLE: &'static [VariantID] = &VTABLE_PackageLocation;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_ALLOCATE_LOCATION: usize = 0x01;
        pub fn allocate_location(
            &mut self,
            context: *mut AIWorldLocationContext
        ) -> *const IAIWorldLocation
    }

    crate::virtual_method! {
        pub const VFUNC_GET_AS_PACKAGE_LOCATION: usize = 0x02;
        pub fn get_as_package_location(&mut self) -> *mut PackageLocation
    }

    crate::virtual_method! {
        pub const VFUNC_IS_REF_AT_LOCATION: usize = 0x03;
        pub fn is_ref_at_location(
            &mut self,
            context: *mut AIWorldLocationContext,
            refr: *mut TESObjectREFR
        ) -> bool
    }
}
