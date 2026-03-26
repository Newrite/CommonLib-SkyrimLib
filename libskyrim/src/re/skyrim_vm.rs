use crate::re::IVirtualMachine;
use crate::relocation::RelocationID;

/// Minimal source-backed `RE::SkyrimVM` prefix needed to reach `impl` at `0x200`.
///
/// TODO: Expand this into a fuller `SkyrimVM` translation when the surrounding
/// runtime-divergent layout and event-relay surface are needed beyond singleton
/// access to `IVirtualMachine`.
#[repr(C)]
pub struct SkyrimVM {
    pub pad00: [u8; 0x200],
    pub impl_: *mut IVirtualMachine,
}

const _: () = assert!(core::mem::size_of::<SkyrimVM>() == 0x208);
const _: () = assert!(core::mem::offset_of!(SkyrimVM, impl_) == 0x200);

impl SkyrimVM {
    crate::relocation_variable! {
        fn singleton_ptr() -> &'static *mut SkyrimVM => RelocationID::new(514315, 400475)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut SkyrimVM {
        *Self::singleton_ptr()
    }

    #[inline(always)]
    pub fn get_virtual_machine(&self) -> *mut IVirtualMachine {
        self.impl_
    }
}
