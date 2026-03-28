use bitflags::bitflags;

use crate::offsets::offsets_rtti::RTTI_BGSStoryManagerQuestNode;
use crate::offsets::offsets_vtable::VTABLE_BGSStoryManagerQuestNode;
use crate::re::BGSStoryManagerNodeBase;
use crate::re::BGSStoryManagerTreeForm;
use crate::re::BSTArray;
use crate::re::BSTHashMap;
use crate::re::FormCastable;
use crate::re::FormType;
use crate::re::TESQuest;
use crate::relocation::{RttiType, VariantID};
use core_util::inherit;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSStoryManagerQuestNodeChangeFlags: u32 {
        const NONE = 0;
        const TIME_LAST_RUN = 1u32 << 31;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSStoryManagerQuestNodeRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::BGSStoryManagerQuestNode`
#[repr(C)]
pub struct BGSStoryManagerQuestNode {
    pub base: BGSStoryManagerNodeBase,                   // 00
    pub quests: BSTArray<*mut TESQuest>,                 // 48
    pub per_quest_flags: BSTHashMap<*mut TESQuest, u32>, // 60
    pub per_quest_hours_until_reset: BSTHashMap<*mut TESQuest, f32>, // 90
    pub num_quests_to_start: u32,                        // C0
    pub padc4: u32,                                      // C4
    pub children_last_run: BSTArray<f32>,                // C8
}

const _: () = assert!(core::mem::size_of::<BGSStoryManagerQuestNode>() == 0xE0);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerQuestNode, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerQuestNode, quests) == 0x48);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerQuestNode, per_quest_flags) == 0x60);
const _: () =
    assert!(core::mem::offset_of!(BGSStoryManagerQuestNode, per_quest_hours_until_reset) == 0x90);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerQuestNode, num_quests_to_start) == 0xC0);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerQuestNode, children_last_run) == 0xC8);

impl RttiType for BGSStoryManagerQuestNode {
    const RTTI: VariantID = RTTI_BGSStoryManagerQuestNode;
}

impl FormCastable for BGSStoryManagerQuestNode {
    const TARGET_FORM_TYPE: FormType = FormType::StoryManagerQuestNode;
}

inherit!(BGSStoryManagerQuestNode : BGSStoryManagerNodeBase);

impl BGSStoryManagerQuestNode {
    pub const RTTI: VariantID = RTTI_BGSStoryManagerQuestNode;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSStoryManagerQuestNode;
    pub const FORMTYPE: FormType = FormType::StoryManagerQuestNode;

    // override (BGSStoryManagerNodeBase)
    // void ClearData() override;  // 05
    // bool Load(TESFile* a_mod) override;  // 06
    // void SaveGame(BGSSaveFormBuffer* a_buf) override;  // 0E
    // void LoadGame(BGSLoadFormBuffer* a_buf) override;  // 0F
    // void Revert(BGSLoadFormBuffer* a_buf) override;  // 12
    // void InitItemImpl() override;  // 13
    // std::uint32_t QChildCount() const override;  // 3B - { return quests.size(); }
    // BGSStoryManagerTreeForm* GetChild(std::uint32_t a_idx) const override;  // 3C - { return quests[a_idx]; }
    // BGSStoryManagerTreeVisitor::VisitControl AcceptVisitor(BGSStoryManagerTreeVisitor& a_visitor) override;  // 3E - { return a_visitor.VisitQuestNode(*this, ...); }

    #[inline]
    pub fn q_child_count(&self) -> u32 {
        self.quests.len()
    }

    #[inline]
    pub fn get_child(&self, idx: u32) -> *mut BGSStoryManagerTreeForm {
        unsafe {
            if idx >= self.quests.len() {
                core::ptr::null_mut()
            } else {
                *self.quests.as_slice().get_unchecked(idx as usize) as *mut BGSStoryManagerTreeForm
            }
        }
    }

    #[inline]
    pub fn get_child_ref(&self, idx: u32) -> Option<&BGSStoryManagerTreeForm> {
        unsafe { self.get_child(idx).as_ref() }
    }
}
