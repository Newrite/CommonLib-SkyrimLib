use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_IMovementPlannerDirectControl;
use crate::offsets::offsets_vtable::VTABLE_IMovementPlannerDirectControl;
use crate::re::IMovementInterface;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::IMovementPlannerDirectControl`
#[repr(C)]
pub struct IMovementPlannerDirectControl {
    pub base: IMovementInterface, // 00
}

const _: () = assert!(core::mem::size_of::<IMovementPlannerDirectControl>() == 0x8);
const _: () = assert!(core::mem::offset_of!(IMovementPlannerDirectControl, base) == 0x00);

impl RttiType for IMovementPlannerDirectControl {
    const RTTI: VariantID = RTTI_IMovementPlannerDirectControl;
}

inherit!(IMovementPlannerDirectControl : IMovementInterface, base);

impl AsRef<IMovementPlannerDirectControl> for IMovementPlannerDirectControl {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<IMovementPlannerDirectControl> for IMovementPlannerDirectControl {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl IMovementPlannerDirectControl {
    pub const RTTI: VariantID = RTTI_IMovementPlannerDirectControl;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMovementPlannerDirectControl;

    virtual_method! { pub const VFUNC_UNK_01: usize = 0x01; pub fn unk_01(&mut self) }
    virtual_method! { pub const VFUNC_UNK_02: usize = 0x02; pub fn unk_02(&mut self) }
    virtual_method! { pub const VFUNC_UNK_03: usize = 0x03; pub fn unk_03(&mut self) }
    virtual_method! { pub const VFUNC_UNK_04: usize = 0x04; pub fn unk_04(&mut self) }
    virtual_method! { pub const VFUNC_UNK_05: usize = 0x05; pub fn unk_05(&mut self) }
}

pub trait IMovementPlannerDirectControlExt {
    fn unk_01(&mut self);
    fn unk_02(&mut self);
    fn unk_03(&mut self);
    fn unk_04(&mut self);
    fn unk_05(&mut self);
}

impl<T: AsMut<IMovementPlannerDirectControl>> IMovementPlannerDirectControlExt for T {
    #[inline(always)]
    fn unk_01(&mut self) {
        IMovementPlannerDirectControl::unk_01(self.as_mut())
    }
    #[inline(always)]
    fn unk_02(&mut self) {
        IMovementPlannerDirectControl::unk_02(self.as_mut())
    }
    #[inline(always)]
    fn unk_03(&mut self) {
        IMovementPlannerDirectControl::unk_03(self.as_mut())
    }
    #[inline(always)]
    fn unk_04(&mut self) {
        IMovementPlannerDirectControl::unk_04(self.as_mut())
    }
    #[inline(always)]
    fn unk_05(&mut self) {
        IMovementPlannerDirectControl::unk_05(self.as_mut())
    }
}
