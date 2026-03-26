use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_IMovementMotionDrivenControl;
use crate::offsets::offsets_vtable::VTABLE_IMovementMotionDrivenControl;
use crate::re::IMovementInterface;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::IMovementMotionDrivenControl`
#[repr(C)]
pub struct IMovementMotionDrivenControl {
    pub base: IMovementInterface, // 00
}

const _: () = assert!(core::mem::size_of::<IMovementMotionDrivenControl>() == 0x8);
const _: () = assert!(core::mem::offset_of!(IMovementMotionDrivenControl, base) == 0x00);

impl RttiType for IMovementMotionDrivenControl {
    const RTTI: VariantID = RTTI_IMovementMotionDrivenControl;
}

inherit!(IMovementMotionDrivenControl : IMovementInterface, base);

impl AsRef<IMovementMotionDrivenControl> for IMovementMotionDrivenControl {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<IMovementMotionDrivenControl> for IMovementMotionDrivenControl {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl IMovementMotionDrivenControl {
    pub const RTTI: VariantID = RTTI_IMovementMotionDrivenControl;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMovementMotionDrivenControl;

    virtual_method! { pub const VFUNC_UNK_01: usize = 0x01; pub fn unk_01(&mut self) }
    virtual_method! { pub const VFUNC_UNK_02: usize = 0x02; pub fn unk_02(&mut self) }
    virtual_method! { pub const VFUNC_UNK_03: usize = 0x03; pub fn unk_03(&mut self) }
    virtual_method! { pub const VFUNC_UNK_04: usize = 0x04; pub fn unk_04(&mut self) }
    virtual_method! { pub const VFUNC_UNK_05: usize = 0x05; pub fn unk_05(&mut self) }
    virtual_method! { pub const VFUNC_UNK_06: usize = 0x06; pub fn unk_06(&mut self) }
    virtual_method! { pub const VFUNC_UNK_07: usize = 0x07; pub fn unk_07(&mut self) }
    virtual_method! { pub const VFUNC_UNK_08: usize = 0x08; pub fn unk_08(&mut self) }
}

pub trait IMovementMotionDrivenControlExt {
    fn unk_01(&mut self);
    fn unk_02(&mut self);
    fn unk_03(&mut self);
    fn unk_04(&mut self);
    fn unk_05(&mut self);
    fn unk_06(&mut self);
    fn unk_07(&mut self);
    fn unk_08(&mut self);
}

impl<T: AsMut<IMovementMotionDrivenControl>> IMovementMotionDrivenControlExt for T {
    #[inline(always)]
    fn unk_01(&mut self) {
        IMovementMotionDrivenControl::unk_01(self.as_mut())
    }
    #[inline(always)]
    fn unk_02(&mut self) {
        IMovementMotionDrivenControl::unk_02(self.as_mut())
    }
    #[inline(always)]
    fn unk_03(&mut self) {
        IMovementMotionDrivenControl::unk_03(self.as_mut())
    }
    #[inline(always)]
    fn unk_04(&mut self) {
        IMovementMotionDrivenControl::unk_04(self.as_mut())
    }
    #[inline(always)]
    fn unk_05(&mut self) {
        IMovementMotionDrivenControl::unk_05(self.as_mut())
    }
    #[inline(always)]
    fn unk_06(&mut self) {
        IMovementMotionDrivenControl::unk_06(self.as_mut())
    }
    #[inline(always)]
    fn unk_07(&mut self) {
        IMovementMotionDrivenControl::unk_07(self.as_mut())
    }
    #[inline(always)]
    fn unk_08(&mut self) {
        IMovementMotionDrivenControl::unk_08(self.as_mut())
    }
}
