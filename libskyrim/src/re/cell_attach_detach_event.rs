//! Pointer-only event payload surface for `BSTEventSource<CellAttachDetachEvent>`.

// TODO: SOURCE - replace this opaque payload with a real translation once a
// CommonLib header exposes `CellAttachDetachEvent`; current source use is only
// forward declarations in `RE/I/ICellAttachDetachEventSource.h` and
// `RE/P/Pathing.h`.
core_util::abstract_type! {
    pub type CellAttachDetachEvent;
}
