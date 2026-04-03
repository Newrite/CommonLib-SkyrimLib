use crate::re::{BSFixedString, ButtonEvent, NiPoint2, ThumbstickEvent, ThumbstickInputType};
use crate::sdk::events::input::InputEvents;

use super::directional::{
    AnalogInputSnapshot, DEFAULT_DIRECTION_THRESHOLD, InputDirection, direction_from_vector,
    thumbstick_input_snapshot,
};

pub const DEFAULT_TAP_MAX_HELD_SECS: f32 = 0.25;
pub const DEFAULT_HOLD_MIN_HELD_SECS: f32 = 0.25;

#[inline(always)]
fn sanitize_duration_threshold(seconds: f32, default: f32) -> f32 {
    if seconds.is_finite() && seconds >= 0.0 {
        seconds
    } else {
        default
    }
}

#[inline(always)]
fn user_event_matches_fixed_string(user_event: &BSFixedString, candidate: &str) -> bool {
    !candidate.is_empty() && user_event.as_str() == candidate
}

#[inline(always)]
pub fn button_user_event(button: &ButtonEvent) -> &str {
    button.get_user_event().as_str()
}

#[inline(always)]
pub fn button_matches_user_event(button: &ButtonEvent, user_event: &str) -> bool {
    user_event_matches_fixed_string(button.get_user_event(), user_event)
}

#[inline(always)]
pub fn button_is_pressed_for(button: &ButtonEvent, user_event: &str) -> bool {
    button_matches_user_event(button, user_event) && button.is_pressed()
}

#[inline(always)]
pub fn button_is_down_for(button: &ButtonEvent, user_event: &str) -> bool {
    button_matches_user_event(button, user_event) && button.is_down()
}

#[inline(always)]
pub fn button_is_up_for(button: &ButtonEvent, user_event: &str) -> bool {
    button_matches_user_event(button, user_event) && button.is_up()
}

#[inline(always)]
pub fn button_is_repeating_for(button: &ButtonEvent, user_event: &str) -> bool {
    button_matches_user_event(button, user_event) && button.is_repeating()
}

#[inline(always)]
pub fn button_is_tap_for(button: &ButtonEvent, user_event: &str, max_held_secs: f32) -> bool {
    let max_held_secs = sanitize_duration_threshold(max_held_secs, DEFAULT_TAP_MAX_HELD_SECS);
    button_matches_user_event(button, user_event)
        && button.is_up()
        && button.held_duration() <= max_held_secs
}

#[inline(always)]
pub fn button_is_hold_for(button: &ButtonEvent, user_event: &str, min_held_secs: f32) -> bool {
    let min_held_secs = sanitize_duration_threshold(min_held_secs, DEFAULT_HOLD_MIN_HELD_SECS);
    button_matches_user_event(button, user_event)
        && button.held_duration() >= min_held_secs
        && (button.is_held() || button.is_up())
}

#[inline(always)]
pub fn any_user_event_pressed(events: &InputEvents<'_>, user_event: &str) -> bool {
    events
        .buttons()
        .any(|button| button_is_pressed_for(button, user_event))
}

#[inline(always)]
pub fn any_user_event_down(events: &InputEvents<'_>, user_event: &str) -> bool {
    events
        .buttons()
        .any(|button| button_is_down_for(button, user_event))
}

#[inline(always)]
pub fn any_user_event_up(events: &InputEvents<'_>, user_event: &str) -> bool {
    events
        .buttons()
        .any(|button| button_is_up_for(button, user_event))
}

#[inline(always)]
pub fn any_user_event_repeating(events: &InputEvents<'_>, user_event: &str) -> bool {
    events
        .buttons()
        .any(|button| button_is_repeating_for(button, user_event))
}

#[inline(always)]
pub fn any_user_event_tap(events: &InputEvents<'_>, user_event: &str, max_held_secs: f32) -> bool {
    events
        .buttons()
        .any(|button| button_is_tap_for(button, user_event, max_held_secs))
}

#[inline(always)]
pub fn any_user_event_hold(events: &InputEvents<'_>, user_event: &str, min_held_secs: f32) -> bool {
    events
        .buttons()
        .any(|button| button_is_hold_for(button, user_event, min_held_secs))
}

#[inline(always)]
pub fn thumbstick_matches_side(event: &ThumbstickEvent, side: ThumbstickInputType) -> bool {
    match side {
        ThumbstickInputType::kLeftThumbstick => event.is_left(),
        ThumbstickInputType::kRightThumbstick => event.is_right(),
    }
}

#[inline(always)]
pub fn thumbstick_vector(event: &ThumbstickEvent) -> NiPoint2 {
    thumbstick_input_snapshot(event).current()
}

#[inline(always)]
pub fn thumbstick_direction_for_side(
    event: &ThumbstickEvent,
    side: ThumbstickInputType,
    threshold: f32,
) -> Option<InputDirection> {
    thumbstick_matches_side(event, side)
        .then(|| direction_from_vector(thumbstick_vector(event), threshold))
        .flatten()
}

pub fn thumbstick_snapshot_in_events(
    events: &InputEvents<'_>,
    side: ThumbstickInputType,
) -> Option<AnalogInputSnapshot> {
    events
        .thumbsticks()
        .filter(|event| thumbstick_matches_side(event, side))
        .map(thumbstick_input_snapshot)
        .last()
}

#[inline(always)]
pub fn thumbstick_vector_in_events(
    events: &InputEvents<'_>,
    side: ThumbstickInputType,
) -> Option<NiPoint2> {
    thumbstick_snapshot_in_events(events, side).map(AnalogInputSnapshot::current)
}

#[inline(always)]
pub fn thumbstick_direction_in_events(
    events: &InputEvents<'_>,
    side: ThumbstickInputType,
    threshold: f32,
) -> Option<InputDirection> {
    thumbstick_vector_in_events(events, side)
        .and_then(|vector| direction_from_vector(vector, threshold))
}

#[inline(always)]
pub fn left_thumbstick_direction_in_events(
    events: &InputEvents<'_>,
    threshold: f32,
) -> Option<InputDirection> {
    thumbstick_direction_in_events(events, ThumbstickInputType::kLeftThumbstick, threshold)
}

#[inline(always)]
pub fn right_thumbstick_direction_in_events(
    events: &InputEvents<'_>,
    threshold: f32,
) -> Option<InputDirection> {
    thumbstick_direction_in_events(events, ThumbstickInputType::kRightThumbstick, threshold)
}

#[inline(always)]
pub fn left_thumbstick_direction_default(events: &InputEvents<'_>) -> Option<InputDirection> {
    left_thumbstick_direction_in_events(events, DEFAULT_DIRECTION_THRESHOLD)
}

#[inline(always)]
pub fn right_thumbstick_direction_default(events: &InputEvents<'_>) -> Option<InputDirection> {
    right_thumbstick_direction_in_events(events, DEFAULT_DIRECTION_THRESHOLD)
}

#[cfg(test)]
mod tests {
    use crate::re::{
        BSFixedString, ButtonEvent, IDEvent, INPUT_DEVICE, InputEvent, NiPoint2, ThumbstickEvent,
        ThumbstickInputType,
    };
    use crate::sdk::events::input::InputEvents;

    use super::{
        any_user_event_down, any_user_event_hold, any_user_event_tap,
        left_thumbstick_direction_in_events, right_thumbstick_direction_in_events,
        thumbstick_vector,
    };
    use crate::sdk::gameplay::input::InputDirection;

    #[repr(C, align(8))]
    struct TestButtonEvent {
        storage: [u8; 0x38],
    }

    impl TestButtonEvent {
        fn new(user_event: &str, value: f32, held_down_secs: f32) -> Self {
            let mut event = Self { storage: [0; 0x38] };
            event.button_mut().init(
                INPUT_DEVICE::kKeyboard,
                42,
                value,
                held_down_secs,
                BSFixedString::from_str(user_event),
            );
            event
        }

        fn button(&self) -> &ButtonEvent {
            unsafe { &*(self.storage.as_ptr().cast()) }
        }

        fn button_mut(&mut self) -> &mut ButtonEvent {
            unsafe { &mut *(self.storage.as_mut_ptr().cast()) }
        }

        fn input_mut(&mut self) -> &mut InputEvent {
            &mut self.button_mut().base
        }
    }

    impl Drop for TestButtonEvent {
        fn drop(&mut self) {
            unsafe { core::ptr::drop_in_place(self.storage.as_mut_ptr().cast::<IDEvent>()) };
        }
    }

    fn make_thumbstick(side: ThumbstickInputType, x_value: f32, y_value: f32) -> ThumbstickEvent {
        let mut event: ThumbstickEvent = unsafe { core::mem::zeroed() };
        event.init(side, x_value, y_value);
        event
    }

    #[test]
    fn chain_level_button_helpers_detect_down_tap_and_hold() {
        let mut down = TestButtonEvent::new("Jump", 1.0, 0.0);
        let hold = TestButtonEvent::new("Jump", 0.0, 0.35);
        down.input_mut().next =
            hold.button() as *const ButtonEvent as *mut ButtonEvent as *mut InputEvent;

        let events = unsafe {
            InputEvents::from_raw(
                down.button() as *const ButtonEvent as *mut ButtonEvent as *mut InputEvent
            )
        };

        assert!(any_user_event_down(&events, "Jump"));
        assert!(any_user_event_tap(&events, "Jump", 0.4));
        assert!(any_user_event_hold(&events, "Jump", 0.25));
    }

    #[test]
    fn thumbstick_helpers_pick_matching_side_and_direction() {
        let mut left = make_thumbstick(ThumbstickInputType::kLeftThumbstick, -1.0, 0.0);
        let right = make_thumbstick(ThumbstickInputType::kRightThumbstick, 0.0, 1.0);
        left.base.base.next =
            &right as *const ThumbstickEvent as *mut ThumbstickEvent as *mut InputEvent;

        let events = unsafe {
            InputEvents::from_raw(
                &left as *const ThumbstickEvent as *mut ThumbstickEvent as *mut InputEvent,
            )
        };

        assert_eq!(thumbstick_vector(&left), NiPoint2::new(-1.0, 0.0));
        assert_eq!(
            left_thumbstick_direction_in_events(&events, 0.25),
            Some(InputDirection::Left)
        );
        assert_eq!(
            right_thumbstick_direction_in_events(&events, 0.25),
            Some(InputDirection::Up)
        );
    }
}
