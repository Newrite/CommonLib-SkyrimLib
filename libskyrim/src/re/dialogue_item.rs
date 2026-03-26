use core::mem::MaybeUninit;

use core_util::{EnumSet, inherit};

use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bs_intrusive_ref_counted::BSIntrusiveRefCounted;
use crate::re::bs_string::BSString;
use crate::re::bssimple_list::{BSSimpleList, BSSimpleListNode};
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::emotion_types::EmotionType;
use crate::re::extra_say_to_topic_info::ExtraSayToTopicInfo;
use crate::re::tes_idle_form::TESIdleForm;
use crate::re::tes_object_refr::TESObjectREFR;
use crate::re::tes_quest::TESQuest;
use crate::re::tes_topic::TESTopic;
use crate::re::tes_topic_info::TESTopicInfo;
use crate::relocation::RelocationID;

/// C++ `RE::DialogueResponse`
#[repr(C)]
pub struct DialogueResponse {
    pub text: BSString,                                 // 00
    pub anim_face_arch_type: EnumSet<EmotionType, u32>, // 10
    pub percent: u16,                                   // 14
    pub pad16: u16,                                     // 16
    pub voice: BSFixedString,                           // 18
    pub speaker_idle: *mut TESIdleForm,                 // 20
    pub listen_idle: *mut TESIdleForm,                  // 28
    pub voice_sound: *mut BGSSoundDescriptorForm,       // 30
    pub use_emotion: bool,                              // 38
    pub sound_lip: bool,                                // 39
    pub pad3a: u16,                                     // 3A
    pub pad3c: u32,                                     // 3C
}

const _: () = assert!(core::mem::size_of::<DialogueResponse>() == 0x40);

/// C++ `RE::DialogueItem`
#[repr(C)]
pub struct DialogueItem {
    pub base: BSIntrusiveRefCounted,                    // 00
    pub pad04: u32,                                     // 04
    pub responses: BSSimpleList<*mut DialogueResponse>, // 08
    pub current_response: *mut BSSimpleListNode<*mut DialogueResponse>, // 18
    pub info: *mut TESTopicInfo,                        // 20
    pub topic: *mut TESTopic,                           // 28
    pub quest: *mut TESQuest,                           // 30
    pub speaker: *mut TESObjectREFR,                    // 38
    pub extra_data: *mut ExtraSayToTopicInfo,           // 40
}

const _: () = assert!(core::mem::size_of::<DialogueItem>() == 0x48);
const _: () = assert!(core::mem::offset_of!(DialogueItem, responses) == 0x08);
const _: () = assert!(core::mem::offset_of!(DialogueItem, current_response) == 0x18);
const _: () = assert!(core::mem::offset_of!(DialogueItem, info) == 0x20);
const _: () = assert!(core::mem::offset_of!(DialogueItem, topic) == 0x28);
const _: () = assert!(core::mem::offset_of!(DialogueItem, quest) == 0x30);
const _: () = assert!(core::mem::offset_of!(DialogueItem, speaker) == 0x38);
const _: () = assert!(core::mem::offset_of!(DialogueItem, extra_data) == 0x40);

inherit!(DialogueItem : BSIntrusiveRefCounted);

impl BSTSmartPointerIntrusiveRefCountable for DialogueItem {
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
        let this = self as *const Self as *mut Self;
        unsafe {
            core::ptr::drop_in_place(this);
            crate::ffi::commonlib_free(this.cast());
        }
    }
}

impl DialogueItem {
    crate::relocation_func! {
        fn ctor(
            &mut self,
            quest: *mut TESQuest,
            topic: *mut TESTopic,
            topic_info: *mut TESTopicInfo,
            speaker: *mut TESObjectREFR
        ) -> *mut DialogueItem => RelocationID::new(34413, 35220)
    }

    pub fn new(
        quest: *mut TESQuest,
        topic: *mut TESTopic,
        topic_info: *mut TESTopicInfo,
        speaker: *mut TESObjectREFR,
    ) -> Self {
        let mut item = MaybeUninit::<Self>::zeroed();
        unsafe {
            (*item.as_mut_ptr()).ctor(quest, topic, topic_info, speaker);
            item.assume_init()
        }
    }
}
