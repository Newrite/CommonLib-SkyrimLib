use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_IMovementMessageInterface;
use crate::offsets::offsets_vtable::VTABLE_IMovementMessageInterface;
use crate::re::IMovementInterface;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::IMovementMessageInterface`
#[repr(C)]
pub struct IMovementMessageInterface {
    pub base: IMovementInterface, // 00
}

const _: () = assert!(core::mem::size_of::<IMovementMessageInterface>() == 0x8);
const _: () = assert!(core::mem::offset_of!(IMovementMessageInterface, base) == 0x00);

impl RttiType for IMovementMessageInterface {
    const RTTI: VariantID = RTTI_IMovementMessageInterface;
}

inherit!(IMovementMessageInterface : IMovementInterface, base);

impl AsRef<IMovementMessageInterface> for IMovementMessageInterface {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<IMovementMessageInterface> for IMovementMessageInterface {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl IMovementMessageInterface {
    pub const RTTI: VariantID = RTTI_IMovementMessageInterface;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMovementMessageInterface;

    virtual_method! { pub const VFUNC_UNK_01: usize = 0x01; pub fn unk_01(&mut self) }
    virtual_method! { pub const VFUNC_UNK_02: usize = 0x02; pub fn unk_02(&mut self) }
    virtual_method! { pub const VFUNC_UNK_03: usize = 0x03; pub fn unk_03(&mut self) }
}

pub trait IMovementMessageInterfaceExt {
    fn unk_01(&mut self);
    fn unk_02(&mut self);
    fn unk_03(&mut self);
}

impl<T: AsMut<IMovementMessageInterface>> IMovementMessageInterfaceExt for T {
    #[inline(always)]
    fn unk_01(&mut self) {
        IMovementMessageInterface::unk_01(self.as_mut())
    }
    #[inline(always)]
    fn unk_02(&mut self) {
        IMovementMessageInterface::unk_02(self.as_mut())
    }
    #[inline(always)]
    fn unk_03(&mut self) {
        IMovementMessageInterface::unk_03(self.as_mut())
    }
}
