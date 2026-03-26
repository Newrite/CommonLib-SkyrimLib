#![allow(non_camel_case_types)]

use core_util::EnumSet;

use crate::re::{BSSpinLock, ObjectRefHandle, TESPackage};

crate::core_util::abstract_type! { pub type ActorPackageData; }

/// C++ `RE::ActorPackage::ACTOR_PACKAGE_FLAG`
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ACTOR_PACKAGE_FLAG {
    None = 0,
    SaveLoadSharedPackage = 1 << 0,
}

core_util::impl_enumset_type!(ACTOR_PACKAGE_FLAG => i8);

/// C++ `RE::ActorPackage`
#[repr(C)]
pub struct ActorPackage {
    pub package_lock: BSSpinLock,                             // 00
    pub package: *mut TESPackage,                             // 08
    pub data: *mut ActorPackageData,                          // 10
    pub target: ObjectRefHandle,                              // 18
    pub current_procedure_index: i32,                         // 1C
    pub package_start_time: f32,                              // 20
    pub modified_package_flag: u32,                           // 24
    pub modified_interrupt_flag: u16,                         // 28
    pub actor_package_flags: EnumSet<ACTOR_PACKAGE_FLAG, i8>, // 2A
    pub preferred_speed: i8,                                  // 2B
    pub pad2c: u32,                                           // 2C
}

const _: () = assert!(core::mem::size_of::<ActorPackage>() == 0x30);
const _: () = assert!(core::mem::offset_of!(ActorPackage, package_lock) == 0x00);
const _: () = assert!(core::mem::offset_of!(ActorPackage, package) == 0x08);
const _: () = assert!(core::mem::offset_of!(ActorPackage, data) == 0x10);
const _: () = assert!(core::mem::offset_of!(ActorPackage, target) == 0x18);
const _: () = assert!(core::mem::offset_of!(ActorPackage, current_procedure_index) == 0x1C);
const _: () = assert!(core::mem::offset_of!(ActorPackage, package_start_time) == 0x20);
const _: () = assert!(core::mem::offset_of!(ActorPackage, modified_package_flag) == 0x24);
const _: () = assert!(core::mem::offset_of!(ActorPackage, modified_interrupt_flag) == 0x28);
const _: () = assert!(core::mem::offset_of!(ActorPackage, actor_package_flags) == 0x2A);
const _: () = assert!(core::mem::offset_of!(ActorPackage, preferred_speed) == 0x2B);
