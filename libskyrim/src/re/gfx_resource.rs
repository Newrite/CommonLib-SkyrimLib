#![allow(non_camel_case_types)]

use crate::ffi::commonlib_gfx_resource_delete;
use core::sync::atomic::{AtomicI32, Ordering};

use core_util::{Enum, inherit};

use crate::re::{
    GAtomicInt, GFxResourceKey, GFxResourceLibBase, GFxResourceReport, GNewOverrideBase,
    GPtrTarget, GStatGroups,
};

/// C++ `RE::GFxResource::ResourceType`
#[libskyrim_macros::open_enum(ignore(kTypeCode_Mask, kTypeCode_Shift))]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxResourceResourceType {
    kNone = 0,
    kImage = 1,
    kFont = 2,
    kMovieDef = 3,
    kSoundSample = 4,
    kMovieDataDef = (1 << 7),
    kButtonDef = (1 << 7) | 1,
    kTextDef = (1 << 7) | 2,
    kEditTextDef = (1 << 7) | 3,
    kSpriteDef = (1 << 7) | 4,
    kShapeDef = (1 << 7) | 5,
    kVideoDef = (1 << 7) | 6,
    kTypeCode_Mask = 0xFF00,
    kTypeCode_Shift = 8,
}

impl GFxResourceResourceType {
    pub const CHARACTER_DEF_BIT: u32 = 1 << 7;
}

/// C++ `RE::GFxResource::ResourceUse`
#[libskyrim_macros::open_enum(ignore(kTypeCode_Mask))]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxResourceResourceUse {
    kNone = 0,
    kBitmap = 1,
    kGradient = 2,
    kFontTexture = 3,
    kSoundSample = 4,
    kTypeCode_Mask = 0xFF,
}

/// C++ `RE::GFxResource`
#[repr(C)]
pub struct GFxResource {
    pub base: GNewOverrideBase<{ GStatGroups::DEFAULT_MEM as u32 }>, // 00
    pub vtable: *const usize,                                        // 00
    pub ref_count: GAtomicInt<i32>,                                  // 08
    pub pad0c: u32,                                                  // 0C
    pub lib: *mut GFxResourceLibBase,                                // 10
}

const _: () = assert!(core::mem::size_of::<GFxResource>() == 0x18);
const _: () = assert!(core::mem::offset_of!(GFxResource, ref_count) == 0x08);
const _: () = assert!(core::mem::offset_of!(GFxResource, lib) == 0x10);

inherit!(GFxResource => GNewOverrideBase<{ GStatGroups::DEFAULT_MEM as u32 }>, base);

impl GFxResource {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_GET_KEY: usize = 0x01; pub fn get_key(&mut self) -> GFxResourceKey }
    crate::virtual_method! { pub const VFUNC_GET_RESOURCE_TYPE_CODE: usize = 0x02; pub fn get_resource_type_code(&self) -> u32 }
    crate::virtual_method! { pub const VFUNC_GET_RESOURCE_REPORT: usize = 0x03; pub fn get_resource_report(&mut self) -> *mut GFxResourceReport }

    #[inline(always)]
    pub const fn make_type_code(
        resource_type: GFxResourceResourceType,
        resource_use: GFxResourceResourceUse,
    ) -> u32 {
        ((resource_type as u32) << (GFxResourceResourceType::kTypeCode_Shift as u32))
            | (resource_use as u32)
    }

    #[inline(always)]
    pub fn add_ref(&mut self) {
        let counter =
            unsafe { &*(core::ptr::addr_of!(self.ref_count.base.value).cast::<AtomicI32>()) };
        counter.fetch_add(1, Ordering::AcqRel);
    }

    #[inline(always)]
    pub fn add_ref_not_zero(&mut self) -> bool {
        let counter =
            unsafe { &*(core::ptr::addr_of!(self.ref_count.base.value).cast::<AtomicI32>()) };
        let mut current = counter.load(Ordering::Acquire);
        loop {
            if current == 0 {
                return false;
            }
            match counter.compare_exchange_weak(
                current,
                current + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(next) => current = next,
            }
        }
    }

    #[inline(always)]
    pub fn release(&mut self) {
        let counter =
            unsafe { &*(core::ptr::addr_of!(self.ref_count.base.value).cast::<AtomicI32>()) };
        if counter.fetch_sub(1, Ordering::AcqRel) == 1 {
            if !self.lib.is_null() {
                unsafe {
                    (*self.lib).remove_resource_on_release(self);
                }
                self.lib = core::ptr::null_mut();
            }
            unsafe {
                commonlib_gfx_resource_delete(core::ptr::from_mut(self).cast());
            }
        }
    }

    #[inline(always)]
    pub fn get_ref_count(&self) -> i32 {
        let counter =
            unsafe { &*(core::ptr::addr_of!(self.ref_count.base.value).cast::<AtomicI32>()) };
        counter.load(Ordering::Acquire)
    }

    #[inline(always)]
    pub fn set_owner_resource_lib(&mut self, lib: *mut GFxResourceLibBase) {
        debug_assert!(self.lib.is_null() || lib.is_null());
        self.lib = lib;
    }

    #[inline(always)]
    pub fn resource_type_storage(&self) -> Enum<GFxResourceResourceType, u32> {
        Enum::from_underlying(self.get_resource_type_code() >> 8)
    }

    #[inline(always)]
    pub fn get_resource_type(&self) -> GFxResourceResourceType {
        self.try_get_resource_type()
            .unwrap_or(GFxResourceResourceType::kNone)
    }

    #[inline(always)]
    pub fn try_get_resource_type(&self) -> Option<GFxResourceResourceType> {
        self.resource_type_storage().get()
    }

    #[inline(always)]
    pub fn resource_use_storage(&self) -> Enum<GFxResourceResourceUse, u32> {
        Enum::from_underlying(self.get_resource_type_code() & 0xFF)
    }

    #[inline(always)]
    pub fn get_resource_use(&self) -> GFxResourceResourceUse {
        self.try_get_resource_use()
            .unwrap_or(GFxResourceResourceUse::kNone)
    }

    #[inline(always)]
    pub fn try_get_resource_use(&self) -> Option<GFxResourceResourceUse> {
        self.resource_use_storage().get()
    }
}

impl GPtrTarget for GFxResource {
    #[inline(always)]
    fn gptr_add_ref(&self) {
        unsafe {
            (*(core::ptr::from_ref(self).cast_mut())).add_ref();
        }
    }

    #[inline(always)]
    fn gptr_release(&self) {
        unsafe {
            (*(core::ptr::from_ref(self).cast_mut())).release();
        }
    }
}

impl AsRef<GFxResource> for GFxResource {
    #[inline(always)]
    fn as_ref(&self) -> &GFxResource {
        self
    }
}

impl AsMut<GFxResource> for GFxResource {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut GFxResource {
        self
    }
}

pub trait GFxResourceExt: AsRef<GFxResource> + AsMut<GFxResource> {
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn get_key(&mut self) -> GFxResourceKey {
        self.as_mut().get_key()
    }

    #[inline(always)]
    fn get_resource_type_code(&self) -> u32 {
        self.as_ref().get_resource_type_code()
    }

    #[inline(always)]
    fn get_resource_report(&mut self) -> *mut GFxResourceReport {
        self.as_mut().get_resource_report()
    }

    #[inline(always)]
    fn add_ref(&mut self) {
        self.as_mut().add_ref()
    }

    #[inline(always)]
    fn add_ref_not_zero(&mut self) -> bool {
        self.as_mut().add_ref_not_zero()
    }

    #[inline(always)]
    fn release(&mut self) {
        self.as_mut().release()
    }

    #[inline(always)]
    fn get_ref_count(&self) -> i32 {
        self.as_ref().get_ref_count()
    }

    #[inline(always)]
    fn set_owner_resource_lib(&mut self, lib: *mut GFxResourceLibBase) {
        self.as_mut().set_owner_resource_lib(lib)
    }

    #[inline(always)]
    fn resource_type_storage(&self) -> Enum<GFxResourceResourceType, u32> {
        self.as_ref().resource_type_storage()
    }

    #[inline(always)]
    fn get_resource_type(&self) -> GFxResourceResourceType {
        self.as_ref().get_resource_type()
    }

    #[inline(always)]
    fn try_get_resource_type(&self) -> Option<GFxResourceResourceType> {
        self.as_ref().try_get_resource_type()
    }

    #[inline(always)]
    fn resource_use_storage(&self) -> Enum<GFxResourceResourceUse, u32> {
        self.as_ref().resource_use_storage()
    }

    #[inline(always)]
    fn get_resource_use(&self) -> GFxResourceResourceUse {
        self.as_ref().get_resource_use()
    }

    #[inline(always)]
    fn try_get_resource_use(&self) -> Option<GFxResourceResourceUse> {
        self.as_ref().try_get_resource_use()
    }
}

impl<T> GFxResourceExt for T where T: AsRef<GFxResource> + AsMut<GFxResource> {}
