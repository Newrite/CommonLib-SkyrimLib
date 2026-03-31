use crate::offsets::offsets_rtti::RTTI_hkWorldMemoryAvailableWatchDog;
use crate::offsets::offsets_vtable::VTABLE_hkWorldMemoryAvailableWatchDog;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type hkWorldMemoryAvailableWatchDog;
}

impl RttiType for hkWorldMemoryAvailableWatchDog {
    const RTTI: VariantID = RTTI_hkWorldMemoryAvailableWatchDog;
}

impl hkWorldMemoryAvailableWatchDog {
    pub const RTTI: VariantID = RTTI_hkWorldMemoryAvailableWatchDog;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkWorldMemoryAvailableWatchDog;
}

// TODO: SOURCE - CommonLibVR forward-declares this type from `hkpWorldCinfo.h`
// / `hkpWorld.h` but does not include a dedicated header in this snapshot.
// Keep the pointer target opaque until the owning Havok watchdog surface is
// available to translate.
