use crate::offsets::offsets_rtti::RTTI_QueuedPromoteLocationReferencesTask;
use crate::offsets::offsets_vtable::VTABLE_QueuedPromoteLocationReferencesTask;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type QueuedPromoteLocationReferencesTask; }

impl RttiType for QueuedPromoteLocationReferencesTask {
    const RTTI: VariantID = RTTI_QueuedPromoteLocationReferencesTask;
}

impl QueuedPromoteLocationReferencesTask {
    pub const RTTI: VariantID = RTTI_QueuedPromoteLocationReferencesTask;
    pub const VTABLE: &'static [VariantID] = &VTABLE_QueuedPromoteLocationReferencesTask;
}
