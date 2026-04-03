use crate::re::{INPUT_DEVICE, INPUT_EVENT_TYPE, InputEvent};

use super::InputEvents;

#[test]
fn wrappers_iterate_chain_and_specialize_types() {
    let mut tail = InputEvent {
        vtable: core::ptr::null(),
        device: core_util::EnumSet::from(INPUT_DEVICE::kMouse),
        event_type: core_util::EnumSet::from(INPUT_EVENT_TYPE::kMouseMove),
        next: core::ptr::null_mut(),
    };
    let mut head = InputEvent {
        vtable: core::ptr::null(),
        device: core_util::EnumSet::from(INPUT_DEVICE::kKeyboard),
        event_type: core_util::EnumSet::from(INPUT_EVENT_TYPE::kButton),
        next: &mut tail,
    };

    let events = unsafe { InputEvents::from_raw(&mut head) };
    assert!(!events.is_empty());
    assert_eq!(events.len(), 2);
    assert_eq!(events.iter().count(), 2);
    assert_eq!(events.devices().count(), 2);
    assert!(events.contains_device(INPUT_DEVICE::kKeyboard));
    assert!(events.contains_device(INPUT_DEVICE::kMouse));
    assert_eq!(events.buttons().count(), 1);
    assert_eq!(events.mouse_moves().count(), 1);
    assert_eq!(events.keyboard().count(), 1);
    assert_eq!(events.mouse().count(), 1);
    assert_eq!(events.gamepad().count(), 0);
    assert_eq!(events.thumbsticks().count(), 0);

    let mut events = unsafe { InputEvents::from_raw(&mut head) };
    assert!(events.first_mut().is_some());
    assert_eq!(events.iter_mut().count(), 2);
    assert_eq!(events.keyboard_mut().count(), 1);
    assert_eq!(events.mouse_mut().count(), 1);
    assert_eq!(events.gamepad_mut().count(), 0);
}
