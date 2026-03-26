use core_util::EnumSet;

use crate::re::USER_EVENT_FLAG;

/// C++ `RE::UserEventEnabled`
#[repr(C)]
pub struct UserEventEnabled {
    pub new_user_event_flag: EnumSet<USER_EVENT_FLAG, u32>, // 00
    pub old_user_event_flag: EnumSet<USER_EVENT_FLAG, u32>, // 04
}

const _: () = assert!(core::mem::size_of::<UserEventEnabled>() == 0x08);
const _: () = assert!(core::mem::offset_of!(UserEventEnabled, new_user_event_flag) == 0x00);
const _: () = assert!(core::mem::offset_of!(UserEventEnabled, old_user_event_flag) == 0x04);
