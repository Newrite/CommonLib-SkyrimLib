use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_IMovementState;
use crate::offsets::offsets_vtable::VTABLE_IMovementState;
use crate::re::{IMovementInterface, NiPoint3};
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::{relocation_func, virtual_method};

/// C++ `RE::IMovementState`
#[repr(C)]
pub struct IMovementState {
    pub base: IMovementInterface, // 00
}

const _: () = assert!(core::mem::size_of::<IMovementState>() == 0x8);
const _: () = assert!(core::mem::offset_of!(IMovementState, base) == 0x00);

impl RttiType for IMovementState {
    const RTTI: VariantID = RTTI_IMovementState;
}

inherit!(IMovementState : IMovementInterface, base);

impl AsRef<IMovementState> for IMovementState {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<IMovementState> for IMovementState {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl IMovementState {
    pub const RTTI: VariantID = RTTI_IMovementState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMovementState;

    relocation_func! {
        pub fn can_strafe(&self) -> bool => RelocationID::new(88498, 0)
    }

    // override (IMovementInterface)
    // ~IMovementState() override;  // 00

    virtual_method! {
        pub const VFUNC_UNK_01: usize = 0x01;
        pub fn unk_01(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_02: usize = 0x02;
        pub fn unk_02(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_03: usize = 0x03;
        pub fn unk_03(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_04: usize = 0x04;
        pub fn unk_04(&mut self)
    }

    virtual_method! {
        pub const VFUNC_DO_GET_MOVEMENT_SPEED: usize = 0x05;
        pub fn do_get_movement_speed(&mut self) -> f32
    }

    virtual_method! {
        pub const VFUNC_DO_GET_ROTATION_SPEED: usize = 0x06;
        pub fn do_get_rotation_speed(&mut self) -> f32
    }

    virtual_method! {
        pub const VFUNC_UNK_07: usize = 0x07;
        pub fn unk_07(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_08: usize = 0x08;
        pub fn unk_08(&mut self)
    }

    virtual_method! {
        pub const VFUNC_DO_GET_MOVEMENT_ROTATION: usize = 0x09;
        pub fn do_get_movement_rotation(&mut self, a_rotation: *mut NiPoint3)
    }

    virtual_method! {
        pub const VFUNC_UNK_0A: usize = 0x0A;
        pub fn unk_0a(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_0B: usize = 0x0B;
        pub fn unk_0b(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_0C: usize = 0x0C;
        pub fn unk_0c(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_0D: usize = 0x0D;
        pub fn unk_0d(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_0E: usize = 0x0E;
        pub fn unk_0e(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_0F: usize = 0x0F;
        pub fn unk_0f(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_10: usize = 0x10;
        pub fn unk_10(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_11: usize = 0x11;
        pub fn unk_11(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_12: usize = 0x12;
        pub fn unk_12(&mut self)
    }

    virtual_method! {
        pub const VFUNC_UNK_13: usize = 0x13;
        pub fn unk_13(&mut self)
    }
}

pub trait IMovementStateExt {
    fn can_strafe(&self) -> bool;
    fn unk_01(&mut self);
    fn unk_02(&mut self);
    fn unk_03(&mut self);
    fn unk_04(&mut self);
    fn do_get_movement_speed(&mut self) -> f32;
    fn do_get_rotation_speed(&mut self) -> f32;
    fn unk_07(&mut self);
    fn unk_08(&mut self);
    fn do_get_movement_rotation(&mut self, a_rotation: *mut NiPoint3);
    fn unk_0a(&mut self);
    fn unk_0b(&mut self);
    fn unk_0c(&mut self);
    fn unk_0d(&mut self);
    fn unk_0e(&mut self);
    fn unk_0f(&mut self);
    fn unk_10(&mut self);
    fn unk_11(&mut self);
    fn unk_12(&mut self);
    fn unk_13(&mut self);
}

impl<T: AsRef<IMovementState> + AsMut<IMovementState>> IMovementStateExt for T {
    #[inline(always)]
    fn can_strafe(&self) -> bool {
        IMovementState::can_strafe(self.as_ref())
    }

    #[inline(always)]
    fn unk_01(&mut self) {
        IMovementState::unk_01(self.as_mut())
    }

    #[inline(always)]
    fn unk_02(&mut self) {
        IMovementState::unk_02(self.as_mut())
    }

    #[inline(always)]
    fn unk_03(&mut self) {
        IMovementState::unk_03(self.as_mut())
    }

    #[inline(always)]
    fn unk_04(&mut self) {
        IMovementState::unk_04(self.as_mut())
    }

    #[inline(always)]
    fn do_get_movement_speed(&mut self) -> f32 {
        IMovementState::do_get_movement_speed(self.as_mut())
    }

    #[inline(always)]
    fn do_get_rotation_speed(&mut self) -> f32 {
        IMovementState::do_get_rotation_speed(self.as_mut())
    }

    #[inline(always)]
    fn unk_07(&mut self) {
        IMovementState::unk_07(self.as_mut())
    }

    #[inline(always)]
    fn unk_08(&mut self) {
        IMovementState::unk_08(self.as_mut())
    }

    #[inline(always)]
    fn do_get_movement_rotation(&mut self, a_rotation: *mut NiPoint3) {
        IMovementState::do_get_movement_rotation(self.as_mut(), a_rotation)
    }

    #[inline(always)]
    fn unk_0a(&mut self) {
        IMovementState::unk_0a(self.as_mut())
    }

    #[inline(always)]
    fn unk_0b(&mut self) {
        IMovementState::unk_0b(self.as_mut())
    }

    #[inline(always)]
    fn unk_0c(&mut self) {
        IMovementState::unk_0c(self.as_mut())
    }

    #[inline(always)]
    fn unk_0d(&mut self) {
        IMovementState::unk_0d(self.as_mut())
    }

    #[inline(always)]
    fn unk_0e(&mut self) {
        IMovementState::unk_0e(self.as_mut())
    }

    #[inline(always)]
    fn unk_0f(&mut self) {
        IMovementState::unk_0f(self.as_mut())
    }

    #[inline(always)]
    fn unk_10(&mut self) {
        IMovementState::unk_10(self.as_mut())
    }

    #[inline(always)]
    fn unk_11(&mut self) {
        IMovementState::unk_11(self.as_mut())
    }

    #[inline(always)]
    fn unk_12(&mut self) {
        IMovementState::unk_12(self.as_mut())
    }

    #[inline(always)]
    fn unk_13(&mut self) {
        IMovementState::unk_13(self.as_mut())
    }
}
