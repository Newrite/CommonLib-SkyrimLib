use crate::offsets::offsets_rtti::RTTI_CombatGroupDetectionListener;
use crate::offsets::offsets_vtable::VTABLE_CombatGroupDetectionListener;
use crate::re::DetectionListener;
use crate::relocation::{RttiType, VariantID};
use core_util::inherit;

/// C++ `RE::CombatGroupDetectionListener`
#[repr(C)]
pub struct CombatGroupDetectionListener {
    pub base: DetectionListener,       // 00
    pub unk10: *mut core::ffi::c_void, // 10
}

const _: () = assert!(core::mem::size_of::<CombatGroupDetectionListener>() == 0x18);
const _: () = assert!(core::mem::offset_of!(CombatGroupDetectionListener, unk10) == 0x10);

impl RttiType for CombatGroupDetectionListener {
    const RTTI: VariantID = RTTI_CombatGroupDetectionListener;
}

inherit!(CombatGroupDetectionListener : DetectionListener);

impl CombatGroupDetectionListener {
    pub const RTTI: VariantID = RTTI_CombatGroupDetectionListener;
    pub const VTABLE: &'static [VariantID] = &VTABLE_CombatGroupDetectionListener;

    crate::virtual_method! {
        pub const VFUNC_UNK_05: usize = 0x05;
        pub fn unk_05(&mut self)
    }
}
