use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_IMovementDirectControl;
use crate::offsets::offsets_vtable::VTABLE_IMovementDirectControl;
use crate::re::IMovementInterface;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::IMovementDirectControl`
#[repr(C)]
pub struct IMovementDirectControl {
    pub base: IMovementInterface, // 00
}

const _: () = assert!(core::mem::size_of::<IMovementDirectControl>() == 0x8);
const _: () = assert!(core::mem::offset_of!(IMovementDirectControl, base) == 0x00);

impl RttiType for IMovementDirectControl {
    const RTTI: VariantID = RTTI_IMovementDirectControl;
}

inherit!(IMovementDirectControl : IMovementInterface, base);

impl AsRef<IMovementDirectControl> for IMovementDirectControl {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<IMovementDirectControl> for IMovementDirectControl {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl IMovementDirectControl {
    pub const RTTI: VariantID = RTTI_IMovementDirectControl;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMovementDirectControl;

    virtual_method! { pub const VFUNC_UNK_01: usize = 0x01; pub fn unk_01(&mut self) }
    virtual_method! { pub const VFUNC_UNK_02: usize = 0x02; pub fn unk_02(&mut self) }
    virtual_method! { pub const VFUNC_UNK_03: usize = 0x03; pub fn unk_03(&mut self) }
    virtual_method! { pub const VFUNC_UNK_04: usize = 0x04; pub fn unk_04(&mut self) }
    virtual_method! { pub const VFUNC_UNK_05: usize = 0x05; pub fn unk_05(&mut self) }
    virtual_method! { pub const VFUNC_UNK_06: usize = 0x06; pub fn unk_06(&mut self) }
    virtual_method! { pub const VFUNC_UNK_07: usize = 0x07; pub fn unk_07(&mut self) }
    virtual_method! { pub const VFUNC_UNK_08: usize = 0x08; pub fn unk_08(&mut self) }
}

pub trait IMovementDirectControlExt {
    fn unk_01(&mut self);
    fn unk_02(&mut self);
    fn unk_03(&mut self);
    fn unk_04(&mut self);
    fn unk_05(&mut self);
    fn unk_06(&mut self);
    fn unk_07(&mut self);
    fn unk_08(&mut self);
}

impl<T: AsMut<IMovementDirectControl>> IMovementDirectControlExt for T {
    #[inline(always)]
    fn unk_01(&mut self) {
        IMovementDirectControl::unk_01(self.as_mut())
    }
    #[inline(always)]
    fn unk_02(&mut self) {
        IMovementDirectControl::unk_02(self.as_mut())
    }
    #[inline(always)]
    fn unk_03(&mut self) {
        IMovementDirectControl::unk_03(self.as_mut())
    }
    #[inline(always)]
    fn unk_04(&mut self) {
        IMovementDirectControl::unk_04(self.as_mut())
    }
    #[inline(always)]
    fn unk_05(&mut self) {
        IMovementDirectControl::unk_05(self.as_mut())
    }
    #[inline(always)]
    fn unk_06(&mut self) {
        IMovementDirectControl::unk_06(self.as_mut())
    }
    #[inline(always)]
    fn unk_07(&mut self) {
        IMovementDirectControl::unk_07(self.as_mut())
    }
    #[inline(always)]
    fn unk_08(&mut self) {
        IMovementDirectControl::unk_08(self.as_mut())
    }
}
