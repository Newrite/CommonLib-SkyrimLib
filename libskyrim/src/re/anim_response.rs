use crate::re::bs_intrusive_ref_counted::BSIntrusiveRefCounted;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;

crate::core_util::abstract_type! {
    pub type AnimResponse;
}

#[repr(C)]
struct AnimResponseIntrusiveRefCountedView {
    vtable: *const usize,
    base: BSIntrusiveRefCounted,
}

const _: () = assert!(core::mem::size_of::<AnimResponseIntrusiveRefCountedView>() == 0x10);
const _: () = assert!(core::mem::offset_of!(AnimResponseIntrusiveRefCountedView, base) == 0x08);

impl BSTSmartPointerIntrusiveRefCountable for AnimResponse {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        unsafe {
            (&*(self as *const Self as *const AnimResponseIntrusiveRefCountedView))
                .base
                .inc_ref()
        };
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        unsafe {
            (&*(self as *const Self as *const AnimResponseIntrusiveRefCountedView))
                .base
                .dec_ref()
        }
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let func: extern "C" fn(*mut Self) =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0usize) };
        func(self as *const Self as *mut Self);
    }
}
