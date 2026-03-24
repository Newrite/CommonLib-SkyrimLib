use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESSoulGem;
use crate::offsets::offsets_vtable::VTABLE_TESSoulGem;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::soul_levels::SOUL_LEVEL;
use crate::re::tes_form::TESForm;
use crate::re::tes_object_misc::TESObjectMISC;
use crate::relocation::{RttiType, VariantID};

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESSoulGemRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
        const CAN_HOLD_NPC_SOUL = 1 << 17;
    }
}

/// C++ `RE::TESSoulGem`
#[repr(C)]
pub struct TESSoulGem {
    pub base: TESObjectMISC,                    // 000
    pub linked_soul_gem: *mut TESSoulGem,       // 100
    pub current_soul: EnumSet<SOUL_LEVEL, u8>,  // 108
    pub soul_capacity: EnumSet<SOUL_LEVEL, u8>, // 109
    pub unk10a: u16,                            // 10A
    pub unk10c: u32,                            // 10C
}

const _: () = assert!(core::mem::size_of::<TESSoulGem>() == 0x110);
const _: () = assert!(core::mem::offset_of!(TESSoulGem, base) == 0x000);
const _: () = assert!(core::mem::offset_of!(TESSoulGem, linked_soul_gem) == 0x100);
const _: () = assert!(core::mem::offset_of!(TESSoulGem, current_soul) == 0x108);
const _: () = assert!(core::mem::offset_of!(TESSoulGem, soul_capacity) == 0x109);
const _: () = assert!(core::mem::offset_of!(TESSoulGem, unk10a) == 0x10A);
const _: () = assert!(core::mem::offset_of!(TESSoulGem, unk10c) == 0x10C);

impl RttiType for TESSoulGem {
    const RTTI: VariantID = RTTI_TESSoulGem;
}

impl FormCastable for TESSoulGem {
    const TARGET_FORM_TYPE: FormType = FormType::SoulGem;
}

inherit!(TESSoulGem : TESObjectMISC);

impl TESSoulGem {
    pub const RTTI: VariantID = RTTI_TESSoulGem;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESSoulGem;
    pub const FORMTYPE: FormType = FormType::SoulGem;

    // override (TESObjectMISC)
    // void InitializeData() override;                                   // 04
    // void LoadImpl(TESFile* a_mod, std::uint32_t a_chunkID) override;  // 54
    // void InitImpl() override;                                         // 55

    // override (BGSKeywordForm)
    // BGSKeyword* GetDefaultKeyword() const override;  // 05

    #[inline(always)]
    pub fn can_hold_npc_soul(&self) -> bool {
        let form = unsafe { &*(self as *const Self as *const TESForm) };
        (form.get_form_flags().bits() & TESSoulGemRecordFlags::CAN_HOLD_NPC_SOUL.bits()) != 0
    }

    #[inline(always)]
    pub fn get_contained_soul(&self) -> SOUL_LEVEL {
        self.current_soul.get().unwrap_or(SOUL_LEVEL::None)
    }

    #[inline(always)]
    pub fn get_maximum_capacity(&self) -> SOUL_LEVEL {
        self.soul_capacity.get().unwrap_or(SOUL_LEVEL::None)
    }
}
