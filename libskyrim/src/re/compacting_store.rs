#![allow(non_snake_case)]

use crate::offsets::offsets_rtti::RTTI_CompactingStore__Store;
use crate::offsets::offsets_vtable::VTABLE_CompactingStore__Store;
use crate::relocation::{RttiType, VariantID};

pub mod CompactingStore {
    use super::*;

    // TODO: SOURCE - translate `CompactingStore.h` fully when callers need the
    // store internals or its `IMemoryStoreBase` virtual surface directly.
    // `MemoryManager` currently stores this only as an opaque engine pointer.
    core_util::abstract_type! { pub type Store; }

    impl RttiType for Store {
        const RTTI: VariantID = RTTI_CompactingStore__Store;
    }

    impl Store {
        pub const RTTI: VariantID = RTTI_CompactingStore__Store;
        pub const VTABLE: &'static [VariantID] = &VTABLE_CompactingStore__Store;
    }
}
