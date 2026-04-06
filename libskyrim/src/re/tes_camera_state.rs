use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESCameraState;
use crate::offsets::offsets_vtable::VTABLE_TESCameraState;
use crate::re::{
    BGSLoadFormBuffer, BGSSaveFormBuffer, BSIntrusiveRefCounted, BSTSmartPointer,
    BSTSmartPointerIntrusiveRefCountable, CameraState, NiPoint3, NiQuaternion, TESCamera,
};
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::TESCameraState`
#[repr(C)]
pub struct TESCameraState {
    pub vtable: *const usize,        // 00
    pub base: BSIntrusiveRefCounted, // 08
    pub pad0c: u32,                  // 0C
    pub camera: *mut TESCamera,      // 10
    pub id: CameraState,             // 18
    pub pad1c: u32,                  // 1C
}

const _: () = assert!(core::mem::size_of::<TESCameraState>() == 0x20);
const _: () = assert!(core::mem::offset_of!(TESCameraState, base) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESCameraState, camera) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESCameraState, id) == 0x18);

impl RttiType for TESCameraState {
    const RTTI: VariantID = RTTI_TESCameraState;
}

inherit!(TESCameraState : BSIntrusiveRefCounted, base);

impl BSTSmartPointerIntrusiveRefCountable for TESCameraState {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        unsafe {
            (*(self as *const Self as *mut Self)).dtor();
        }
    }
}

impl TESCameraState {
    pub const RTTI: VariantID = RTTI_TESCameraState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESCameraState;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_BEGIN: usize = 0x01;
        pub fn begin(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_END: usize = 0x02;
        pub fn end(&mut self)
    }

    #[inline(always)]
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
        pub fn get_rotation(&self, rotation: &mut NiQuaternion)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_TRANSLATION: VariantOffset = VariantOffset::new_se_ae(0x05, 0x06);
        pub fn get_translation(&self, translation: &mut NiPoint3)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SAVE_GAME: VariantOffset = VariantOffset::new_se_ae(0x06, 0x07);
        pub fn save_game(&mut self, buf: *mut BGSSaveFormBuffer)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_LOAD_GAME: VariantOffset = VariantOffset::new_se_ae(0x07, 0x08);
        pub fn load_game(&mut self, buf: *mut BGSLoadFormBuffer)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_REVERT: VariantOffset = VariantOffset::new_se_ae(0x08, 0x09);
        pub fn revert(&mut self, buf: *mut BGSLoadFormBuffer)
    }
}

pub trait TESCameraStateExt {
    fn dtor(&mut self);
    fn begin(&mut self);
    fn end(&mut self);
    fn unk_03(&mut self);
    fn update(&mut self, next_state: &mut BSTSmartPointer<TESCameraState>);
    fn get_rotation(&self, rotation: &mut NiQuaternion);
    fn get_translation(&self, translation: &mut NiPoint3);
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer);
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer);
    fn revert(&mut self, buf: *mut BGSLoadFormBuffer);
}

impl<T> TESCameraStateExt for T
where
    T: AsRef<TESCameraState> + AsMut<TESCameraState>,
{
    #[inline(always)]
    fn dtor(&mut self) {
        TESCameraState::dtor(self.as_mut())
    }

    #[inline(always)]
    fn begin(&mut self) {
        TESCameraState::begin(self.as_mut())
    }

    #[inline(always)]
    fn end(&mut self) {
        TESCameraState::end(self.as_mut())
    }

    #[inline(always)]
    fn unk_03(&mut self) {
        TESCameraState::unk_03(self.as_mut())
    }

    #[inline(always)]
    fn update(&mut self, next_state: &mut BSTSmartPointer<TESCameraState>) {
        TESCameraState::update(self.as_mut(), next_state)
    }

    #[inline(always)]
    fn get_rotation(&self, rotation: &mut NiQuaternion) {
        TESCameraState::get_rotation(self.as_ref(), rotation)
    }

    #[inline(always)]
    fn get_translation(&self, translation: &mut NiPoint3) {
        TESCameraState::get_translation(self.as_ref(), translation)
    }

    #[inline(always)]
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer) {
        TESCameraState::save_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        TESCameraState::load_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn revert(&mut self, buf: *mut BGSLoadFormBuffer) {
        TESCameraState::revert(self.as_mut(), buf)
    }
}
