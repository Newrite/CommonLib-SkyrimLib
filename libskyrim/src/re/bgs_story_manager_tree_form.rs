use crate::offsets::offsets_rtti::{RTTI_BGSStoryManagerTreeForm, RTTI_BGSStoryManagerTreeVisitor};
use crate::offsets::offsets_vtable::{
    VTABLE_BGSStoryManagerTreeForm, VTABLE_BGSStoryManagerTreeVisitor,
};
use crate::re::BGSStoryManagerBranchNode;
use crate::re::BGSStoryManagerQuestNode;
use crate::re::BSTArray;
use crate::re::PeriodicUpdateTimer;
use crate::re::TESCondition;
use crate::re::TESForm;
use crate::re::TESQuest;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::inherit;

pub type BGSStoryManagerVisitControl = i32;

/// C++ `RE::BGSStoryManagerTreeVisitor`
#[repr(C)]
pub struct BGSStoryManagerTreeVisitor {
    pub vtable: *const usize,                                    // 00
    pub timer: *mut PeriodicUpdateTimer,                         // 08
    pub current_cursor_depth: i32,                               // 10
    pub pad14: u32,                                              // 14
    pub last_quest_parent: *mut BGSStoryManagerQuestNode,        // 18
    pub cursor_ancestry: BSTArray<*mut BGSStoryManagerTreeForm>, // 20
    pub query_id: u32,                                           // 38
    pub pad3c: u32,                                              // 3C
}

const _: () = assert!(core::mem::size_of::<BGSStoryManagerTreeVisitor>() == 0x40);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerTreeVisitor, timer) == 0x08);
const _: () =
    assert!(core::mem::offset_of!(BGSStoryManagerTreeVisitor, current_cursor_depth) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerTreeVisitor, last_quest_parent) == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerTreeVisitor, cursor_ancestry) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerTreeVisitor, query_id) == 0x38);

impl RttiType for BGSStoryManagerTreeVisitor {
    const RTTI: VariantID = RTTI_BGSStoryManagerTreeVisitor;
}

impl BGSStoryManagerTreeVisitor {
    pub const RTTI: VariantID = RTTI_BGSStoryManagerTreeVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSStoryManagerTreeVisitor;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_VISIT_BRANCH_NODE: usize = 0x01;
        pub fn visit_branch_node(node: *mut BGSStoryManagerBranchNode) -> BGSStoryManagerVisitControl
    }

    virtual_method! {
        pub const VFUNC_VISIT_QUEST_NODE: usize = 0x02;
        pub fn visit_quest_node(node: *mut BGSStoryManagerQuestNode, arg2: bool) -> BGSStoryManagerVisitControl
    }

    virtual_method! {
        pub const VFUNC_VISIT_QUEST: usize = 0x03;
        pub fn visit_quest(quest: *mut TESQuest) -> BGSStoryManagerVisitControl
    }

    virtual_method! {
        pub const VFUNC_REVERT: usize = 0x04;
        pub fn revert()
    }
}

/// C++ `RE::BGSStoryManagerTreeForm`
#[repr(C)]
pub struct BGSStoryManagerTreeForm {
    pub base: TESForm,        // 00
    pub last_visitor_id: u32, // 20
    pub pad24: u32,           // 24
}

const _: () = assert!(core::mem::size_of::<BGSStoryManagerTreeForm>() == 0x28);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerTreeForm, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerTreeForm, last_visitor_id) == 0x20);

impl RttiType for BGSStoryManagerTreeForm {
    const RTTI: VariantID = RTTI_BGSStoryManagerTreeForm;
}

impl AsRef<BGSStoryManagerTreeForm> for BGSStoryManagerTreeForm {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BGSStoryManagerTreeForm> for BGSStoryManagerTreeForm {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(BGSStoryManagerTreeForm : TESForm);

impl BGSStoryManagerTreeForm {
    pub const RTTI: VariantID = RTTI_BGSStoryManagerTreeForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSStoryManagerTreeForm;

    virtual_method! {
        pub const VFUNC_Q_CHILD_COUNT: usize = 0x3B;
        pub fn q_child_count() -> u32
    }

    virtual_method! {
        pub const VFUNC_GET_CHILD: usize = 0x3C;
        pub fn get_child(idx: u32) -> *mut BGSStoryManagerTreeForm
    }

    virtual_method! {
        pub const VFUNC_Q_CONDITIONS: usize = 0x3D;
        pub fn q_conditions() -> *mut TESCondition
    }

    virtual_method! {
        pub const VFUNC_ACCEPT_VISITOR: usize = 0x3E;
        pub fn accept_visitor(visitor: &mut BGSStoryManagerTreeVisitor) -> BGSStoryManagerVisitControl
    }
}

pub trait BGSStoryManagerTreeFormExt {
    fn q_child_count(&self) -> u32;
    fn get_child(&self, idx: u32) -> *mut BGSStoryManagerTreeForm;
    fn q_conditions(&mut self) -> *mut TESCondition;
    fn accept_visitor(
        &mut self,
        visitor: &mut BGSStoryManagerTreeVisitor,
    ) -> BGSStoryManagerVisitControl;
}

impl<T: AsRef<BGSStoryManagerTreeForm> + AsMut<BGSStoryManagerTreeForm>> BGSStoryManagerTreeFormExt
    for T
{
    #[inline(always)]
    fn q_child_count(&self) -> u32 {
        self.as_ref().q_child_count()
    }

    #[inline(always)]
    fn get_child(&self, idx: u32) -> *mut BGSStoryManagerTreeForm {
        self.as_ref().get_child(idx)
    }

    #[inline(always)]
    fn q_conditions(&mut self) -> *mut TESCondition {
        BGSStoryManagerTreeForm::q_conditions(self.as_mut())
    }

    #[inline(always)]
    fn accept_visitor(
        &mut self,
        visitor: &mut BGSStoryManagerTreeVisitor,
    ) -> BGSStoryManagerVisitControl {
        BGSStoryManagerTreeForm::accept_visitor(self.as_mut(), visitor)
    }
}
