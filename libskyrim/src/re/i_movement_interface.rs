use crate::offsets::offsets_rtti::RTTI_IMovementInterface;
use crate::offsets::offsets_vtable::VTABLE_IMovementInterface;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::IMovementInterface`
#[repr(C)]
pub struct IMovementInterface {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<IMovementInterface>() == 0x8);
const _: () = assert!(core::mem::offset_of!(IMovementInterface, vtable) == 0x00);

impl RttiType for IMovementInterface {
    const RTTI: VariantID = RTTI_IMovementInterface;
}

impl AsRef<IMovementInterface> for IMovementInterface {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<IMovementInterface> for IMovementInterface {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl IMovementInterface {
    pub const RTTI: VariantID = RTTI_IMovementInterface;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMovementInterface;

    virtual_method! {
        pub const VFUNC_DESTRUCTOR: usize = 0x00;
        pub fn destructor(&mut self)
    }
}

pub trait IMovementInterfaceExt {
    fn destructor(&mut self);
}

impl<T: AsMut<IMovementInterface>> IMovementInterfaceExt for T {
    #[inline(always)]
    fn destructor(&mut self) {
        IMovementInterface::destructor(self.as_mut())
    }
}
