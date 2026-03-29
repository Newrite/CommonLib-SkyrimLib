//! Pointer-only event payload surface for `BSTEventSink<BSSystemEvent>`.

// TODO: SOURCE - replace this opaque payload with a real translation once a
// CommonLib header exposes `BSSystemEvent`; current source use is only forward
// declarations such as `RE/T/TES.h` and `RE/B/BSSystemUtility.h`.
core_util::abstract_type! {
    pub type BSSystemEvent;
}
