use crate::offsets::offsets_rtti::RTTI_TESCamera;
use crate::offsets::offsets_vtable::VTABLE_TESCamera;
use crate::re::{BSTPoint2, BSTPoint3, BSTSmartPointer, NiNode, NiPointer, TESCameraState};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::TESCamera`
#[repr(C)]
pub struct TESCamera {
    pub vtable: *const usize,                           // 00
    pub rotation_input: BSTPoint2<f32>,                 // 08
    pub translation_input: BSTPoint3<f32>,              // 10
    pub zoom_input: f32,                                // 1C
    pub camera_root: NiPointer<NiNode>,                 // 20
    pub current_state: BSTSmartPointer<TESCameraState>, // 28
    pub enabled: bool,                                  // 30
    pub pad31: u8,                                      // 31
    pub pad32: u16,                                     // 32
    pub pad34: u32,                                     // 34
}

const _: () = assert!(core::mem::size_of::<TESCamera>() == 0x38);
const _: () = assert!(core::mem::offset_of!(TESCamera, rotation_input) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESCamera, translation_input) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESCamera, zoom_input) == 0x1C);
const _: () = assert!(core::mem::offset_of!(TESCamera, camera_root) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESCamera, current_state) == 0x28);
const _: () = assert!(core::mem::offset_of!(TESCamera, enabled) == 0x30);

impl RttiType for TESCamera {
    const RTTI: VariantID = RTTI_TESCamera;
}

impl TESCamera {
    pub const RTTI: VariantID = RTTI_TESCamera;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESCamera;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_CAMERA_ROOT: usize = 0x01;
        fn set_camera_root_impl(&mut self, root: *mut NiNode)
    }

    crate::virtual_method! {
        pub const VFUNC_UPDATE: usize = 0x02;
        pub fn update(&mut self)
    }

    crate::relocation_func! {
        pub fn set_state_impl(this: &mut TESCamera, state: *mut TESCameraState) => RelocationID::new(32290, 33026)
    }

    #[inline(always)]
    pub fn set_camera_root(&mut self, root: NiPointer<NiNode>) {
        self.set_camera_root_ref(&root);
    }

    #[inline(always)]
    pub fn set_camera_root_ref(&mut self, root: &NiPointer<NiNode>) {
        self.set_camera_root_ptr(root.get());
    }

    #[inline(always)]
    pub fn set_camera_root_ptr(&mut self, root: *mut NiNode) {
        Self::set_camera_root_impl(self, root)
    }

    #[inline(always)]
    pub fn set_state(&mut self, state: *mut TESCameraState) {
        Self::set_state_impl(self, state)
    }
}

pub trait TESCameraExt {
    fn set_camera_root(&mut self, root: NiPointer<NiNode>);
    fn set_camera_root_ref(&mut self, root: &NiPointer<NiNode>);
    fn set_camera_root_ptr(&mut self, root: *mut NiNode);
    fn update(&mut self);
    fn set_state(&mut self, state: *mut TESCameraState);
}

impl<T> TESCameraExt for T
where
    T: AsRef<TESCamera> + AsMut<TESCamera>,
{
    #[inline(always)]
    fn set_camera_root(&mut self, root: NiPointer<NiNode>) {
        TESCamera::set_camera_root(self.as_mut(), root)
    }

    #[inline(always)]
    fn set_camera_root_ref(&mut self, root: &NiPointer<NiNode>) {
        TESCamera::set_camera_root_ref(self.as_mut(), root)
    }

    #[inline(always)]
    fn set_camera_root_ptr(&mut self, root: *mut NiNode) {
        TESCamera::set_camera_root_ptr(self.as_mut(), root)
    }

    #[inline(always)]
    fn update(&mut self) {
        TESCamera::update(self.as_mut())
    }

    #[inline(always)]
    fn set_state(&mut self, state: *mut TESCameraState) {
        TESCamera::set_state(self.as_mut(), state)
    }
}
