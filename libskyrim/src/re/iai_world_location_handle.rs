use crate::offsets::offsets_rtti::RTTI_IAIWorldLocationHandle;
use crate::offsets::offsets_vtable::VTABLE_IAIWorldLocationHandle;
use crate::re::{AIWorldLocationContext, IAIWorldLocation, PackageLocation, TESObjectREFR};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::IAIWorldLocationHandle`
#[repr(C)]
pub struct IAIWorldLocationHandle {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<IAIWorldLocationHandle>() == 0x8);

impl RttiType for IAIWorldLocationHandle {
    const RTTI: VariantID = RTTI_IAIWorldLocationHandle;
}

impl IAIWorldLocationHandle {
    pub const RTTI: VariantID = RTTI_IAIWorldLocationHandle;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IAIWorldLocationHandle;

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

pub trait IAIWorldLocationHandleExt {
    fn dtor(&mut self);
    fn allocate_location(
        &mut self,
        context: *mut AIWorldLocationContext,
    ) -> *const IAIWorldLocation;
    fn get_as_package_location(&mut self) -> *mut PackageLocation;
    fn is_ref_at_location(
        &mut self,
        context: *mut AIWorldLocationContext,
        refr: *mut TESObjectREFR,
    ) -> bool;
}

impl<T: AsMut<IAIWorldLocationHandle>> IAIWorldLocationHandleExt for T {
    #[inline(always)]
    fn dtor(&mut self) {
        IAIWorldLocationHandle::dtor(self.as_mut())
    }

    #[inline(always)]
    fn allocate_location(
        &mut self,
        context: *mut AIWorldLocationContext,
    ) -> *const IAIWorldLocation {
        IAIWorldLocationHandle::allocate_location(self.as_mut(), context)
    }

    #[inline(always)]
    fn get_as_package_location(&mut self) -> *mut PackageLocation {
        IAIWorldLocationHandle::get_as_package_location(self.as_mut())
    }

    #[inline(always)]
    fn is_ref_at_location(
        &mut self,
        context: *mut AIWorldLocationContext,
        refr: *mut TESObjectREFR,
    ) -> bool {
        IAIWorldLocationHandle::is_ref_at_location(self.as_mut(), context, refr)
    }
}
