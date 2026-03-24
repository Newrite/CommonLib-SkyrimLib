use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSPerkRankArray;
use crate::offsets::offsets_vtable::VTABLE_BGSPerkRankArray;
use crate::re::BGSPerk;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PerkRankData {
    pub perk: *mut BGSPerk, // 00
    pub current_rank: i8,   // 08
    pub pad09: u8,          // 09
    pub pad0a: u16,         // 0A
    pub pad0c: u32,         // 0C
}

const _: () = assert!(core::mem::size_of::<PerkRankData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(PerkRankData, perk) == 0x00);
const _: () = assert!(core::mem::offset_of!(PerkRankData, current_rank) == 0x08);

impl Default for PerkRankData {
    fn default() -> Self {
        Self::new()
    }
}

impl PerkRankData {
    #[inline]
    pub const fn new() -> Self {
        Self {
            perk: core::ptr::null_mut(),
            current_rank: 0,
            pad09: 0,
            pad0a: 0,
            pad0c: 0,
        }
    }

    #[inline]
    pub const fn with_rank(perk: *mut BGSPerk, rank: i8) -> Self {
        Self {
            perk,
            current_rank: rank,
            pad09: 0,
            pad0a: 0,
            pad0c: 0,
        }
    }
}

#[repr(C)]
pub struct BGSPerkRankArray {
    pub base: BaseFormComponent,  // 00
    pub perks: *mut PerkRankData, // 08
    pub perk_count: u32,          // 10
    pub pad14: u32,               // 14
}

const _: () = assert!(core::mem::size_of::<BGSPerkRankArray>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSPerkRankArray, perks) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSPerkRankArray, perk_count) == 0x10);

impl RttiType for BGSPerkRankArray {
    const RTTI: VariantID = RTTI_BGSPerkRankArray;
}

impl AsRef<BGSPerkRankArray> for BGSPerkRankArray {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BGSPerkRankArray> for BGSPerkRankArray {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(BGSPerkRankArray : BaseFormComponent);

impl BGSPerkRankArray {
    pub const RTTI: VariantID = RTTI_BGSPerkRankArray;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSPerkRankArray;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;              // 01
    // void ClearDataComponent() override;                   // 02
    // void CopyComponent(BaseFormComponent* rhs) override;  // 03

    #[inline]
    pub fn perks_slice(&self) -> &[PerkRankData] {
        if self.perks.is_null() || self.perk_count == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.perks, self.perk_count as usize) }
        }
    }
}

pub trait BGSPerkRankArrayExt {
    fn perks_slice(&self) -> &[PerkRankData];
}

impl<T: AsRef<BGSPerkRankArray>> BGSPerkRankArrayExt for T {
    #[inline(always)]
    fn perks_slice(&self) -> &[PerkRankData] {
        self.as_ref().perks_slice()
    }
}
