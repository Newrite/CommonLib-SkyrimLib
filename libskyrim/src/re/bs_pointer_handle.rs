//! Translation of `RE::BSPointerHandle.h` (partial).
//!
//! Provides `ObjectRefHandle` — a handle to a `TESObjectREFR`.
//! In C++ this is `BSPointerHandle<TESObjectREFR>`, which wraps
//! `BSUntypedPointerHandle<21, 5>` — a single `u32`.

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
}
