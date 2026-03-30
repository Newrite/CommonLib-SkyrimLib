use crate::offsets::offsets_rtti::RTTI_BSSmallBlockAllocator;
use crate::offsets::offsets_vtable::VTABLE_BSSmallBlockAllocator;
use crate::relocation::{RttiType, VariantID};

// TODO: SOURCE - translate `BSSmallBlockAllocator.h` fully when pool/page
// internals or allocation methods are needed outside pointer-typed owner
// fields. `MemoryManager` currently only stores a pointer to this engine-owned
// allocator, so an opaque marker is sufficient for the honest surface today.
core_util::abstract_type! { pub type BSSmallBlockAllocator; }

impl RttiType for BSSmallBlockAllocator {
    const RTTI: VariantID = RTTI_BSSmallBlockAllocator;
}

impl BSSmallBlockAllocator {
    pub const RTTI: VariantID = RTTI_BSSmallBlockAllocator;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSSmallBlockAllocator;
}
