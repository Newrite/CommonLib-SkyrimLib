use crate::re::{MenuOpenCloseEvent, TESHitEvent};
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
fn sdk_dispatcher_example(_event: &ActionEvent) -> EventFlow {
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
        sdk_bus_mut_example_event,
        sdk_bus_ref_example_event,
    ) as Result<(), events::EventBatchError>;
}
