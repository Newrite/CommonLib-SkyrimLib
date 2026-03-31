#![allow(non_camel_case_types)]

use core::ffi::c_void;

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_bhkWorld;
use crate::offsets::offsets_rtti::RTTI_bhkWorld;
use crate::offsets::offsets_vtable::VTABLE_bhkWorld;
use crate::re::bs_atomic::BSReadWriteLock;
use crate::re::{
    BGSAcousticSpaceListener, BSTArray, NiAVObject, bhkPickData, bhkSerializable, hkVector4,
    hkpSuspendInactiveAgentsUtil,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

crate::core_util::abstract_type! { pub type bhkConstraintProjector; }

/// C++ `RE::bhkWorld`
#[repr(C)]
pub struct bhkWorld {
    pub base: bhkSerializable,                                           // 0000
    pub unk0020: [u8; 0x320],                                            // 0020
    pub unk0340: [u8; 0x6400],                                           // 0340
    pub unk6740: [u8; 0x5DC0],                                           // 6740
    pub unk_c500: BSTArray<*mut c_void>,                                 // C500
    pub unk_c518: BSTArray<*mut c_void>,                                 // C518
    pub unk_c530: BSTArray<*mut c_void>,                                 // C530
    pub unk_c548: BSTArray<*mut c_void>,                                 // C548
    pub unk_c560: u64,                                                   // C560
    pub unk_c568: u32,                                                   // C568
    pub unk_c56c: f32,                                                   // C56C
    pub constraint_projector: *mut bhkConstraintProjector,               // C570
    pub unk_c578: u64,                                                   // C578
    pub unk_c580: u32,                                                   // C580
    pub unk_c584: f32,                                                   // C584
    pub unk_c588: u64,                                                   // C588
    pub unk_c590: u64,                                                   // C590
    pub world_lock: BSReadWriteLock,                                     // C598
    pub unk_c5a0: BSReadWriteLock,                                       // C5A0
    pub unk_c5a8: u64,                                                   // C5A8
    pub unk_c5b0: hkVector4,                                             // C5B0
    pub unk_c5c0: u64,                                                   // C5C0
    pub acoustic_space_listener: *mut BGSAcousticSpaceListener,          // C5C8
    pub suspend_inactive_agents_util: *mut hkpSuspendInactiveAgentsUtil, // C5D0
    pub unk_c5d8: u32,                                                   // C5D8
    pub unk_c5dc: u32,                                                   // C5DC
    pub unk_c5e0: u32,                                                   // C5E0
    pub unk_c5e4: u32,                                                   // C5E4
    pub unk_c5e8: u32,                                                   // C5E8
    pub unk_c5ec: u32,                                                   // C5EC
    pub tau: f32,                                                        // C5F0
    pub damping: f32,                                                    // C5F4
    pub unk_c5f8: u8,                                                    // C5F8
    pub toggle_collision: bool,                                          // C5F9
    pub unk_c5fa: u16,                                                   // C5FA
    pub unk_c5fc: u16,                                                   // C5FC
    pub unk_c5fe: u16,                                                   // C5FE
}

const _: () = assert!(core::mem::size_of::<bhkWorld>() == 0xC600);
const _: () = assert!(core::mem::offset_of!(bhkWorld, base) == 0x0000);
const _: () = assert!(core::mem::offset_of!(bhkWorld, constraint_projector) == 0xC570);
const _: () = assert!(core::mem::offset_of!(bhkWorld, world_lock) == 0xC598);
const _: () = assert!(core::mem::offset_of!(bhkWorld, acoustic_space_listener) == 0xC5C8);
const _: () = assert!(core::mem::offset_of!(bhkWorld, suspend_inactive_agents_util) == 0xC5D0);
const _: () = assert!(core::mem::offset_of!(bhkWorld, tau) == 0xC5F0);
const _: () = assert!(core::mem::offset_of!(bhkWorld, damping) == 0xC5F4);
const _: () = assert!(core::mem::offset_of!(bhkWorld, toggle_collision) == 0xC5F9);

impl RttiType for bhkWorld {
    const RTTI: VariantID = RTTI_bhkWorld;
}

inherit!(bhkWorld : bhkSerializable, base);

impl AsRef<bhkWorld> for bhkWorld {
    #[inline(always)]
    fn as_ref(&self) -> &bhkWorld {
        self
    }
}

impl AsMut<bhkWorld> for bhkWorld {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut bhkWorld {
        self
    }
}

impl bhkWorld {
    pub const RTTI: VariantID = RTTI_bhkWorld;
    pub const NI_RTTI: VariantID = NiRTTI_bhkWorld;
    pub const VTABLE: &'static [VariantID] = &VTABLE_bhkWorld;

    crate::virtual_method! {
        pub const VFUNC_UNK_32: usize = 0x32;
        pub fn unk_32(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_PICK_OBJECT: usize = 0x33;
        pub fn pick_object(&mut self, pick_data: &mut bhkPickData) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_34: usize = 0x34;
        pub fn unk_34(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_35: usize = 0x35;
        pub fn unk_35(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_HAVOK: usize = 0x36;
        pub fn init_havok(&mut self, scene_object: *mut NiAVObject, root: *mut NiAVObject)
    }

    crate::relocation_variable! {
        fn world_scale() -> &'static mut f32 => RelocationID::new(231896, 188105)
    }

    crate::relocation_variable! {
        fn world_scale_inverse() -> &'static mut f32 => RelocationID::new(230692, 187407)
    }

    #[inline(always)]
    pub fn get_world_scale() -> f32 {
        *Self::world_scale()
    }

    #[inline(always)]
    pub fn get_world_scale_inverse() -> f32 {
        *Self::world_scale_inverse()
    }

    #[inline(always)]
    pub fn pick_object_with(&mut self, pick_data: &mut bhkPickData) -> bool {
        Self::pick_object(self, pick_data)
    }
}

pub trait bhkWorldExt {
    fn unk_32(&mut self);
    fn pick_object(&mut self, pick_data: &mut bhkPickData) -> bool;
    fn unk_34(&mut self);
    fn unk_35(&mut self);
    fn init_havok(&mut self, scene_object: *mut NiAVObject, root: *mut NiAVObject);
    fn pick_object_with(&mut self, pick_data: &mut bhkPickData) -> bool;
}

impl<T: AsMut<bhkWorld>> bhkWorldExt for T {
    #[inline(always)]
    fn unk_32(&mut self) {
        bhkWorld::unk_32(self.as_mut())
    }

    #[inline(always)]
    fn pick_object(&mut self, pick_data: &mut bhkPickData) -> bool {
        bhkWorld::pick_object(self.as_mut(), pick_data)
    }

    #[inline(always)]
    fn unk_34(&mut self) {
        bhkWorld::unk_34(self.as_mut())
    }

    #[inline(always)]
    fn unk_35(&mut self) {
        bhkWorld::unk_35(self.as_mut())
    }

    #[inline(always)]
    fn init_havok(&mut self, scene_object: *mut NiAVObject, root: *mut NiAVObject) {
        bhkWorld::init_havok(self.as_mut(), scene_object, root)
    }

    #[inline(always)]
    fn pick_object_with(&mut self, pick_data: &mut bhkPickData) -> bool {
        bhkWorld::pick_object_with(self.as_mut(), pick_data)
    }
}
