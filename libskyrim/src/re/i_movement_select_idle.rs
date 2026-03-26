use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_IMovementSelectIdle;
use crate::offsets::offsets_vtable::VTABLE_IMovementSelectIdle;
use crate::re::IMovementInterface;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::IMovementSelectIdle`
#[repr(C)]
pub struct IMovementSelectIdle {
    pub base: IMovementInterface, // 00
}

const _: () = assert!(core::mem::size_of::<IMovementSelectIdle>() == 0x8);
const _: () = assert!(core::mem::offset_of!(IMovementSelectIdle, base) == 0x00);

impl RttiType for IMovementSelectIdle {
    const RTTI: VariantID = RTTI_IMovementSelectIdle;
}

inherit!(IMovementSelectIdle : IMovementInterface, base);

impl AsRef<IMovementSelectIdle> for IMovementSelectIdle {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<IMovementSelectIdle> for IMovementSelectIdle {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl IMovementSelectIdle {
    pub const RTTI: VariantID = RTTI_IMovementSelectIdle;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMovementSelectIdle;

    virtual_method! { pub const VFUNC_UNK_01: usize = 0x01; pub fn unk_01(&mut self) }
    virtual_method! { pub const VFUNC_UNK_02: usize = 0x02; pub fn unk_02(&mut self) }
    virtual_method! { pub const VFUNC_UNK_03: usize = 0x03; pub fn unk_03(&mut self) }
    virtual_method! { pub const VFUNC_UNK_04: usize = 0x04; pub fn unk_04(&mut self) }
    virtual_method! { pub const VFUNC_UNK_05: usize = 0x05; pub fn unk_05(&mut self) }
}

pub trait IMovementSelectIdleExt {
    fn unk_01(&mut self);
    fn unk_02(&mut self);
    fn unk_03(&mut self);
    fn unk_04(&mut self);
    fn unk_05(&mut self);
}

impl<T: AsMut<IMovementSelectIdle>> IMovementSelectIdleExt for T {
    #[inline(always)]
    fn unk_01(&mut self) {
        IMovementSelectIdle::unk_01(self.as_mut())
    }
    #[inline(always)]
    fn unk_02(&mut self) {
        IMovementSelectIdle::unk_02(self.as_mut())
    }
    #[inline(always)]
    fn unk_03(&mut self) {
        IMovementSelectIdle::unk_03(self.as_mut())
    }
    #[inline(always)]
    fn unk_04(&mut self) {
        IMovementSelectIdle::unk_04(self.as_mut())
    }
    #[inline(always)]
    fn unk_05(&mut self) {
        IMovementSelectIdle::unk_05(self.as_mut())
    }
}
