use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSOutfit;
use crate::offsets::offsets_vtable::VTABLE_BGSOutfit;
use crate::re::bs_container::BSContainerForEachResult;
use crate::re::bst_array::BSTArray;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_form::TESForm;
use crate::relocation::{RttiType, VariantID};

bitflags! {
    /// C++ `RE::BGSOutfit::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSOutfitRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::BGSOutfit`
#[repr(C)]
pub struct BGSOutfit {
    pub base: TESForm,                        // 00
    pub outfit_items: BSTArray<*mut TESForm>, // 20 - INAM
}

const _: () = assert!(core::mem::size_of::<BGSOutfit>() == 0x38);
const _: () = assert!(core::mem::offset_of!(BGSOutfit, outfit_items) == 0x20);

impl RttiType for BGSOutfit {
    const RTTI: VariantID = RTTI_BGSOutfit;
}

impl FormCastable for BGSOutfit {
    const TARGET_FORM_TYPE: FormType = FormType::Outfit;
}

inherit!(BGSOutfit : TESForm);

impl BGSOutfit {
    pub const RTTI: VariantID = RTTI_BGSOutfit;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSOutfit;
    pub const FORMTYPE: FormType = FormType::Outfit;

    // override (TESForm)
    // void ClearData() override;           // 05
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13

    #[inline]
    pub fn outfit_items_slice(&self) -> &[*mut TESForm] {
        unsafe { self.outfit_items.as_slice() }
    }

    pub fn for_each_item<F>(&self, mut callback: F)
    where
        F: FnMut(*mut TESForm) -> BSContainerForEachResult,
    {
        for &item in self.outfit_items_slice() {
            if !item.is_null() && callback(item).is_stop() {
                return;
            }
        }
    }
}
