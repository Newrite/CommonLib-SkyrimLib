use crate::offsets::offsets_rtti::RTTI_BGSStoryTeller;
use crate::offsets::offsets_vtable::VTABLE_BGSStoryTeller;
use crate::re::TESQuest;
use crate::relocation::{RelocationID, RttiType, VariantID};

core_util::abstract_type! { pub type BGSStoryTeller; }

impl RttiType for BGSStoryTeller {
    const RTTI: VariantID = RTTI_BGSStoryTeller;
}

impl BGSStoryTeller {
    pub const RTTI: VariantID = RTTI_BGSStoryTeller;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSStoryTeller;

    crate::relocation_variable! {
        fn singleton() -> *mut BGSStoryTeller => RelocationID::new(514316, 400476), is_ptr
    }

    #[inline]
    pub fn get_singleton() -> *mut BGSStoryTeller {
        Self::singleton()
    }

    crate::relocation_func! {
        pub fn begin_shut_down_quest(&mut self, quest: *mut TESQuest) => RelocationID::new(31718, 32486)
    }

    crate::relocation_func! {
        pub fn begin_start_up_quest(&mut self, quest: *mut TESQuest) => RelocationID::new(31717, 32485)
    }
}
