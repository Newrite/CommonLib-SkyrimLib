//! High-level helpers for SKSE dispatcher-backed `BSTEventSource<T>`.
//!
//! This domain is intended for events retrieved through the SKSE API storage,
//! for example `ModCallbackEvent`, `CameraEvent`, `CrosshairRefEvent`,
//! `ActionEvent`, and `NiNodeUpdateEvent`.

use crate::re::BSTEventSource;
use crate::skse;

use crate::skse::{
    ActionEvent, CameraEvent, CrosshairRefEvent, ModCallbackEvent, NiNodeUpdateEvent,
};

use crate::sdk::events::source::{
    EventInstallError, EventSubscription, IntoEventFlow, prepend_static, subscribe_static,
};

mod sealed {
    pub trait Sealed {}
}

pub trait DispatcherEvent: Sized + sealed::Sealed {
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

#[inline(always)]
pub fn subscribe<E, F, R>(callback: F) -> Result<EventSubscription<'static, E>, EventInstallError>
where
    E: DispatcherEvent,
    F: FnMut(Option<&E>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { subscribe_static(E::source(), callback) }
}

#[inline(always)]
pub fn prepend<E, F, R>(callback: F) -> Result<EventSubscription<'static, E>, EventInstallError>
where
    E: DispatcherEvent,
    F: FnMut(Option<&E>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { prepend_static(E::source(), callback) }
}
