use crate::offsets::offsets_rtti::RTTI_QueuedPromoteQuestTask;
use crate::offsets::offsets_vtable::VTABLE_QueuedPromoteQuestTask;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type QueuedPromoteQuestTask; }

impl RttiType for QueuedPromoteQuestTask {
    const RTTI: VariantID = RTTI_QueuedPromoteQuestTask;
}

impl QueuedPromoteQuestTask {
    pub const RTTI: VariantID = RTTI_QueuedPromoteQuestTask;
    pub const VTABLE: &'static [VariantID] = &VTABLE_QueuedPromoteQuestTask;
}
