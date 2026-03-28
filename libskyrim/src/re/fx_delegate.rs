use crate::ffi::{commonlib_fx_delegate_add_ref, commonlib_fx_delegate_release};
use crate::offsets::offsets_rtti::RTTI_FxDelegate;
use crate::offsets::offsets_vtable::VTABLE_FxDelegate;
use crate::re::GPtrTarget;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    /// Pointer-compatible partial translation of `RE::FxDelegate`.
    ///
    /// Current `UI`/`IMenu` usage only needs the pointee identity and pointer
    /// layout through `GPtr<FxDelegate>`.
    pub type FxDelegate;
}

impl RttiType for FxDelegate {
    const RTTI: VariantID = RTTI_FxDelegate;
}

impl FxDelegate {
    pub const RTTI: VariantID = RTTI_FxDelegate;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FxDelegate;
}

impl GPtrTarget for FxDelegate {
    #[inline(always)]
    fn gptr_add_ref(&self) {
        unsafe {
            commonlib_fx_delegate_add_ref(core::ptr::from_ref(self).cast_mut().cast());
        }
    }

    #[inline(always)]
    fn gptr_release(&self) {
        unsafe {
            commonlib_fx_delegate_release(core::ptr::from_ref(self).cast_mut().cast());
        }
    }
}
