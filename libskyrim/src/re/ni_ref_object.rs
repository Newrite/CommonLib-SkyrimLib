use crate::offsets::offsets_rtti::RTTI_NiRefObject;
use crate::offsets::offsets_vtable::VTABLE_NiRefObject;
use crate::re::NiPointer;
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::{relocation_variable, virtual_method};
use core::sync::atomic::{AtomicU32, Ordering};

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

    /// Rust-side helper for header-level `make_nismart<NiRefObject>()`.
    #[inline(always)]
    pub fn make_ptr() -> Option<NiPointer<Self>> {
        unsafe {
            NiPointer::<Self>::try_construct_with(|out: *mut NiPointer<Self>| {
                crate::ffi::commonlib_make_nismart_ni_ref_object(out.cast())
            })
        }
    }

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_DELETE_THIS: usize = 0x01;
        pub fn delete_this()
    }

    #[inline(always)]
    pub fn get_ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::SeqCst)
    }

    #[inline(always)]
    pub fn inc_ref_count(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    #[inline(always)]
    pub fn dec_ref_count(&self) {
        if self.ref_count.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.delete_this();
        }
    }

    relocation_variable! {
        pub fn get_total_object_count() -> &'static AtomicU32 => RelocationID::new(523912, 410493)
    }
}

/// Trait for intrusive reference counting.
/// Implemented by `NiRefObject` and `BSHandleRefObject`.
/// Required bound for `NiPointer<T>`.
pub trait NiRef {
    fn inc_ref(&self);
    fn dec_ref(&self);
}

impl NiRef for NiRefObject {
    #[inline(always)]
    fn inc_ref(&self) {
        self.inc_ref_count();
    }
    #[inline(always)]
    fn dec_ref(&self) {
        self.dec_ref_count();
    }
}
