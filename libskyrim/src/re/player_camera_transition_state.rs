use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_PlayerCameraTransitionState;
use crate::offsets::offsets_vtable::VTABLE_PlayerCameraTransitionState;
use crate::re::{BSTSmartPointer, NiPoint3, NiQuaternion, TESCameraState};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct PlayerCameraTransitionState {
    pub base: TESCameraState,
    pub unk20: u64,
    pub transition_from: *mut TESCameraState,
    pub transition_to: *mut TESCameraState,
    pub unk38: u16,
    pub pad3a: u16,
    pub pad3c: u32,
}

const _: () = assert!(core::mem::size_of::<PlayerCameraTransitionState>() == 0x40);

impl RttiType for PlayerCameraTransitionState {
    const RTTI: VariantID = RTTI_PlayerCameraTransitionState;
}

inherit!(PlayerCameraTransitionState : TESCameraState, base);

impl PlayerCameraTransitionState {
    pub const RTTI: VariantID = RTTI_PlayerCameraTransitionState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_PlayerCameraTransitionState;

    crate::virtual_method! {
        pub const VFUNC_BEGIN: usize = 0x01;
        pub fn begin(&mut self)
    }

    pub fn unk_03(&mut self) {
        if crate::runtime::is_vr() {
            crate::relocate_virtual!(
                extern "C" fn(*mut Self),
                self as *mut Self,
                VariantOffset::new_se_ae(0x0, 0x03)
            );
        }
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE: VariantOffset = VariantOffset::new_se_ae(0x03, 0x04);
        pub fn update(&mut self, next_state: &mut BSTSmartPointer<TESCameraState>)
    }
    crate::relocated_virtual_method! {
        pub const VFUNC_GET_ROTATION: VariantOffset = VariantOffset::new_se_ae(0x04, 0x05);
        pub fn get_rotation(&mut self, rotation: &mut NiQuaternion)
    }
    crate::relocated_virtual_method! {
        pub const VFUNC_GET_TRANSLATION: VariantOffset = VariantOffset::new_se_ae(0x05, 0x06);
        pub fn get_translation(&mut self, translation: &mut NiPoint3)
    }
}
