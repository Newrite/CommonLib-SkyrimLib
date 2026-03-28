//! Ergonomic wrappers over `BSInputDeviceManager` and `InputEvent*` chains.

use core::marker::PhantomData;

use crate::re::{
    BSInputDeviceManager, ButtonEvent, CharEvent, IDEvent, InputEvent, MouseMoveEvent,
    ThumbstickEvent,
};
use crate::sdk::core::{GameRef, GameRefMut};

use super::source::{self, EventInstallError, EventSubscription, IntoEventFlow};

/// Borrowed view of one `InputEvent*` chain delivered through
/// `BSInputDeviceManager`.
pub struct InputEvents<'a> {
    head: *mut InputEvent,
    marker: PhantomData<&'a mut InputEvent>,
}

impl<'a> InputEvents<'a> {
    /// # Safety
    /// `head` must be null or point to a valid input-event chain for the
    /// duration of `'a`.
    #[inline(always)]
    pub const unsafe fn from_raw(head: *mut InputEvent) -> Self {
        Self {
            head,
            marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn head_ptr(&self) -> *mut InputEvent {
        self.head
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    #[inline(always)]
    pub fn head(&self) -> GameRef<'_, InputEvent> {
        unsafe { GameRef::from_raw(self.head) }
    }

    #[inline(always)]
    pub fn head_mut(&mut self) -> GameRefMut<'_, InputEvent> {
        unsafe { GameRefMut::from_raw(self.head) }
    }

    #[inline(always)]
    pub fn first(&self) -> Option<&InputEvent> {
        unsafe { self.head.as_ref() }
    }

    #[inline(always)]
    pub fn first_mut(&mut self) -> Option<&mut InputEvent> {
        unsafe { self.head.as_mut() }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    #[inline(always)]
    pub fn devices(&self) -> impl Iterator<Item = crate::re::INPUT_DEVICE> + '_ {
        self.iter().map(InputEvent::get_device)
    }

    #[inline(always)]
    pub fn contains_device(&self, device: crate::re::INPUT_DEVICE) -> bool {
        self.devices().any(|candidate| candidate == device)
    }

    #[inline(always)]
    pub fn iter(&self) -> InputEventIter<'_> {
        InputEventIter {
            current: self.head,
            marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> InputEventIterMut<'_> {
        InputEventIterMut {
            current: self.head,
            marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn buttons(&self) -> impl Iterator<Item = &ButtonEvent> + '_ {
        self.iter().filter_map(InputEvent::as_button_event)
    }

    #[inline(always)]
    pub fn buttons_mut(&mut self) -> impl Iterator<Item = &mut ButtonEvent> + '_ {
        self.iter_mut().filter_map(InputEvent::as_button_event_mut)
    }

    #[inline(always)]
    pub fn chars(&self) -> impl Iterator<Item = &CharEvent> + '_ {
        self.iter().filter_map(InputEvent::as_char_event)
    }

    #[inline(always)]
    pub fn chars_mut(&mut self) -> impl Iterator<Item = &mut CharEvent> + '_ {
        self.iter_mut().filter_map(InputEvent::as_char_event_mut)
    }

    #[inline(always)]
    pub fn ids(&self) -> impl Iterator<Item = &IDEvent> + '_ {
        self.iter().filter_map(InputEvent::as_id_event)
    }

    #[inline(always)]
    pub fn ids_mut(&mut self) -> impl Iterator<Item = &mut IDEvent> + '_ {
        self.iter_mut().filter_map(InputEvent::as_id_event_mut)
    }

    #[inline(always)]
    pub fn mouse_moves(&self) -> impl Iterator<Item = &MouseMoveEvent> + '_ {
        self.iter().filter_map(InputEvent::as_mouse_move_event)
    }

    #[inline(always)]
    pub fn mouse_moves_mut(&mut self) -> impl Iterator<Item = &mut MouseMoveEvent> + '_ {
        self.iter_mut()
            .filter_map(InputEvent::as_mouse_move_event_mut)
    }

    #[inline(always)]
    pub fn thumbsticks(&self) -> impl Iterator<Item = &ThumbstickEvent> + '_ {
        self.iter().filter_map(InputEvent::as_thumbstick_event)
    }

    #[inline(always)]
    pub fn thumbsticks_mut(&mut self) -> impl Iterator<Item = &mut ThumbstickEvent> + '_ {
        self.iter_mut()
            .filter_map(InputEvent::as_thumbstick_event_mut)
    }

    #[inline(always)]
    pub fn keyboard(&self) -> impl Iterator<Item = &InputEvent> + '_ {
        self.iter()
            .filter(|event| event.get_device() == crate::re::INPUT_DEVICE::kKeyboard)
    }

    #[inline(always)]
    pub fn keyboard_mut(&mut self) -> impl Iterator<Item = &mut InputEvent> + '_ {
        self.iter_mut()
            .filter(|event| event.get_device() == crate::re::INPUT_DEVICE::kKeyboard)
    }

    #[inline(always)]
    pub fn mouse(&self) -> impl Iterator<Item = &InputEvent> + '_ {
        self.iter()
            .filter(|event| event.get_device() == crate::re::INPUT_DEVICE::kMouse)
    }

    #[inline(always)]
    pub fn mouse_mut(&mut self) -> impl Iterator<Item = &mut InputEvent> + '_ {
        self.iter_mut()
            .filter(|event| event.get_device() == crate::re::INPUT_DEVICE::kMouse)
    }

    #[inline(always)]
    pub fn gamepad(&self) -> impl Iterator<Item = &InputEvent> + '_ {
        self.iter()
            .filter(|event| event.get_device() == crate::re::INPUT_DEVICE::kGamepad)
    }

    #[inline(always)]
    pub fn gamepad_mut(&mut self) -> impl Iterator<Item = &mut InputEvent> + '_ {
        self.iter_mut()
            .filter(|event| event.get_device() == crate::re::INPUT_DEVICE::kGamepad)
    }
}

impl<'a> Default for InputEvents<'a> {
    #[inline(always)]
    fn default() -> Self {
        unsafe { Self::from_raw(core::ptr::null_mut()) }
    }
}

/// Immutable iterator over one input-event chain.
pub struct InputEventIter<'a> {
    current: *mut InputEvent,
    marker: PhantomData<&'a InputEvent>,
}

impl<'a> Iterator for InputEventIter<'a> {
    type Item = &'a InputEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let event = unsafe { self.current.as_ref()? };
        self.current = event.next;
        Some(event)
    }
}

/// Mutable iterator over one input-event chain.
pub struct InputEventIterMut<'a> {
    current: *mut InputEvent,
    marker: PhantomData<&'a mut InputEvent>,
}

impl<'a> Iterator for InputEventIterMut<'a> {
    type Item = &'a mut InputEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let event = unsafe { self.current.as_mut()? };
        self.current = event.next;
        Some(event)
    }
}

#[inline(always)]
fn manager_source() -> *mut crate::re::BSTEventSource<*mut InputEvent> {
    let manager = BSInputDeviceManager::get_singleton();
    if manager.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { &mut (*manager).event_source }
    }
}

/// Subscribe to `BSInputDeviceManager` input chains.
pub fn subscribe<F, R>(
    callback: F,
) -> Result<EventSubscription<'static, *mut InputEvent>, EventInstallError>
where
    F: for<'a> FnMut(InputEvents<'a>) -> R + 'static,
    R: IntoEventFlow,
{
    let mut callback = callback;
    unsafe {
        source::subscribe_static(manager_source(), move |event| {
            let head = event.copied().unwrap_or(core::ptr::null_mut());
            callback(InputEvents::from_raw(head))
        })
    }
}

/// Subscribe to `BSInputDeviceManager` input chains at the front of the sink
/// list.
pub fn prepend<F, R>(
    callback: F,
) -> Result<EventSubscription<'static, *mut InputEvent>, EventInstallError>
where
    F: for<'a> FnMut(InputEvents<'a>) -> R + 'static,
    R: IntoEventFlow,
{
    let mut callback = callback;
    unsafe {
        source::prepend_static(manager_source(), move |event| {
            let head = event.copied().unwrap_or(core::ptr::null_mut());
            callback(InputEvents::from_raw(head))
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::re::{INPUT_DEVICE, INPUT_EVENT_TYPE};

    use super::*;

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
}
