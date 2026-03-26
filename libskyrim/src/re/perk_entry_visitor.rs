use crate::offsets::offsets_rtti::RTTI_PerkEntryVisitor;
use crate::offsets::offsets_vtable::VTABLE_PerkEntryVisitor;
use crate::re::{BGSPerkEntry, BSContainerForEachResult};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::PerkEntryVisitor`
#[repr(C)]
pub struct PerkEntryVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<PerkEntryVisitor>() == 0x8);

impl RttiType for PerkEntryVisitor {
    const RTTI: VariantID = RTTI_PerkEntryVisitor;
}

impl PerkEntryVisitor {
    pub const RTTI: VariantID = RTTI_PerkEntryVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_PerkEntryVisitor;

    crate::virtual_method! {
        pub const VFUNC_VISIT: usize = 0x00;
        pub fn visit(perk_entry: *mut BGSPerkEntry) -> BSContainerForEachResult
    }
}

pub trait PerkEntryVisitorExt {
    fn visit(&mut self, perk_entry: *mut BGSPerkEntry) -> BSContainerForEachResult;
}

impl<T: AsMut<PerkEntryVisitor>> PerkEntryVisitorExt for T {
    #[inline(always)]
    fn visit(&mut self, perk_entry: *mut BGSPerkEntry) -> BSContainerForEachResult {
        PerkEntryVisitor::visit(self.as_mut(), perk_entry)
    }
}
