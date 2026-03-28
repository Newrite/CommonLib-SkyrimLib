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
mod tests {
    use crate::re::{MenuOpenCloseEvent, TESHitEvent};
    use crate::sdk::core::GameRef;
    use crate::sdk::events::{self, Bus, EventFlow, InputEvents};
    use crate::skse::ActionEvent;
    use alloc::boxed::Box;
    use spin::Once;

    fn sdk_test_bus() -> &'static Bus<u32> {
        static PTR: Once<usize> = Once::new();
        let ptr = *PTR.call_once(|| Box::into_raw(Box::new(Bus::<u32>::new())) as usize);
        unsafe { &*(ptr as *const Bus<u32>) }
    }

    #[crate::sdk::events::game_event(event = crate::re::TESHitEvent)]
    fn sdk_game_example(_event: Option<&TESHitEvent>) -> EventFlow {
        EventFlow::Continue
    }

    #[crate::sdk::events::ui_event(event = crate::re::MenuOpenCloseEvent, prepend)]
    fn sdk_ui_example(_event: &MenuOpenCloseEvent) {}

    #[crate::sdk::events::dispatcher_event(event = crate::skse::ActionEvent)]
    fn sdk_dispatcher_example(_event: GameRef<'_, ActionEvent>) -> EventFlow {
        EventFlow::Continue
    }

    #[crate::sdk::events::input_event(prepend)]
    fn sdk_input_example(_events: InputEvents<'_>) {}

    #[crate::sdk::events::message_event(
        kind = crate::sdk::events::skse::messages::MessageKind::DataLoaded
    )]
    fn sdk_message_example(_message: crate::sdk::events::skse::messages::MessageRef<'_>) {}

    #[crate::sdk::events::message_event(
        plugin_phase = crate::sdk::core::PluginLifecyclePhase::DataLoaded,
        sender = "SKSE"
    )]
    fn sdk_message_phase_example(_message: crate::sdk::events::skse::messages::MessageRef<'_>) {}

    #[crate::sdk::events::bus_event(bus = sdk_test_bus(), early)]
    fn sdk_bus_mut_example(value: &mut u32) -> EventFlow {
        *value += 1;
        EventFlow::Continue
    }

    #[crate::sdk::events::bus_event(
        bus = sdk_test_bus(),
        priority = crate::sdk::events::SubscriberPriority::LAST
    )]
    fn sdk_bus_ref_example(_value: &u32) {}

    #[test]
    fn attribute_event_modules_compile() {
        let _ = sdk_game_example_event::try_install
            as for<'a> fn(&mut events::EventBatch<'a>) -> Result<(), events::EventBatchError>;
        let _ = sdk_game_example_event::INSTALLER;

        let _ = sdk_ui_example_event::try_install
            as for<'a> fn(&mut events::EventBatch<'a>) -> Result<(), events::EventBatchError>;
        let _ = sdk_ui_example_event::INSTALLER;

        let _ = sdk_dispatcher_example_event::try_install
            as for<'a> fn(&mut events::EventBatch<'a>) -> Result<(), events::EventBatchError>;
        let _ = sdk_dispatcher_example_event::INSTALLER;

        let _ = sdk_input_example_event::try_install
            as for<'a> fn(&mut events::EventBatch<'a>) -> Result<(), events::EventBatchError>;
        let _ = sdk_input_example_event::INSTALLER;

        let _ = sdk_message_example_event::try_install
            as for<'a> fn(&mut events::EventBatch<'a>) -> Result<(), events::EventBatchError>;
        let _ = sdk_message_example_event::INSTALLER;
        let _ = sdk_message_phase_example_event::try_install
            as for<'a> fn(&mut events::EventBatch<'a>) -> Result<(), events::EventBatchError>;
        let _ = sdk_message_phase_example_event::INSTALLER;
        let _ = sdk_bus_mut_example_event::try_install
            as for<'a> fn(&mut events::EventBatch<'a>) -> Result<(), events::EventBatchError>;
        let _ = sdk_bus_mut_example_event::INSTALLER;
        let _ = sdk_bus_ref_example_event::try_install
            as for<'a> fn(&mut events::EventBatch<'a>) -> Result<(), events::EventBatchError>;
        let _ = sdk_bus_ref_example_event::INSTALLER;

        let mut batch = events::EventBatch::new();
        let _ = events::install_all!(
            &mut batch,
            sdk_game_example_event,
            sdk_ui_example_event,
            sdk_dispatcher_example_event,
            sdk_input_example_event,
            sdk_message_example_event,
            sdk_message_phase_example_event,
            sdk_bus_mut_example_event,
            sdk_bus_ref_example_event,
        ) as Result<(), events::EventBatchError>;
    }
}
