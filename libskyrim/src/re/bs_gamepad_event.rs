//! Pointer-only event payload surface for `BSTEventSource<BSGamepadEvent>`.

// TODO: CommonLib only forward-declares `BSGamepadEvent` from the input-device headers currently
// used here. Replace this opaque payload with a real matching translation once a source-backed
// header exposes the event fields.
core_util::abstract_type! {
    pub type BSGamepadEvent;
}
