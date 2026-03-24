use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSNiAllocator;
use crate::offsets::offsets_vtable::VTABLE_BSNiAllocator;
use crate::re::ni_allocator::NiAllocator;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSNiAllocator`
#[repr(C)]
pub struct BSNiAllocator {
    pub base: NiAllocator, // 00
}

const _: () = assert!(core::mem::size_of::<BSNiAllocator>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSNiAllocator, base) == 0x0);

inherit!(BSNiAllocator : NiAllocator);

impl RttiType for BSNiAllocator {
    const RTTI: VariantID = RTTI_BSNiAllocator;
}

impl BSNiAllocator {
    pub const RTTI: VariantID = RTTI_BSNiAllocator;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSNiAllocator;

    // override (NiAllocator)
    // ~BSNiAllocator() override;  // 00
    // void* Allocate(...) override;  // 01
    // void Deallocate(...) override;  // 02
    // void* Reallocate(...) override;  // 03
    // bool TrackAllocate(...) override;  // 04
    // bool TrackDeallocate(...) override;  // 05
    // void Unk_06() override;  // 06
    // void Initialize() override;  // 07
    // void Shutdown() override;  // 08
    // bool VerifyAddress(...) override;  // 09
}
