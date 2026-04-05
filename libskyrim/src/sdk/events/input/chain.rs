use core::marker::PhantomData;

use crate::re::{ButtonEvent, CharEvent, IDEvent, InputEvent, MouseMoveEvent, ThumbstickEvent};

/// Borrowed view of one `InputEvent*` chain delivered through
/// `BSInputDeviceManager`.
///
/// `InputEvents` is the core convenience wrapper for Skyrim's linked-list input
/// packets. It keeps the original chain shape, but exposes iterator/filter
/// helpers for common plugin workflows:
///
/// - inspect all events in order through [`InputEvents::iter`]
/// - scan one event family such as [`InputEvents::buttons`] or
///   [`InputEvents::thumbsticks`]
/// - gate work by device with [`InputEvents::keyboard`],
///   [`InputEvents::mouse`], or [`InputEvents::gamepad`]
///
/// The wrapper is intentionally borrowed and short-lived: it reflects one live
/// dispatch chain, not a retained copy of input state.
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
        self.iter().filter_map(InputEvent::try_get_device)
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
            .filter(|event| event.try_get_device() == Some(crate::re::INPUT_DEVICE::kKeyboard))
    }

    #[inline(always)]
    pub fn keyboard_mut(&mut self) -> impl Iterator<Item = &mut InputEvent> + '_ {
        self.iter_mut()
            .filter(|event| event.try_get_device() == Some(crate::re::INPUT_DEVICE::kKeyboard))
    }

    #[inline(always)]
    pub fn mouse(&self) -> impl Iterator<Item = &InputEvent> + '_ {
        self.iter()
            .filter(|event| event.try_get_device() == Some(crate::re::INPUT_DEVICE::kMouse))
    }

    #[inline(always)]
    pub fn mouse_mut(&mut self) -> impl Iterator<Item = &mut InputEvent> + '_ {
        self.iter_mut()
            .filter(|event| event.try_get_device() == Some(crate::re::INPUT_DEVICE::kMouse))
    }

    #[inline(always)]
    pub fn gamepad(&self) -> impl Iterator<Item = &InputEvent> + '_ {
        self.iter()
            .filter(|event| event.try_get_device() == Some(crate::re::INPUT_DEVICE::kGamepad))
    }

    #[inline(always)]
    pub fn gamepad_mut(&mut self) -> impl Iterator<Item = &mut InputEvent> + '_ {
        self.iter_mut()
            .filter(|event| event.try_get_device() == Some(crate::re::INPUT_DEVICE::kGamepad))
    }
}

impl<'a> Default for InputEvents<'a> {
    #[inline(always)]
    fn default() -> Self {
        unsafe { Self::from_raw(core::ptr::null_mut()) }
    }
}

/// Immutable iterator over one input-event chain.
///
/// Produced by [`InputEvents::iter`].
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
///
/// Produced by [`InputEvents::iter_mut`] when the sink wants to mutate events
/// in-place before later listeners observe them.
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
