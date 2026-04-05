//! User-facing event registration and dispatch helpers.
//!
//! The event SDK is intentionally split by runtime semantics instead of
//! pretending every event source in Skyrim and SKSE follows one universal
//! model. `BSTEventSource<T>`, input chains, SKSE messaging, and plugin-local
//! Rust buses each keep their own domain surface while sharing a common
//! authoring style.
//!
//! Planned layering:
//!
//! - `game`
//!   gameplay events owned by `ScriptEventSourceHolder`
//! - `ui`
//!   UI-owned `BSTEventSource<T>` registrations
//! - `input`
//!   `BSInputDeviceManager` and ergonomic input-chain wrappers
//! - `source`
//!   direct subscription helpers for already-known raw `BSTEventSource<T>*`
//! - `skse::dispatchers`
//!   SKSE dispatcher-backed `BSTEventSource<T>` surfaces such as
//!   `ActionEvent` or `ModCallbackEvent`
//! - `skse::messages`
//!   plugin messaging / lifecycle listener helpers
//! - `bus`
//!   Rust-local plugin event buses for intra-plugin orchestration
//!
//! Decision guide:
//!
//! - use `game` or `ui` when the plugin is subscribing to engine-owned
//!   `BSTEventSource<T>` families
//!   `game` goes through `ScriptEventSourceHolder`; `ui` goes through the
//!   `UI` singleton
//! - use `input` when the source of truth is an `InputEvent*` chain or device
//!   manager callback
//! - use `source` when plugin code already owns a raw `BSTEventSource<T>*`
//! - use `skse::dispatchers` for dispatcher-backed SKSE event families
//! - use `skse::messages` when the underlying primitive is SKSE messaging
//! - use `bus` for plugin-local Rust orchestration that should not cross the
//!   plugin boundary
//! - use `install` when several subscriptions should be named, retained, and
//!   installed together during bootstrap
//!
//! Typical plugin flow:
//!
//! 1. install engine/SKSE listeners through `game`, `ui`, `input`, or
//!    `skse::*`
//! 2. optionally fan those callbacks into plugin-local `bus` events
//! 3. retain the whole listener set through `install::EventBatch`
//!
//! Example local event bus:
//!
//! ```rust,ignore
//! use libskyrim::sdk::events::{Bus, EventFlow};
//!
//! #[derive(Default)]
//! struct HudEvent {
//!     visible: bool,
//! }
//!
//! let bus = Bus::<HudEvent>::new();
//! let _subscription = bus.subscribe(|event| {
//!     event.visible = true;
//!     EventFlow::Continue
//! });
//!
//! let result = bus.publish_default();
//! assert!(result.event().visible);
//! ```
//!
//! Example batch installation:
//!
//! ```rust,ignore
//! use libskyrim::sdk::events::{self, EventBatch};
//!
//! fn install_events(batch: &mut EventBatch<'_>) -> events::EventBatchInstallResult {
//!     events::install_all(batch, MyGameEvent, MyUiEvent)
//! }
//! ```

pub mod bus;
pub mod game;
pub mod input;
pub mod install;
pub mod skse;
pub mod source;
pub mod ui;

pub use bus::{Bus, BusSubscription, PublishResult, SubscriberPriority};
pub use input::InputEvents;
pub use install::{
    EventBatch, EventBatchError, EventBatchInstallResult, EventInstallFn, EventInstaller,
    install_batch_or_fatal, try_install_all,
};
pub use libskyrim_macros::{
    bus_event, dispatcher_event, game_event, input_event, message_event, ui_event,
};
pub use source::{
    EventFlow, EventInstallError, EventSourceExt, EventSourceRef, EventSubscription, IntoEventFlow,
};

#[macro_export]
macro_rules! __libskyrim_sdk_events_install_all {
    ($batch:expr, $($event:ident),+ $(,)?) => {{
        $crate::sdk::events::try_install_all(
            $batch,
            &[
                $(
                    {
                        $event::INSTALLER
                    }
                ),+
            ],
        )
    }};
}

pub use crate::__libskyrim_sdk_events_install_all as install_all;

#[macro_export]
macro_rules! __libskyrim_sdk_events_install_all_or_fatal {
    ($batch:expr, $($event:ident),+ $(,)?) => {{
        $crate::sdk::events::install_batch_or_fatal(
            $batch,
            &[
                $(
                    {
                        $event::INSTALLER
                    }
                ),+
            ],
        )
    }};
}

pub use crate::__libskyrim_sdk_events_install_all_or_fatal as install_all_or_fatal;

#[cfg(test)]
mod tests;
