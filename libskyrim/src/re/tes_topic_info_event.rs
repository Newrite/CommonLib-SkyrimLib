use core_util::EnumSet;

use crate::re::bs_core_types::FormID;
use crate::re::{NiPointer, TESObjectREFR, refr_event_callbacks};

/// C++ `RE::TESTopicInfoEvent::TopicInfoEventType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopicInfoEventType {
    TopicBegin = 0,
    TopicEnd = 1,
}

core_util::impl_enumset_type!(TopicInfoEventType => u32);

/// C++ `RE::TESTopicInfoEvent`
#[repr(C)]
pub struct TESTopicInfoEvent {
    // TODO: CommonLib stores `BSTSmartPointer<REFREventCallbacks::IEventCallback>` here.
    // This stays a raw pointer until `REFREventCallbacks::IEventCallback` has an honest
    // BSTSmartPointer intrusive-refcount bridge instead of the current pointer-only stub.
    pub callback: *mut refr_event_callbacks::IEventCallback, // 00
    pub speaker_ref: NiPointer<TESObjectREFR>,               // 08
    pub topic_info_form_id: FormID,                          // 10
    pub type_: EnumSet<TopicInfoEventType, u32>,             // 14
    pub stage: u16,                                          // 18
    pub pad1a: u16,                                          // 1A
    pub pad1c: u32,                                          // 1C
}

const _: () = assert!(core::mem::size_of::<TESTopicInfoEvent>() == 0x20);
