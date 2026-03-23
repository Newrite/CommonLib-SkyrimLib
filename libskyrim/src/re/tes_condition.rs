use crate::re::TESForm;
use crate::re::TESObjectREFR;
use crate::relocation::RelocationID;

core_util::abstract_type! {
    pub type TESConditionItem;
}

#[repr(C)]
pub struct TESCondition {
    pub head: *mut TESConditionItem, // 0x00
}

const _: () = assert!(core::mem::size_of::<TESCondition>() == 0x8);

impl Default for TESCondition {
    fn default() -> Self {
        Self::new()
    }
}

impl TESCondition {
    pub const fn new() -> Self {
        Self {
            head: core::ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn as_bool(&self) -> bool {
        !self.is_empty()
    }

    pub fn evaluate(&self, action_ref: *mut TESObjectREFR, target_ref: *mut TESObjectREFR) -> bool {
        self.is_true(action_ref, target_ref)
    }

    crate::relocation_func! {
        pub fn copy(&mut self, other: *const TESCondition, arg2: *mut TESForm) => RelocationID::new(29067, 29879)
    }

    crate::relocation_func! {
        pub fn is_true(&self, action_ref: *mut TESObjectREFR, target_ref: *mut TESObjectREFR) -> bool => RelocationID::new(29074, 29888)
    }
}
