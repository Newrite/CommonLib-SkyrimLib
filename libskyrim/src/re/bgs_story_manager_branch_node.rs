use bitflags::bitflags;

use crate::offsets::offsets_rtti::RTTI_BGSStoryManagerBranchNode;
use crate::offsets::offsets_vtable::VTABLE_BGSStoryManagerBranchNode;
use crate::re::BGSStoryManagerNodeBase;
use crate::re::BGSStoryManagerTreeForm;
use crate::re::BSTArray;
use crate::re::FormCastable;
use crate::re::FormType;
use crate::relocation::{RttiType, VariantID};
use core_util::inherit;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSStoryManagerBranchNodeRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::BGSStoryManagerBranchNode`
#[repr(C)]
pub struct BGSStoryManagerBranchNode {
    pub base: BGSStoryManagerNodeBase,                    // 00
    pub children: BSTArray<*mut BGSStoryManagerNodeBase>, // 48
}

const _: () = assert!(core::mem::size_of::<BGSStoryManagerBranchNode>() == 0x60);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerBranchNode, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSStoryManagerBranchNode, children) == 0x48);

impl RttiType for BGSStoryManagerBranchNode {
    const RTTI: VariantID = RTTI_BGSStoryManagerBranchNode;
}

impl FormCastable for BGSStoryManagerBranchNode {
    const TARGET_FORM_TYPE: FormType = FormType::StoryManagerBranchNode;
}

inherit!(BGSStoryManagerBranchNode : BGSStoryManagerNodeBase);

impl BGSStoryManagerBranchNode {
    pub const RTTI: VariantID = RTTI_BGSStoryManagerBranchNode;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSStoryManagerBranchNode;
    pub const FORMTYPE: FormType = FormType::StoryManagerBranchNode;

    // override (BGSStoryManagerNodeBase)
    // void ClearData() override;  // 05 - { BGSStoryManagerNodeBase::ClearData(); }
    // void InitItemImpl() override;  // 13
    // std::uint32_t QChildCount() const override;  // 3B - { return children.size(); }
    // BGSStoryManagerTreeForm* GetChild(std::uint32_t a_idx) const override;  // 3C - { return children[a_idx]; }
    // BGSStoryManagerTreeVisitor::VisitControl AcceptVisitor(BGSStoryManagerTreeVisitor& a_visitor) override;  // 3E - { return a_visitor.VisitBranchNode(*this); }

    #[inline]
    pub fn q_child_count(&self) -> u32 {
        self.children.len()
    }

    #[inline]
    pub fn get_child(&self, idx: u32) -> *mut BGSStoryManagerTreeForm {
        unsafe {
            if idx >= self.children.len() {
                core::ptr::null_mut()
            } else {
                *self.children.as_slice().get_unchecked(idx as usize)
                    as *mut BGSStoryManagerTreeForm
            }
        }
    }
}
