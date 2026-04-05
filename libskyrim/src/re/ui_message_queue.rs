use crate::re::bst_hash_map::{BSTHashMap, UnkKey, UnkValue};
use crate::re::bst_message_queue::BSTCommonStaticMessageQueue;
use crate::re::bst_singleton::BSTSingletonSDM;
use crate::re::{BSFixedString, IUIMessageData, UIMessage, UIMessageType};
use crate::relocation::RelocationID;

/// C++ `RE::UIMessageQueue`
#[repr(C)]
pub struct UIMessageQueue {
    pub base: BSTSingletonSDM<UIMessageQueue>, // 000
    pub pad001: u8,                            // 001
    pub pad002: u16,                           // 002
    pub pad004: u32,                           // 004
    pub messages: BSTCommonStaticMessageQueue<*mut UIMessage, 100>, // 008
    pub unk348: BSTHashMap<UnkKey, UnkValue>,  // 348
    pub pool_used: u32,                        // 378
    pub pad37c: u32,                           // 37C
    pub message_pool: [UIMessage; 64],         // 380
}

const _: () = assert!(core::mem::size_of::<UIMessageQueue>() == 0xB80);
const _: () = assert!(core::mem::offset_of!(UIMessageQueue, base) == 0x000);
const _: () = assert!(core::mem::offset_of!(UIMessageQueue, messages) == 0x008);
const _: () = assert!(core::mem::offset_of!(UIMessageQueue, unk348) == 0x348);
const _: () = assert!(core::mem::offset_of!(UIMessageQueue, pool_used) == 0x378);
const _: () = assert!(core::mem::offset_of!(UIMessageQueue, message_pool) == 0x380);

core_util::inherit!(UIMessageQueue : BSTSingletonSDM<UIMessageQueue>);

impl UIMessageQueue {
    pub const POOL_SIZE: usize = 64;

    crate::relocation_variable! {
        fn singleton() -> *mut UIMessageQueue => RelocationID::new(514285, 400445), is_indirect_ptr
    }

    crate::relocation_func! {
        pub fn add_message(
            &mut self,
            menu_name: &BSFixedString,
            type_: UIMessageType,
            data: *mut IUIMessageData,
        ) => RelocationID::new(13530, 13631)
    }

    crate::relocation_func! {
        pub fn create_ui_message_data(&mut self, name: &BSFixedString) -> *mut IUIMessageData
            => RelocationID::new(80061, 82169)
    }

    crate::relocation_func! {
        pub fn process_commands(&mut self) => RelocationID::new(80059, 82167)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut UIMessageQueue {
        Self::singleton()
    }
}
