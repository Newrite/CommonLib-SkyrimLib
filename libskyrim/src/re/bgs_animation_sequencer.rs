use crate::re::{BSFixedString, BSTHashMap};

crate::core_util::abstract_type! { pub type BGSActionData; }

/// C++ `RE::BGSAnimationSequencer`
#[repr(C)]
pub struct BGSAnimationSequencer {
    pub num_sequences: u32,                                     // 00
    pub pad04: u32,                                             // 04
    pub actions: BSTHashMap<BSFixedString, *mut BGSActionData>, // 08
}

const _: () = assert!(core::mem::size_of::<BGSAnimationSequencer>() == 0x38);
const _: () = assert!(core::mem::offset_of!(BGSAnimationSequencer, num_sequences) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSAnimationSequencer, actions) == 0x08);
