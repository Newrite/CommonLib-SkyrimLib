use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_FurnitureCameraState;
use crate::offsets::offsets_vtable::VTABLE_FurnitureCameraState;
use crate::re::{
    BGSLoadFormBuffer, BGSSaveFormBuffer, BSTSmartPointer, NiPoint3, NiQuaternion, TESCameraState,
};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct FurnitureCameraState {
    pub base: TESCameraState,
    pub unk20: u32,
    pub unk24: NiPoint3,
    pub unk30: u64,
    pub unk38: u32,
    pub unk3c: u16,
    pub unk3e: u16,
}

const _: () = assert!(core::mem::size_of::<FurnitureCameraState>() == 0x40);

impl RttiType for FurnitureCameraState {
    const RTTI: VariantID = RTTI_FurnitureCameraState;
}

inherit!(FurnitureCameraState : TESCameraState, base);

impl FurnitureCameraState {
    pub const RTTI: VariantID = RTTI_FurnitureCameraState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FurnitureCameraState;

    crate::virtual_method! { pub const VFUNC_BEGIN: usize = 0x01; pub fn begin(&mut self) }
    crate::virtual_method! { pub const VFUNC_END: usize = 0x02; pub fn end(&mut self) }

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
    crate::relocated_virtual_method! { pub const VFUNC_SAVE_GAME: VariantOffset = VariantOffset::new_se_ae(0x06, 0x07); pub fn save_game(&mut self, buf: *mut BGSSaveFormBuffer) }
    crate::relocated_virtual_method! { pub const VFUNC_LOAD_GAME: VariantOffset = VariantOffset::new_se_ae(0x07, 0x08); pub fn load_game(&mut self, buf: *mut BGSLoadFormBuffer) }
    crate::relocated_virtual_method! { pub const VFUNC_REVERT: VariantOffset = VariantOffset::new_se_ae(0x08, 0x09); pub fn revert(&mut self, buf: *mut BGSLoadFormBuffer) }
}
