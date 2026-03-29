//! Pointer-only event payload surface for
//! `BSTEventSink<BSResource::ArchiveStreamOpenedEvent>`.

// TODO: SOURCE - replace this opaque payload with a real translation once a
// CommonLib header exposes `BSResource::ArchiveStreamOpenedEvent`; current
// source use is only a forward declaration in `RE/T/TES.h`.
core_util::abstract_type! {
    pub type ArchiveStreamOpenedEvent;
}
