//! Pointer-only event payload surface for `BSTEventSource<BSRemoteGamepadEvent>`.

// TODO: CommonLib only forward-declares `BSRemoteGamepadEvent` from `BSInputDeviceManager.h`.
// Replace this opaque payload with a real matching translation once a source-backed header exposes
// the event fields.
core_util::abstract_type! {
    pub type BSRemoteGamepadEvent;
}
