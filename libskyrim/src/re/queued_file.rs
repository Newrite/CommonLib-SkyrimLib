use crate::offsets::offsets_rtti::RTTI_QueuedFile;
use crate::offsets::offsets_vtable::VTABLE_QueuedFile;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type QueuedFile; }

impl RttiType for QueuedFile {
    const RTTI: VariantID = RTTI_QueuedFile;
}

impl QueuedFile {
    pub const RTTI: VariantID = RTTI_QueuedFile;
    pub const VTABLE: &'static [VariantID] = &VTABLE_QueuedFile;
}
