use crate::offsets::offsets_rtti::RTTI_IPostAnimationChannelUpdateFunctor;
use crate::offsets::offsets_vtable::VTABLE_IPostAnimationChannelUpdateFunctor;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::IPostAnimationChannelUpdateFunctor`
#[repr(C)]
pub struct IPostAnimationChannelUpdateFunctor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<IPostAnimationChannelUpdateFunctor>() == 0x8);
const _: () = assert!(core::mem::offset_of!(IPostAnimationChannelUpdateFunctor, vtable) == 0x00);

impl RttiType for IPostAnimationChannelUpdateFunctor {
    const RTTI: VariantID = RTTI_IPostAnimationChannelUpdateFunctor;
}

impl AsRef<IPostAnimationChannelUpdateFunctor> for IPostAnimationChannelUpdateFunctor {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<IPostAnimationChannelUpdateFunctor> for IPostAnimationChannelUpdateFunctor {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl IPostAnimationChannelUpdateFunctor {
    pub const RTTI: VariantID = RTTI_IPostAnimationChannelUpdateFunctor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IPostAnimationChannelUpdateFunctor;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_01: usize = 0x01;
        pub fn unk_01()
    }
}

pub trait IPostAnimationChannelUpdateFunctorExt {
    fn dtor(&mut self);
    fn unk_01(&mut self);
}

impl<T: AsRef<IPostAnimationChannelUpdateFunctor> + AsMut<IPostAnimationChannelUpdateFunctor>>
    IPostAnimationChannelUpdateFunctorExt for T
{
    #[inline(always)]
    fn dtor(&mut self) {
        IPostAnimationChannelUpdateFunctor::dtor(self.as_mut())
    }

    #[inline(always)]
    fn unk_01(&mut self) {
        IPostAnimationChannelUpdateFunctor::unk_01(self.as_mut())
    }
}
