//! Translation of `RE::BSPointerHandle.h` (partial).
//!
//! Provides `ObjectRefHandle` вЂ” a handle to a `TESObjectREFR`.
//! In C++ this is `BSPointerHandle<TESObjectREFR>`, which wraps
//! `BSUntypedPointerHandle<21, 5>` вЂ” a single `u32`.

use crate::re::{Actor, BSHandleRefObject, NiPointer, NiRef, Projectile, TESObjectREFR};
use crate::relocation::RelocationID;

/// C++ `RE::ObjectRefHandle` = `BSPointerHandle<TESObjectREFR>`
///
/// A handle used by the engine to safely reference `TESObjectREFR`
/// instances. Internally just a `u32` with encoded free-list bits and age.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, bytemuck::Zeroable, bytemuck::Pod)]
pub struct ObjectRefHandle {
    pub handle: u32, // 0
}

const _: () = assert!(core::mem::size_of::<ObjectRefHandle>() == 0x4);

/// C++ `RE::ActorHandle` = `BSPointerHandle<Actor>`
///
/// A handle used by the engine to safely reference `Actor`
/// instances. Internally just a `u32` with encoded free-list bits and age.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, bytemuck::Zeroable, bytemuck::Pod)]
pub struct ActorHandle {
    pub handle: u32, // 0
}

const _: () = assert!(core::mem::size_of::<ActorHandle>() == 0x4);

/// C++ `RE::ProjectileHandle` = `BSPointerHandle<Projectile>`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, bytemuck::Zeroable, bytemuck::Pod)]
pub struct ProjectileHandle {
    pub handle: u32, // 0
}

const _: () = assert!(core::mem::size_of::<ProjectileHandle>() == 0x4);

impl NiRef for Projectile {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (&*(self as *const Self as *const BSHandleRefObject)).inc_ref_count() };
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (&*(self as *const Self as *const BSHandleRefObject)).dec_ref_count() };
    }
}

impl ActorHandle {
    /// Creates a null/empty handle.
    #[inline(always)]
    pub const fn new() -> Self {
        Self { handle: 0 }
    }

    /// Returns `true` if the handle has a value (non-zero).
    #[inline(always)]
    pub const fn has_value(&self) -> bool {
        self.handle != 0
    }

    /// Returns the raw handle value.
    #[inline(always)]
    pub const fn value(&self) -> u32 {
        self.handle
    }

    /// Resets the handle to zero (null).
    #[inline(always)]
    pub fn reset(&mut self) {
        self.handle = 0;
    }

    #[inline(always)]
    pub fn get(&self) -> NiPointer<Actor> {
        let mut out = NiPointer::null();
        let _ = self.get_smart_pointer(&mut out);
        out
    }

    #[inline(always)]
    pub fn from_ptr(ptr: *mut Actor) -> Self {
        Self::get_handle(ptr)
    }

    crate::relocation_func! {
        pub fn get_handle(ptr: *mut Actor) -> ActorHandle => RelocationID::new(15967, 16212)
    }

    crate::relocation_func! {
        fn get_smart_pointer(&self, out: &mut NiPointer<Actor>) -> bool => RelocationID::new(12204, 12332)
    }
}

impl ProjectileHandle {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { handle: 0 }
    }

    #[inline(always)]
    pub const fn has_value(&self) -> bool {
        self.handle != 0
    }

    #[inline(always)]
    pub const fn value(&self) -> u32 {
        self.handle
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.handle = 0;
    }

    #[inline(always)]
    pub fn get(&self) -> NiPointer<Projectile> {
        let mut out = NiPointer::null();
        let _ = self.get_smart_pointer(&mut out);
        out
    }

    #[inline(always)]
    pub fn from_ptr(ptr: *mut Projectile) -> Self {
        Self::get_handle(ptr)
    }

    crate::relocation_func! {
        pub fn get_handle(ptr: *mut Projectile) -> ProjectileHandle => RelocationID::new(15967, 16212)
    }

    crate::relocation_func! {
        fn get_smart_pointer(&self, out: &mut NiPointer<Projectile>) -> bool => RelocationID::new(12204, 12332)
    }
}

impl ObjectRefHandle {
    /// Creates a null/empty handle.
    #[inline(always)]
    pub const fn new() -> Self {
        Self { handle: 0 }
    }

    /// Returns `true` if the handle has a value (non-zero).
    #[inline(always)]
    pub const fn has_value(&self) -> bool {
        self.handle != 0
    }

    /// Returns the raw handle value.
    #[inline(always)]
    pub const fn value(&self) -> u32 {
        self.handle
    }

    /// Resets the handle to zero (null).
    #[inline(always)]
    pub fn reset(&mut self) {
        self.handle = 0;
    }

    #[inline(always)]
    pub fn get(&self) -> NiPointer<TESObjectREFR> {
        let mut out = NiPointer::null();
        let _ = self.get_smart_pointer(&mut out);
        out
    }

    #[inline(always)]
    pub fn from_ptr(ptr: *mut TESObjectREFR) -> Self {
        Self::get_handle(ptr)
    }

    crate::relocation_func! {
        pub fn get_handle(ptr: *mut TESObjectREFR) -> ObjectRefHandle => RelocationID::new(15967, 16212)
    }

    crate::relocation_func! {
        fn get_smart_pointer(&self, out: &mut NiPointer<TESObjectREFR>) -> bool => RelocationID::new(12204, 12332)
    }
}

impl crate::re::bssimple_list::BSSimpleListValue for ActorHandle {
    #[inline(always)]
    fn bs_has_value(&self) -> bool {
        self.has_value()
    }
}

impl crate::re::bssimple_list::BSSimpleListValue for ObjectRefHandle {
    #[inline(always)]
    fn bs_has_value(&self) -> bool {
        self.has_value()
    }
}

impl crate::re::bssimple_list::BSSimpleListValue for ProjectileHandle {
    #[inline(always)]
    fn bs_has_value(&self) -> bool {
        self.has_value()
    }
}
