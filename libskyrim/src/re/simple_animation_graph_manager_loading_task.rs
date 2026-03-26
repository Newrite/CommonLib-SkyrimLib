use crate::offsets::offsets_rtti::RTTI_SimpleAnimationGraphManagerLoadingTask;
use crate::offsets::offsets_vtable::VTABLE_SimpleAnimationGraphManagerLoadingTask;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type SimpleAnimationGraphManagerLoadingTask; }

impl RttiType for SimpleAnimationGraphManagerLoadingTask {
    const RTTI: VariantID = RTTI_SimpleAnimationGraphManagerLoadingTask;
}

impl SimpleAnimationGraphManagerLoadingTask {
    pub const RTTI: VariantID = RTTI_SimpleAnimationGraphManagerLoadingTask;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SimpleAnimationGraphManagerLoadingTask;
}

// TODO: The vendored CommonLib tree only forward-declares
// `RE::SimpleAnimationGraphManagerLoadingTask` at this use-site. Replace this opaque stand-in
// with a real translation once its header or inheritance surface is available, then restore
// `NiPointer<SimpleAnimationGraphManagerLoadingTask>` on owners that currently use raw pointers.
