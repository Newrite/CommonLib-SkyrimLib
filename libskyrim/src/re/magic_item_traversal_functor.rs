use crate::offsets::offsets_rtti::RTTI_MagicItemTraversalFunctor;
use crate::offsets::offsets_vtable::VTABLE_MagicItemTraversalFunctor;
use crate::re::Effect;
use crate::re::bs_container::BSContainerForEachResult;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::MagicItemTraversalFunctor`
#[repr(C)]
pub struct MagicItemTraversalFunctor {
    pub vtable: *const usize, // 00
    pub index: u32,           // 08
    pub pad0c: u32,           // 0C
}

const _: () = assert!(core::mem::size_of::<MagicItemTraversalFunctor>() == 0x10);
const _: () = assert!(core::mem::offset_of!(MagicItemTraversalFunctor, index) == 0x08);
const _: () = assert!(core::mem::offset_of!(MagicItemTraversalFunctor, pad0c) == 0x0C);

impl RttiType for MagicItemTraversalFunctor {
    const RTTI: VariantID = RTTI_MagicItemTraversalFunctor;
}

impl AsRef<MagicItemTraversalFunctor> for MagicItemTraversalFunctor {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<MagicItemTraversalFunctor> for MagicItemTraversalFunctor {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl MagicItemTraversalFunctor {
    pub const RTTI: VariantID = RTTI_MagicItemTraversalFunctor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicItemTraversalFunctor;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_CALL: usize = 0x01;
        pub fn call(effect: *mut Effect) -> BSContainerForEachResult
    }
}

pub trait MagicItemTraversalFunctorExt {
    fn dtor(&mut self);
    fn call(&mut self, effect: *mut Effect) -> BSContainerForEachResult;
}

impl<T: AsRef<MagicItemTraversalFunctor> + AsMut<MagicItemTraversalFunctor>>
    MagicItemTraversalFunctorExt for T
{
    fn dtor(&mut self) {
        MagicItemTraversalFunctor::dtor(self.as_mut())
    }

    fn call(&mut self, effect: *mut Effect) -> BSContainerForEachResult {
        MagicItemTraversalFunctor::call(self.as_mut(), effect)
    }
}
