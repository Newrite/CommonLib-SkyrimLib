use core::sync::atomic::{AtomicU32, Ordering};
use crate::{virtual_method, relocation_variable};
use crate::relocation::{VariantID, RttiType};
use crate::offsets::offsets_rtti::RTTI_NiRefObject;
use crate::offsets::offsets_vtable::VTABLE_NiRefObject;
use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct NiRefObject {
    pub vtable: *const usize,
    pub ref_count: AtomicU32,
    pub pad0c: u32,
}

const _: () = assert!(core::mem::size_of::<NiRefObject>() == 0x10);

impl RttiType for NiRefObject {
    const RTTI: VariantID = RTTI_NiRefObject;
}

impl NiRefObject {
    pub const RTTI: VariantID = RTTI_NiRefObject;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiRefObject;

    virtual_method! {
        pub const VFUNC_DELETE_THIS: usize = 0x01;
        pub fn delete_this()
    }

    #[inline(always)]
    pub fn get_ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::Acquire)
    }

    #[inline(always)]
    pub fn inc_ref_count(&self) {
        self.ref_count.fetch_add(1, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn dec_ref_count(&self) {
        if self.ref_count.fetch_sub(1, Ordering::AcqRel) == 1 {
            // Безопасно передаем управление деструктору движка Скайрима
            self.delete_this();
        }
    }

    relocation_variable! {
        pub fn get_total_object_count() -> &'static AtomicU32 => VariantID::new(523912, 410493, 0)
    }
}

pub trait NiRef {
    fn inc_ref(&self);
    fn dec_ref(&self);
}

impl NiRef for NiRefObject {
    #[inline(always)]
    fn inc_ref(&self) { self.inc_ref_count(); }
    #[inline(always)]
    fn dec_ref(&self) { self.dec_ref_count(); }
}

#[repr(transparent)]
pub struct NiPointer<T: NiRef> {
    ptr: *mut T,
}

impl<T: NiRef> NiPointer<T> {
    #[inline(always)]
    pub unsafe fn from_raw(ptr: *mut T) -> Self { Self { ptr } }
    #[inline(always)]
    pub fn get(&self) -> *mut T { self.ptr }
    #[inline(always)]
    pub fn into_raw(self) -> *mut T {
        let ptr = self.ptr;
        core::mem::forget(self);
        ptr
    }
}

impl<T: NiRef> Clone for NiPointer<T> {
    fn clone(&self) -> Self {
        if !self.ptr.is_null() { unsafe { (*self.ptr).inc_ref(); } }
        Self { ptr: self.ptr }
    }
}

impl<T: NiRef> Drop for NiPointer<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() { unsafe { (*self.ptr).dec_ref(); } }
    }
}

impl<T: NiRef> Deref for NiPointer<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { unsafe { &*self.ptr } }
}

impl<T: NiRef> DerefMut for NiPointer<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { &mut *self.ptr } }
}