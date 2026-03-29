use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ICellAttachDetachEventSource;
use crate::offsets::offsets_vtable::VTABLE_ICellAttachDetachEventSource;
use crate::re::{BSTEventSource, CellAttachDetachEvent};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ICellAttachDetachEventSource`
///
/// The overlapping empty `BSTSingletonExplicit<ICellAttachDetachEventSource>`
/// base is not modeled as fixed storage here; the source-backed concrete data
/// surface is the primary vptr plus `BSTEventSource<CellAttachDetachEvent>` at
/// offset `0x08`.
#[repr(C)]
pub struct ICellAttachDetachEventSource {
    pub vtable: *const usize,                                // 00
    pub event_source: BSTEventSource<CellAttachDetachEvent>, // 08
}

const _: () = assert!(core::mem::size_of::<ICellAttachDetachEventSource>() == 0x60);
const _: () = assert!(core::mem::offset_of!(ICellAttachDetachEventSource, event_source) == 0x08);

inherit!(
    ICellAttachDetachEventSource => BSTEventSource<CellAttachDetachEvent>,
    event_source
);

impl RttiType for ICellAttachDetachEventSource {
    const RTTI: VariantID = RTTI_ICellAttachDetachEventSource;
}

impl ICellAttachDetachEventSource {
    pub const RTTI: VariantID = RTTI_ICellAttachDetachEventSource;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ICellAttachDetachEventSource;

    // virtual ~ICellAttachDetachEventSource();  // 00
}
