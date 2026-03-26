use crate::offsets::offsets_rtti::RTTI_PerkRankVisitor;
use crate::offsets::offsets_vtable::VTABLE_PerkRankVisitor;
use crate::re::PerkRankData;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::PerkRankVisitor`
#[repr(C)]
pub struct PerkRankVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<PerkRankVisitor>() == 0x8);

impl RttiType for PerkRankVisitor {
    const RTTI: VariantID = RTTI_PerkRankVisitor;
}

impl PerkRankVisitor {
    pub const RTTI: VariantID = RTTI_PerkRankVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_PerkRankVisitor;

    crate::virtual_method! {
        pub const VFUNC_CALL: usize = 0x00;
        pub fn call(entry: *const PerkRankData) -> bool
    }
}
