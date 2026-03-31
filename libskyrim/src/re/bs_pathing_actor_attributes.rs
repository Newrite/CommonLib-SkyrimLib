use crate::re::BSPathingLockData;

/// C++ `RE::BSPathingActorAttributes`
#[repr(C)]
pub struct BSPathingActorAttributes {
    pub radius: f32, // 00
    pub height: f32, // 04
    // TODO: SOURCE - replace this raw pointer stand-in with
    // `BSTSmartPointer<BSPathingLockData>` after the lock-data intrusive
    // smart-pointer contract is translated.
    pub lock_data: *mut BSPathingLockData, // 08
    pub data: u32,                         // 10
    pub pad14: u32,                        // 14
}

const _: () = assert!(core::mem::size_of::<BSPathingActorAttributes>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BSPathingActorAttributes, radius) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSPathingActorAttributes, height) == 0x04);
const _: () = assert!(core::mem::offset_of!(BSPathingActorAttributes, lock_data) == 0x08);
const _: () = assert!(core::mem::offset_of!(BSPathingActorAttributes, data) == 0x10);
