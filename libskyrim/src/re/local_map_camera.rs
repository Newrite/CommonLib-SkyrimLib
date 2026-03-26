use core::mem::MaybeUninit;

use core_util::inherit;

use crate::offsets::offsets_rtti::{RTTI_LocalMapCamera, RTTI_LocalMapCamera__DefaultState};
use crate::offsets::offsets_vtable::{VTABLE_LocalMapCamera, VTABLE_LocalMapCamera__DefaultState};
use crate::re::{
    BSTSmartPointer, BSTSmartPointerIntrusiveRefCountable, INISettingCollection, NiCamera,
    NiPoint3, NiPointer, TESCamera, TESCameraState,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

#[repr(C)]
pub struct LocalMapCameraDefaultState {
    pub base: TESCameraState,
    pub initial_position: NiPoint3,
    pub translation: NiPoint3,
    pub zoom: f32,
    pub min_frustum_half_width: f32,
    pub min_frustum_half_height: f32,
    pub pad44: u32,
}

const _: () = assert!(core::mem::size_of::<LocalMapCameraDefaultState>() == 0x48);
const _: () = assert!(core::mem::offset_of!(LocalMapCameraDefaultState, initial_position) == 0x20);
const _: () = assert!(core::mem::offset_of!(LocalMapCameraDefaultState, translation) == 0x2C);
const _: () = assert!(core::mem::offset_of!(LocalMapCameraDefaultState, zoom) == 0x38);
const _: () =
    assert!(core::mem::offset_of!(LocalMapCameraDefaultState, min_frustum_half_width) == 0x3C);
const _: () =
    assert!(core::mem::offset_of!(LocalMapCameraDefaultState, min_frustum_half_height) == 0x40);

impl RttiType for LocalMapCameraDefaultState {
    const RTTI: VariantID = RTTI_LocalMapCamera__DefaultState;
}

inherit!(LocalMapCameraDefaultState : TESCameraState, base);

impl BSTSmartPointerIntrusiveRefCountable for LocalMapCameraDefaultState {
    fn bst_inc_ref(&self) {
        self.base.bst_inc_ref();
    }
    fn bst_dec_ref(&self) -> u32 {
        self.base.bst_dec_ref()
    }
    unsafe fn bst_delete(&self) {
        unsafe { self.base.bst_delete() };
    }
}

impl LocalMapCameraDefaultState {
    pub const RTTI: VariantID = RTTI_LocalMapCamera__DefaultState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_LocalMapCamera__DefaultState;
}

#[repr(C)]
pub struct LocalMapCamera {
    pub base: TESCamera,
    pub max_extent: NiPoint3,
    pub min_extent: NiPoint3,
    pub default_state: BSTSmartPointer<LocalMapCameraDefaultState>,
    pub camera: NiPointer<NiCamera>,
    pub z_rotation: f32,
    pub pad64: u32,
}

const _: () = assert!(core::mem::size_of::<LocalMapCamera>() == 0x68);
const _: () = assert!(core::mem::offset_of!(LocalMapCamera, max_extent) == 0x38);
const _: () = assert!(core::mem::offset_of!(LocalMapCamera, min_extent) == 0x44);
const _: () = assert!(core::mem::offset_of!(LocalMapCamera, default_state) == 0x50);
const _: () = assert!(core::mem::offset_of!(LocalMapCamera, camera) == 0x58);
const _: () = assert!(core::mem::offset_of!(LocalMapCamera, z_rotation) == 0x60);

impl RttiType for LocalMapCamera {
    const RTTI: VariantID = RTTI_LocalMapCamera;
}

inherit!(LocalMapCamera : TESCamera, base);

impl LocalMapCamera {
    pub const RTTI: VariantID = RTTI_LocalMapCamera;
    pub const VTABLE: &'static [VariantID] = &VTABLE_LocalMapCamera;

    crate::relocation_func! {
        fn ctor_impl(this: *mut LocalMapCamera, z_rotation: f32) -> *mut LocalMapCamera => RelocationID::new(16084, 16325)
    }

    crate::relocation_func! {
        fn set_north_rotation_impl(this: *mut LocalMapCamera, north_rotation: f32) => RelocationID::new(16089, 16330)
    }

    pub fn new(z_rotation: f32) -> Self {
        let mut value = MaybeUninit::<Self>::zeroed();
        unsafe {
            Self::ctor_impl(value.as_mut_ptr(), z_rotation);
            value.assume_init()
        }
    }

    fn map_local_height() -> f32 {
        let ini = INISettingCollection::get_singleton();
        if ini.is_null() {
            return 0.0;
        }
        let setting = unsafe { (&*ini).get_setting("fMapLocalHeight:MapMenu") };
        if setting.is_null() {
            0.0
        } else {
            unsafe { (&*setting).get_float() }
        }
    }

    pub fn set_area_bounds(&mut self, max_extent: NiPoint3, min_extent: NiPoint3) {
        self.min_extent = min_extent;
        self.max_extent = max_extent;
        self.max_extent.z += Self::map_local_height();
    }

    pub fn set_default_state_initial_position(&mut self, position: NiPoint3) {
        let default_state = self.default_state.get();
        if default_state.is_null() {
            return;
        }
        unsafe {
            (*default_state).initial_position = position;
            (*default_state).initial_position.z += Self::map_local_height();
        }
    }

    pub fn set_default_state_min_frustum_dimensions(&mut self, width: f32, height: f32) {
        let default_state = self.default_state.get();
        if default_state.is_null() {
            return;
        }
        unsafe {
            (*default_state).min_frustum_half_width = width / 2.0;
            (*default_state).min_frustum_half_height = height / 2.0;
        }
    }

    pub fn set_default_state_translation(&mut self, x: f32, y: f32, z: f32) {
        let default_state = self.default_state.get();
        if default_state.is_null() {
            return;
        }
        unsafe {
            (*default_state).translation.x = x - (*default_state).initial_position.x;
            (*default_state).translation.y = y - (*default_state).initial_position.y;
            (*default_state).translation.z = z - (*default_state).initial_position.z;
        }
    }

    pub fn set_north_rotation(&mut self, north_rotation: f32) {
        Self::set_north_rotation_impl(self, north_rotation)
    }
}
