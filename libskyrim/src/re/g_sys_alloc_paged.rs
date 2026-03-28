use crate::offsets::offsets_rtti::RTTI_GSysAllocPaged;
use crate::offsets::offsets_vtable::VTABLE_GSysAllocPaged;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    /// Pointer-compatible partial translation of `RE::GSysAllocPaged`.
    ///
    /// The current `GMemoryHeap` translation only needs `GSysAllocPaged` as a
    /// raw pointer parameter. Keep the pointee identity and RTTI surface here
    /// until a consumer needs the full allocator vtable and `GSysAllocBase`
    /// inheritance chain translated source-backed.
    pub type GSysAllocPaged;
}

impl RttiType for GSysAllocPaged {
    const RTTI: VariantID = RTTI_GSysAllocPaged;
}

impl GSysAllocPaged {
    pub const RTTI: VariantID = RTTI_GSysAllocPaged;
    pub const VTABLE: &'static [VariantID] = &VTABLE_GSysAllocPaged;

    // TODO: `GSysAllocPaged` is still a pointer-compatible stand-in. Translate
    // `GSysAllocBase` + `GSysAllocPaged` vtable surface when a consumer needs
    // more than raw pointer identity from this allocator chain.
}
