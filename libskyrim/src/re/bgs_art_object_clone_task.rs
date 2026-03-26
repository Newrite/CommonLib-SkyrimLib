use crate::offsets::offsets_rtti::RTTI_BGSArtObjectCloneTask;
use crate::offsets::offsets_vtable::VTABLE_BGSArtObjectCloneTask;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type BGSArtObjectCloneTask; }

impl RttiType for BGSArtObjectCloneTask {
    const RTTI: VariantID = RTTI_BGSArtObjectCloneTask;
}

impl BGSArtObjectCloneTask {
    pub const RTTI: VariantID = RTTI_BGSArtObjectCloneTask;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSArtObjectCloneTask;
}

// TODO: The vendored CommonLib tree only exposes RTTI/VTABLE plus forward declarations for
// `RE::BGSArtObjectCloneTask`. Replace this opaque stand-in with a real translation once its
// header or inheritance surface is available, then restore smart-pointer fields that currently
// use raw-pointer stand-ins instead of `NiPointer` / `BSTSmartPointer`.
