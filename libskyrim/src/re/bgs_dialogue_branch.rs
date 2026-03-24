use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_BGSDialogueBranch;
use crate::offsets::offsets_vtable::VTABLE_BGSDialogueBranch;
use crate::re::dialogue_types::DIALOGUE_TYPE;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_form::TESForm;
use crate::re::tes_quest::TESQuest;
use crate::re::tes_topic::TESTopic;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BGSDialogueBranch::Flag`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSDialogueBranchFlag {
    None = 0,
    TopLevel = 1 << 0,
    Blocking = 1 << 1,
    Exclusive = 1 << 2,
}

core_util::impl_enumset_type!(BGSDialogueBranchFlag => u32);

bitflags! {
    /// C++ `RE::BGSDialogueBranch::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSDialogueBranchRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::BGSDialogueBranch`
#[repr(C)]
pub struct BGSDialogueBranch {
    pub base: TESForm,                              // 00
    pub flags: EnumSet<BGSDialogueBranchFlag, u32>, // 20 - DNAM
    pub pad24: u32,                                 // 24
    pub quest: *mut TESQuest,                       // 28 - QNAM
    pub starting_topic: *mut TESTopic,              // 30 - SNAM
    pub dialogue_type: DIALOGUE_TYPE,               // 38 - TNAM
    pub pad3c: u32,                                 // 3C
}

const _: () = assert!(core::mem::size_of::<BGSDialogueBranch>() == 0x40);
const _: () = assert!(core::mem::offset_of!(BGSDialogueBranch, flags) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSDialogueBranch, quest) == 0x28);
const _: () = assert!(core::mem::offset_of!(BGSDialogueBranch, starting_topic) == 0x30);
const _: () = assert!(core::mem::offset_of!(BGSDialogueBranch, dialogue_type) == 0x38);

impl RttiType for BGSDialogueBranch {
    const RTTI: VariantID = RTTI_BGSDialogueBranch;
}

impl FormCastable for BGSDialogueBranch {
    const TARGET_FORM_TYPE: FormType = FormType::DialogueBranch;
}

inherit!(BGSDialogueBranch : TESForm);

impl BGSDialogueBranch {
    pub const RTTI: VariantID = RTTI_BGSDialogueBranch;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSDialogueBranch;
    pub const FORMTYPE: FormType = FormType::DialogueBranch;

    // override (TESForm)
    // void InitializeData() override;      // 04
    // void ClearData() override;           // 05
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13

    #[inline(always)]
    pub fn is_top_level(&self) -> bool {
        self.flags.any(BGSDialogueBranchFlag::TopLevel)
    }

    #[inline(always)]
    pub fn is_blocking(&self) -> bool {
        self.flags.any(BGSDialogueBranchFlag::Blocking)
    }

    #[inline(always)]
    pub fn is_exclusive(&self) -> bool {
        self.flags.any(BGSDialogueBranchFlag::Exclusive)
    }
}
