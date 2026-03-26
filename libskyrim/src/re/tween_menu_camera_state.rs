use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TweenMenuCameraState;
use crate::offsets::offsets_vtable::VTABLE_TweenMenuCameraState;
use crate::re::{BSTSmartPointer, NiPoint2, NiPoint3, NiQuaternion, TESCameraState};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct TweenMenuCameraState {
    pub base: TESCameraState,
    pub initial_rotation: NiQuaternion,
    pub initial_fov: f32,
    pub current_added_rot: NiPoint2,
    pub target_added_rot: NiPoint2,
    pub current_added_fov: f32,
    pub target_added_fov: f32,
    pub ending_state: bool,
    pub pad4d: u8,
    pub pad4e: u16,
}

const _: () = assert!(core::mem::size_of::<TweenMenuCameraState>() == 0x50);

impl RttiType for TweenMenuCameraState {
    const RTTI: VariantID = RTTI_TweenMenuCameraState;
}

inherit!(TweenMenuCameraState : TESCameraState, base);

impl TweenMenuCameraState {
    pub const RTTI: VariantID = RTTI_TweenMenuCameraState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TweenMenuCameraState;

    crate::virtual_method! { pub const VFUNC_BEGIN: usize = 0x01; pub fn begin() }
    crate::virtual_method! { pub const VFUNC_END: usize = 0x02; pub fn end() }

    pub fn unk_03(&mut self) {
        if crate::runtime::is_vr() {
            crate::relocate_virtual!(
                extern "C" fn(*mut Self),
                self as *mut Self,
                VariantOffset::new_se_ae(0x0, 0x03)
            );
        }
    }

    crate::relocated_virtual_method! { pub const VFUNC_UPDATE: VariantOffset = VariantOffset::new_se_ae(0x03, 0x04); pub fn update(&mut self, next_state: &mut BSTSmartPointer<TESCameraState>) }
    crate::relocated_virtual_method! { pub const VFUNC_GET_ROTATION: VariantOffset = VariantOffset::new_se_ae(0x04, 0x05); pub fn get_rotation(&mut self, rotation: &mut NiQuaternion) }
    crate::relocated_virtual_method! { pub const VFUNC_GET_TRANSLATION: VariantOffset = VariantOffset::new_se_ae(0x05, 0x06); pub fn get_translation(&mut self, translation: &mut NiPoint3) }
}
