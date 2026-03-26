#![allow(non_camel_case_types)]

use core_util::EnumSet;

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkpEntity;
use crate::offsets::offsets_vtable::VTABLE_hkpEntity;
use crate::re::{
    hkArray, hkLocalFrame, hkObjectIndex, hkRefPtr, hkSmallArray, hkpMaterial, hkpMaxSizeMotion,
    hkpWorldObject,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::hkpEntity::SpuCollisionCallbackEventFilter`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpEntitySpuCollisionCallbackEventFilter {
    None = 0,
    ContactPointAdded = 1 << 0,
    ContactPointProcess = 1 << 1,
    ContactPointRemoved = 1 << 2,
    ContactPointAddedOrProcess = (1 << 0) | (1 << 1),
}

core_util::impl_enumset_type!(hkpEntitySpuCollisionCallbackEventFilter => u8);

/// C++ `RE::hkpEntity::SmallArraySerializeOverrideType`
#[repr(C)]
pub struct hkpEntitySmallArraySerializeOverrideType {
    pub data: *mut core::ffi::c_void, // 00
    pub size: u16,                    // 08
    pub capacity_and_flags: u16,      // 0A
    pub pad0c: u32,                   // 0C
}

const _: () = assert!(core::mem::size_of::<hkpEntitySmallArraySerializeOverrideType>() == 0x10);

/// C++ `RE::hkpEntity::SpuCollisionCallback`
#[repr(C)]
pub struct hkpEntitySpuCollisionCallback {
    pub util: *mut core::ffi::c_void, // 00
    pub capacity: u16,                // 08
    pub event_filter: EnumSet<hkpEntitySpuCollisionCallbackEventFilter, u8>, // 0A
    pub user_filter: u8,              // 0B
    pub pad0c: u32,                   // 0C
}

const _: () = assert!(core::mem::size_of::<hkpEntitySpuCollisionCallback>() == 0x10);

/// C++ `RE::hkpEntity::ExtendedListeners`
#[repr(C)]
pub struct hkpEntityExtendedListeners {
    pub activation_listeners: hkSmallArray<*mut core::ffi::c_void>, // 00
    pub entity_listeners: hkSmallArray<*mut core::ffi::c_void>,     // 10
}

const _: () = assert!(core::mem::size_of::<hkpEntityExtendedListeners>() == 0x20);

/// C++ `RE::hkpEntity`
#[repr(C)]
pub struct hkpEntity {
    pub base: hkpWorldObject,                                        // 00
    pub material: hkpMaterial,                                       // 0D0
    pub pad0dc: u32,                                                 // 0DC
    pub limit_contact_impulse_util_and_flag: *mut core::ffi::c_void, // 0E0
    pub damage_multiplier: f32,                                      // 0E8
    pub pad0ec: u32,                                                 // 0EC
    pub breakable_body: *mut core::ffi::c_void,                      // 0F0
    pub solver_data: u32,                                            // 0F8
    pub storage_index: hkObjectIndex,                                // 0FC
    pub contact_point_callback_delay: u16,                           // 0FE
    pub constraints_master: hkSmallArray<*mut core::ffi::c_void>,    // 100
    pub constraints_slave: hkArray<*mut core::ffi::c_void>,          // 110
    pub constraint_runtime: hkArray<u8>,                             // 120
    pub simulation_island: *mut core::ffi::c_void,                   // 130
    pub auto_remove_level: i8,                                       // 138
    pub num_shape_keys_in_contact_point_properties: u8,              // 139
    pub response_modifier_flags: u8,                                 // 13A
    pub pad13b: u8,                                                  // 13B
    pub uid: u32,                                                    // 13C
    pub spu_collision_callback: hkpEntitySpuCollisionCallback,       // 140
    pub motion: hkpMaxSizeMotion,                                    // 150
    pub contact_listeners: hkSmallArray<*mut core::ffi::c_void>,     // 290
    pub actions: hkSmallArray<*mut core::ffi::c_void>,               // 2A0
    pub local_frame: hkRefPtr<hkLocalFrame>,                         // 2B0
    pub extended_listeners: *mut hkpEntityExtendedListeners,         // 2B8
    pub np_data: u32,                                                // 2C0
    pub pad2c4: u32,                                                 // 2C4
    pub pad2c8: u64,                                                 // 2C8
}

const _: () = assert!(core::mem::size_of::<hkpEntity>() == 0x2D0);
const _: () = assert!(core::mem::offset_of!(hkpEntity, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpEntity, material) == 0x0D0);
const _: () = assert!(core::mem::offset_of!(hkpEntity, constraints_master) == 0x100);
const _: () = assert!(core::mem::offset_of!(hkpEntity, motion) == 0x150);
const _: () = assert!(core::mem::offset_of!(hkpEntity, contact_listeners) == 0x290);
const _: () = assert!(core::mem::offset_of!(hkpEntity, local_frame) == 0x2B0);
const _: () = assert!(core::mem::offset_of!(hkpEntity, extended_listeners) == 0x2B8);

impl RttiType for hkpEntity {
    const RTTI: VariantID = RTTI_hkpEntity;
}

inherit!(hkpEntity : hkpWorldObject, base);

impl AsRef<crate::re::hkReferencedObject> for hkpEntity {
    #[inline(always)]
    fn as_ref(&self) -> &crate::re::hkReferencedObject {
        self.base.as_ref()
    }
}

impl AsMut<crate::re::hkReferencedObject> for hkpEntity {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut crate::re::hkReferencedObject {
        self.base.as_mut()
    }
}

impl hkpEntity {
    pub const RTTI: VariantID = RTTI_hkpEntity;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpEntity;

    crate::virtual_method! {
        pub const VFUNC_DEALLOCATE_INTERNAL_ARRAYS: usize = 0x06;
        pub fn deallocate_internal_arrays(&mut self)
    }

    crate::relocation_func! {
        pub fn add_contact_listener(&mut self, listener: *mut core::ffi::c_void) => RelocationID::new(60094, 60844)
    }

    crate::relocation_func! {
        pub fn remove_contact_listener(&mut self, listener: *mut core::ffi::c_void) => RelocationID::new(60095, 60845)
    }

    crate::relocation_func! {
        pub fn activate(&mut self) => RelocationID::new(60096, 60849)
    }
}

pub trait hkpEntityExt {
    fn deallocate_internal_arrays(&mut self);
    fn add_contact_listener(&mut self, listener: *mut core::ffi::c_void);
    fn remove_contact_listener(&mut self, listener: *mut core::ffi::c_void);
    fn activate(&mut self);
}

impl<T: AsMut<hkpEntity>> hkpEntityExt for T {
    #[inline(always)]
    fn deallocate_internal_arrays(&mut self) {
        hkpEntity::deallocate_internal_arrays(self.as_mut())
    }

    #[inline(always)]
    fn add_contact_listener(&mut self, listener: *mut core::ffi::c_void) {
        hkpEntity::add_contact_listener(self.as_mut(), listener)
    }

    #[inline(always)]
    fn remove_contact_listener(&mut self, listener: *mut core::ffi::c_void) {
        hkpEntity::remove_contact_listener(self.as_mut(), listener)
    }

    #[inline(always)]
    fn activate(&mut self) {
        hkpEntity::activate(self.as_mut())
    }
}
