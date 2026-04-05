use crate::re::BSTEventSource;
use crate::skse;
use crate::skse::{
    ActionEvent, CameraEvent, CrosshairRefEvent, ModCallbackEvent, NiNodeUpdateEvent,
};

mod sealed {
    pub trait Sealed {}
}

/// Marker trait for SKSE event families exposed as dispatcher-backed
/// `BSTEventSource<T>` values.
///
/// This keeps the public `dispatchers` helpers limited to event types that are
/// actually retrievable through the SKSE API storage layer.
pub trait DispatcherEvent: Sized + sealed::Sealed {
    /// Returns the raw dispatcher-owned `BSTEventSource<Self>` pointer.
    fn source() -> *mut BSTEventSource<Self>;
}

impl sealed::Sealed for ModCallbackEvent {}
impl sealed::Sealed for CameraEvent {}
impl sealed::Sealed for CrosshairRefEvent {}
impl sealed::Sealed for ActionEvent {}
impl sealed::Sealed for NiNodeUpdateEvent {}

impl DispatcherEvent for ModCallbackEvent {
    #[inline(always)]
    fn source() -> *mut BSTEventSource<Self> {
        skse::get_mod_callback_event_source()
    }
}

impl DispatcherEvent for CameraEvent {
    #[inline(always)]
    fn source() -> *mut BSTEventSource<Self> {
        skse::get_camera_event_source()
    }
}

impl DispatcherEvent for CrosshairRefEvent {
    #[inline(always)]
    fn source() -> *mut BSTEventSource<Self> {
        skse::get_crosshair_ref_event_source()
    }
}

impl DispatcherEvent for ActionEvent {
    #[inline(always)]
    fn source() -> *mut BSTEventSource<Self> {
        skse::get_action_event_source()
    }
}

impl DispatcherEvent for NiNodeUpdateEvent {
    #[inline(always)]
    fn source() -> *mut BSTEventSource<Self> {
        skse::get_ni_node_update_event_source()
    }
}
