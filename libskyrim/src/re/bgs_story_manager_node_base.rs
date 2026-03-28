use crate::offsets::offsets_rtti::RTTI_BGSStoryManagerNodeBase;
use crate::offsets::offsets_vtable::VTABLE_BGSStoryManagerNodeBase;
use crate::re::BGSStoryManagerBranchNode;
use crate::re::BGSStoryManagerTreeForm;
use crate::re::TESCondition;
use crate::relocation::{RttiType, VariantID};
use core_util::{EnumSet, inherit};

/// C++ `RE::BGSStoryManagerNodeBase::Flags::NodeFlag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoryManagerNodeFlag {
    None = 0,
    Random = 1 << 0,
    WarnIfNoChildQuestStarted = 1 << 1,
}

core_util::impl_enumset_type!(StoryManagerNodeFlag => u16);

/// C++ `RE::BGSStoryManagerNodeBase::Flags::QuestFlag`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoryManagerQuestFlag {
    None = 0,
    DoAllBeforeRepeating = 1 << 0,
    SharesEvent = 1 << 1,
    NumQuestsToRun = 1 << 2,
}

core_util::impl_enumset_type!(StoryManagerQuestFlag => u16);

/// C++ `RE::BGSStoryManagerNodeBase::Flags`
#[repr(C)]
pub struct StoryManagerNodeFlags {
    pub node_flags: EnumSet<StoryManagerNodeFlag, u16>, // 00
    pub quest_flags: EnumSet<StoryManagerQuestFlag, u16>, // 02
}

const _: () = assert!(core::mem::size_of::<StoryManagerNodeFlags>() == 0x4);
const _: () = assert!(core::mem::offset_of!(StoryManagerNodeFlags, node_flags) == 0x00);
const _: () = assert!(core::mem::offset_of!(StoryManagerNodeFlags, quest_flags) == 0x02);

/// C++ `RE::BGSStoryManagerNodeBase`
#[repr(C)]
pub struct BGSStoryManagerNodeBase {
    pub base: BGSStoryManagerTreeForm,                  // 00
    pub parent: *mut BGSStoryManagerBranchNode,         // 28
    pub previous_sibling: *mut BGSStoryManagerNodeBase, // 30
    pub max_quests: u32,                                // 38
    pub flags: StoryManagerNodeFlags,                   // 3C
    pub conditions: TESCondition,                       // 40
}

const _: () = assert!(core::mem::size_of::<BGSStoryManagerNodeBase>() == 0x48);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerNodeBase, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerNodeBase, parent) == 0x28);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerNodeBase, previous_sibling) == 0x30);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerNodeBase, max_quests) == 0x38);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerNodeBase, flags) == 0x3C);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerNodeBase, conditions) == 0x40);

impl RttiType for BGSStoryManagerNodeBase {
    const RTTI: VariantID = RTTI_BGSStoryManagerNodeBase;
}

impl AsRef<BGSStoryManagerNodeBase> for BGSStoryManagerNodeBase {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BGSStoryManagerNodeBase> for BGSStoryManagerNodeBase {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(BGSStoryManagerNodeBase : BGSStoryManagerTreeForm);

impl BGSStoryManagerNodeBase {
    pub const RTTI: VariantID = RTTI_BGSStoryManagerNodeBase;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSStoryManagerNodeBase;

    // override (BGSStoryManagerTreeForm)
    // void InitializeData() override;      // 04
    // void ClearData() override;           // 05
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13
    // TESCondition* QConditions() override;  // 3D - { return &conditions; }

    #[inline]
    pub fn q_conditions(&self) -> *mut TESCondition {
        core::ptr::addr_of!(self.conditions) as *mut TESCondition
    }

    #[inline]
    pub fn q_conditions_ref(&self) -> &TESCondition {
        &self.conditions
    }
}

pub trait BGSStoryManagerNodeBaseExt {
    fn q_conditions(&self) -> *mut TESCondition;
    fn q_conditions_ref(&self) -> &TESCondition;
}

impl<T: AsRef<BGSStoryManagerNodeBase>> BGSStoryManagerNodeBaseExt for T {
    #[inline(always)]
    fn q_conditions(&self) -> *mut TESCondition {
        BGSStoryManagerNodeBase::q_conditions(self.as_ref())
    }

    #[inline(always)]
    fn q_conditions_ref(&self) -> &TESCondition {
        BGSStoryManagerNodeBase::q_conditions_ref(self.as_ref())
    }
}
