use crate::offsets::offsets_rtti::RTTI_IAnimationSetCallbackFunctor;
use crate::offsets::offsets_vtable::VTABLE_IAnimationSetCallbackFunctor;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::IAnimationSetCallbackFunctor`
#[repr(C)]
pub struct IAnimationSetCallbackFunctor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<IAnimationSetCallbackFunctor>() == 0x8);
const _: () = assert!(core::mem::offset_of!(IAnimationSetCallbackFunctor, vtable) == 0x00);

impl RttiType for IAnimationSetCallbackFunctor {
    const RTTI: VariantID = RTTI_IAnimationSetCallbackFunctor;
}

impl AsRef<IAnimationSetCallbackFunctor> for IAnimationSetCallbackFunctor {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<IAnimationSetCallbackFunctor> for IAnimationSetCallbackFunctor {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl IAnimationSetCallbackFunctor {
    pub const RTTI: VariantID = RTTI_IAnimationSetCallbackFunctor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IAnimationSetCallbackFunctor;

    // TODO: `HighProcessData` stores `BSTSmartPointer<IAnimationSetCallbackFunctor>`, but
    // the CommonLib header exposes no source-backed intrusive-refcount base or ownership
    // manager contract here yet. Keep smart-pointer ownership unresolved until that
    // prerequisite is translated honestly.

    virtual_method! {
        pub const VFUNC_DESTRUCTOR: usize = 0x00;
        pub fn destructor(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_01: usize = 0x01;
        pub fn unk_01(&mut self)
    }
}

pub trait IAnimationSetCallbackFunctorExt {
    fn destructor(&mut self);
    fn unk_01(&mut self);
}

impl<T: AsMut<IAnimationSetCallbackFunctor>> IAnimationSetCallbackFunctorExt for T {
    #[inline(always)]
    fn destructor(&mut self) {
        IAnimationSetCallbackFunctor::destructor(self.as_mut())
    }

    #[inline(always)]
    fn unk_01(&mut self) {
        IAnimationSetCallbackFunctor::unk_01(self.as_mut())
    }
}
