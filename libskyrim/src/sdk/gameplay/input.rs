//! Gameplay-facing input helpers.
//!
//! This layer intentionally focuses on the parts of Skyrim input state that
//! plugin code repeatedly needs:
//!
//! - `ControlMap` singleton access
//! - context-stack inspection and scoped context pushes
//! - enabled-control snapshots and restore-on-drop guards
//! - player-input hard blocking through `PlayerControls`
//! - simple user-event mapping lookup helpers

use alloc::vec::Vec;
use core::fmt;

use core_util::EnumSet;

use crate::re::{
    BSFixedString, ControlMap, INPUT_CONTEXT_ID, INPUT_DEVICE, PlayerControls, USER_EVENT_FLAG,
    UserEventEnabled, UserEvents,
};
use crate::sdk::core::{GameRef, snapshot_contiguous_copied_named};

pub const ALL_CONTROL_FLAGS: [USER_EVENT_FLAG; 12] = [
    USER_EVENT_FLAG::kMovement,
    USER_EVENT_FLAG::kLooking,
    USER_EVENT_FLAG::kActivate,
    USER_EVENT_FLAG::kMenu,
    USER_EVENT_FLAG::kConsole,
    USER_EVENT_FLAG::kPOVSwitch,
    USER_EVENT_FLAG::kFighting,
    USER_EVENT_FLAG::kSneaking,
    USER_EVENT_FLAG::kMainFour,
    USER_EVENT_FLAG::kWheelZoom,
    USER_EVENT_FLAG::kJumping,
    USER_EVENT_FLAG::kVATS,
];

pub const GAMEPLAY_CONTROL_FLAGS: [USER_EVENT_FLAG; 10] = [
    USER_EVENT_FLAG::kMovement,
    USER_EVENT_FLAG::kLooking,
    USER_EVENT_FLAG::kActivate,
    USER_EVENT_FLAG::kPOVSwitch,
    USER_EVENT_FLAG::kFighting,
    USER_EVENT_FLAG::kSneaking,
    USER_EVENT_FLAG::kMainFour,
    USER_EVENT_FLAG::kWheelZoom,
    USER_EVENT_FLAG::kJumping,
    USER_EVENT_FLAG::kVATS,
];

pub const MOVEMENT_CONTROL_FLAGS: [USER_EVENT_FLAG; 2] =
    [USER_EVENT_FLAG::kMovement, USER_EVENT_FLAG::kJumping];

pub const CAMERA_CONTROL_FLAGS: [USER_EVENT_FLAG; 3] = [
    USER_EVENT_FLAG::kLooking,
    USER_EVENT_FLAG::kPOVSwitch,
    USER_EVENT_FLAG::kWheelZoom,
];

pub const MENU_CONTROL_FLAGS: [USER_EVENT_FLAG; 2] =
    [USER_EVENT_FLAG::kMenu, USER_EVENT_FLAG::kConsole];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputStateSnapshot {
    enabled_controls: EnumSet<USER_EVENT_FLAG, u32>,
    stored_controls: EnumSet<USER_EVENT_FLAG, u32>,
    text_entry_count: i8,
    ignore_keyboard_mouse: bool,
    ignore_activate_disabled_events: bool,
    player_input_blocked: bool,
}

impl InputStateSnapshot {
    #[inline(always)]
    pub fn enabled_controls(self) -> EnumSet<USER_EVENT_FLAG, u32> {
        self.enabled_controls
    }

    #[inline(always)]
    pub fn stored_controls(self) -> Option<EnumSet<USER_EVENT_FLAG, u32>> {
        (self.stored_controls.underlying() != USER_EVENT_FLAG::kInvalid as u32)
            .then_some(self.stored_controls)
    }

    #[inline(always)]
    pub fn text_entry_count(self) -> i8 {
        self.text_entry_count
    }

    #[inline(always)]
    pub fn ignore_keyboard_mouse(self) -> bool {
        self.ignore_keyboard_mouse
    }

    #[inline(always)]
    pub fn ignore_activate_disabled_events(self) -> bool {
        self.ignore_activate_disabled_events
    }

    #[inline(always)]
    pub fn player_input_blocked(self) -> bool {
        self.player_input_blocked
    }
}

#[must_use = "dropping the guard restores the captured input state"]
pub struct InputStateGuard {
    snapshot: InputStateSnapshot,
    active: bool,
}

impl InputStateGuard {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            snapshot: snapshot(),
            active: true,
        }
    }

    #[inline(always)]
    pub fn snapshot(&self) -> InputStateSnapshot {
        self.snapshot
    }

    #[inline(always)]
    pub fn restore_now(mut self) {
        if self.active {
            restore(self.snapshot);
            self.active = false;
        }
    }

    #[inline(always)]
    pub fn disarm(mut self) -> InputStateSnapshot {
        self.active = false;
        self.snapshot
    }
}

impl Default for InputStateGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for InputStateGuard {
    fn drop(&mut self) {
        if self.active {
            restore(self.snapshot);
        }
    }
}

impl fmt::Debug for InputStateGuard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InputStateGuard")
            .field("snapshot", &self.snapshot)
            .field("active", &self.active)
            .finish()
    }
}

#[must_use = "dropping the guard pops the pushed input context"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextGuard {
    context: INPUT_CONTEXT_ID,
    active: bool,
}

impl ContextGuard {
    #[inline(always)]
    pub fn context(&self) -> INPUT_CONTEXT_ID {
        self.context
    }

    #[inline(always)]
    pub fn disarm(mut self) -> INPUT_CONTEXT_ID {
        self.active = false;
        self.context
    }
}

impl Drop for ContextGuard {
    fn drop(&mut self) {
        if self.active {
            pop_context(self.context);
        }
    }
}

#[must_use = "dropping the guard releases the text-input allowance request"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextInputGuard {
    active: bool,
}

impl TextInputGuard {
    #[inline(always)]
    pub fn disarm(mut self) {
        self.active = false;
    }
}

impl Drop for TextInputGuard {
    fn drop(&mut self) {
        if self.active {
            let _ = allow_text_input(false);
        }
    }
}

#[inline(always)]
fn is_valid_context(context: INPUT_CONTEXT_ID) -> bool {
    context != INPUT_CONTEXT_ID::kNone() && context.get() < INPUT_CONTEXT_ID::kTotal()
}

#[inline(always)]
fn is_valid_device(device: INPUT_DEVICE) -> bool {
    let index = device as i32;
    index >= 0 && (index as u32) < INPUT_DEVICE::total()
}

#[inline(always)]
fn has_invalid_control_flag(flags: USER_EVENT_FLAG) -> bool {
    (flags as u32 & USER_EVENT_FLAG::kInvalid as u32) != 0
}

#[inline(always)]
pub fn control_map() -> GameRef<ControlMap> {
    unsafe { GameRef::from_raw(ControlMap::get_singleton()) }
}

#[inline(always)]
pub fn player_controls() -> GameRef<PlayerControls> {
    unsafe { GameRef::from_raw(PlayerControls::get_singleton()) }
}

#[inline(always)]
pub fn user_events() -> GameRef<UserEvents> {
    unsafe { GameRef::from_raw(UserEvents::get_singleton()) }
}

#[inline(always)]
pub fn push_context(context: INPUT_CONTEXT_ID) {
    if !is_valid_context(context) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::input::push_context() ignored invalid context id={}",
            context.get()
        );
        return;
    }

    ControlMap::push_input_context(context);
}

#[inline(always)]
pub fn pop_context(context: INPUT_CONTEXT_ID) {
    if !is_valid_context(context) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::input::pop_context() ignored invalid context id={}",
            context.get()
        );
        return;
    }

    ControlMap::pop_input_context(context);
}

#[inline(always)]
pub fn push_context_scoped(context: INPUT_CONTEXT_ID) -> ContextGuard {
    if !is_valid_context(context) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::input::push_context_scoped() ignored invalid context id={}",
            context.get()
        );
        return ContextGuard {
            context,
            active: false,
        };
    }

    push_context(context);
    ContextGuard {
        context,
        active: true,
    }
}

#[inline(always)]
pub fn context_stack() -> Vec<INPUT_CONTEXT_ID> {
    let control_map = control_map();
    let runtime_data = control_map.runtime_data();
    snapshot_contiguous_copied_named(
        &runtime_data.context_priority_stack,
        "sdk::gameplay::input::context_stack()",
        Default::default(),
    )
}

#[inline(always)]
pub fn context_stack_depth() -> usize {
    control_map().runtime_data().context_priority_stack.len() as usize
}

#[inline(always)]
pub fn top_context() -> Option<INPUT_CONTEXT_ID> {
    let control_map = control_map();
    let runtime_data = control_map.runtime_data();
    snapshot_contiguous_copied_named(
        &runtime_data.context_priority_stack,
        "sdk::gameplay::input::top_context()",
        Default::default(),
    )
    .last()
    .copied()
}

#[inline(always)]
pub fn has_context(context: INPUT_CONTEXT_ID) -> bool {
    if !is_valid_context(context) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::input::has_context() queried invalid context id={}",
            context.get()
        );
        return false;
    }

    let control_map = control_map();
    let runtime_data = control_map.runtime_data();
    snapshot_contiguous_copied_named(
        &runtime_data.context_priority_stack,
        "sdk::gameplay::input::has_context()",
        Default::default(),
    )
    .contains(&context)
}

#[inline(always)]
pub fn top_context_is(context: INPUT_CONTEXT_ID) -> bool {
    top_context().is_some_and(|current| current == context)
}

#[inline(always)]
pub fn is_gameplay_context(context: INPUT_CONTEXT_ID) -> bool {
    context == INPUT_CONTEXT_ID::kGameplay
}

#[inline(always)]
pub fn is_menu_like_context(context: INPUT_CONTEXT_ID) -> bool {
    context == INPUT_CONTEXT_ID::kMenuMode
        || context == INPUT_CONTEXT_ID::kConsole
        || context == INPUT_CONTEXT_ID::kItemMenu
        || context == INPUT_CONTEXT_ID::kInventory
        || context == INPUT_CONTEXT_ID::kDebugText
        || context == INPUT_CONTEXT_ID::kFavorites
        || context == INPUT_CONTEXT_ID::kMap
        || context == INPUT_CONTEXT_ID::kStats
        || context == INPUT_CONTEXT_ID::kCursor
        || context == INPUT_CONTEXT_ID::kBook
        || context == INPUT_CONTEXT_ID::kDebugOverlay
        || context == INPUT_CONTEXT_ID::kJournal
        || context == INPUT_CONTEXT_ID::kMapDebug
        || context == INPUT_CONTEXT_ID::kLockpicking
        || INPUT_CONTEXT_ID::kMarketplace().is_some_and(|marketplace| context == marketplace)
}

#[inline(always)]
pub fn has_menu_like_context() -> bool {
    context_stack().into_iter().any(is_menu_like_context)
}

#[inline(always)]
pub fn top_context_is_gameplay() -> bool {
    top_context_is(INPUT_CONTEXT_ID::kGameplay)
}

#[inline(always)]
pub fn top_context_is_cursor() -> bool {
    top_context_is(INPUT_CONTEXT_ID::kCursor)
}

#[inline(always)]
pub fn top_context_is_console() -> bool {
    top_context_is(INPUT_CONTEXT_ID::kConsole)
}

#[inline(always)]
pub fn top_context_is_item_menu() -> bool {
    top_context_is(INPUT_CONTEXT_ID::kItemMenu)
}

#[inline(always)]
pub fn top_context_is_menu_like() -> bool {
    top_context().is_some_and(is_menu_like_context)
}

#[inline(always)]
pub fn text_entry_count() -> i8 {
    control_map().runtime_data().text_entry_count
}

#[inline(always)]
pub fn has_text_input_requests() -> bool {
    text_entry_count() > 0
}

#[inline(always)]
pub fn allow_text_input(allow: bool) -> i8 {
    unsafe { control_map().with_mut_unchecked(|control_map| control_map.allow_text_input(allow)) }
}

#[inline(always)]
pub fn allow_text_input_scoped() -> TextInputGuard {
    let _ = allow_text_input(true);
    TextInputGuard { active: true }
}

#[inline(always)]
pub fn ignores_keyboard_mouse() -> bool {
    control_map().runtime_data().ignore_keyboard_mouse
}

#[inline(always)]
pub fn set_keyboard_mouse_ignored(ignored: bool) -> bool {
    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            let runtime_data = control_map.runtime_data_mut();
            let previous = runtime_data.ignore_keyboard_mouse;
            runtime_data.ignore_keyboard_mouse = ignored;
            previous
        })
    }
}

#[inline(always)]
pub fn ignores_activate_disabled_events() -> bool {
    control_map().runtime_data().ignore_activate_disabled_events
}

#[inline(always)]
pub fn set_activate_disabled_events_ignored(ignored: bool) -> bool {
    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            let runtime_data = control_map.runtime_data_mut();
            let previous = runtime_data.ignore_activate_disabled_events;
            runtime_data.ignore_activate_disabled_events = ignored;
            previous
        })
    }
}

#[inline(always)]
pub fn enabled_controls() -> EnumSet<USER_EVENT_FLAG, u32> {
    control_map().runtime_data().enabled_controls
}

#[inline(always)]
pub fn stored_controls() -> Option<EnumSet<USER_EVENT_FLAG, u32>> {
    let stored = control_map().runtime_data().stored_controls;
    (stored.underlying() != USER_EVENT_FLAG::kInvalid as u32).then_some(stored)
}

pub fn set_enabled_controls(enabled: EnumSet<USER_EVENT_FLAG, u32>) {
    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            let old_state = {
                let runtime_data = control_map.runtime_data_mut();
                let old_state = runtime_data.enabled_controls;
                if old_state == enabled {
                    return;
                }
                runtime_data.enabled_controls = enabled;
                old_state
            };

            let event = UserEventEnabled {
                new_user_event_flag: enabled,
                old_user_event_flag: old_state,
            };
            control_map.event_source.send_event(&event);
        })
    }
}

#[inline(always)]
pub fn are_controls_enabled(flags: USER_EVENT_FLAG) -> bool {
    control_map().are_controls_enabled(flags)
}

#[inline(always)]
pub fn set_control_enabled(flags: USER_EVENT_FLAG, enable: bool, store_state: bool) {
    if flags == USER_EVENT_FLAG::kNone {
        return;
    }
    if has_invalid_control_flag(flags) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::input::set_control_enabled() ignored invalid control flags=0x{:08X}",
            flags as u32
        );
        return;
    }

    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            control_map.toggle_controls(flags, enable, store_state)
        })
    };
}

#[inline(always)]
pub fn toggle_control(flags: USER_EVENT_FLAG, enable: bool, store_state: bool) {
    set_control_enabled(flags, enable, store_state);
}

pub fn set_control_group_enabled(flags: &[USER_EVENT_FLAG], enable: bool, store_state: bool) {
    if flags.is_empty() {
        return;
    }

    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            for &flag in flags {
                if flag == USER_EVENT_FLAG::kNone {
                    continue;
                }
                if has_invalid_control_flag(flag) {
                    crate::defensive_sdk_warn!(
                        "sdk::gameplay::input::set_control_group_enabled() skipped invalid control flags=0x{:08X}",
                        flag as u32
                    );
                    continue;
                }
                control_map.toggle_controls(flag, enable, store_state);
            }
        })
    };
}

#[inline(always)]
pub fn store_controls() {
    unsafe { control_map().with_mut_unchecked(ControlMap::store_controls) };
}

#[inline(always)]
pub fn restore_stored_controls() {
    unsafe { control_map().with_mut_unchecked(ControlMap::load_stored_controls) };
}

#[inline(always)]
pub fn set_gameplay_controls_enabled(enable: bool, store_state: bool) {
    set_control_group_enabled(&GAMEPLAY_CONTROL_FLAGS, enable, store_state);
}

#[inline(always)]
pub fn is_player_input_blocked() -> bool {
    player_controls().block_player_input
}

#[inline(always)]
pub fn set_player_input_blocked(blocked: bool) -> bool {
    unsafe {
        player_controls().with_mut_unchecked(|controls| {
            let previous = controls.block_player_input;
            controls.block_player_input = blocked;
            previous
        })
    }
}

#[inline(always)]
pub fn snapshot() -> InputStateSnapshot {
    let control_map = control_map();
    let runtime_data = control_map.runtime_data();
    InputStateSnapshot {
        enabled_controls: runtime_data.enabled_controls,
        stored_controls: runtime_data.stored_controls,
        text_entry_count: runtime_data.text_entry_count,
        ignore_keyboard_mouse: runtime_data.ignore_keyboard_mouse,
        ignore_activate_disabled_events: runtime_data.ignore_activate_disabled_events,
        player_input_blocked: is_player_input_blocked(),
    }
}

pub fn restore(snapshot: InputStateSnapshot) {
    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            let runtime_data = control_map.runtime_data_mut();
            runtime_data.stored_controls = snapshot.stored_controls;
            runtime_data.text_entry_count = snapshot.text_entry_count;
            runtime_data.ignore_keyboard_mouse = snapshot.ignore_keyboard_mouse;
            runtime_data.ignore_activate_disabled_events = snapshot.ignore_activate_disabled_events;
        })
    };
    let _ = set_player_input_blocked(snapshot.player_input_blocked);
    set_enabled_controls(snapshot.enabled_controls);
}

#[inline(always)]
pub fn scoped_state() -> InputStateGuard {
    InputStateGuard::new()
}

pub fn scoped_gameplay_controls_disabled() -> InputStateGuard {
    let guard = scoped_state();
    set_gameplay_controls_enabled(false, false);
    guard
}

pub fn scoped_player_input_blocked() -> InputStateGuard {
    let guard = scoped_state();
    let _ = set_player_input_blocked(true);
    guard
}

pub fn scoped_gameplay_input_suppressed() -> InputStateGuard {
    let guard = scoped_state();
    set_gameplay_controls_enabled(false, false);
    let _ = set_player_input_blocked(true);
    guard
}

#[inline(always)]
pub fn mapped_key(event_id: &str, device: INPUT_DEVICE, context: INPUT_CONTEXT_ID) -> Option<u32> {
    if !is_valid_device(device) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::input::mapped_key() queried invalid device={}",
            device as i32
        );
        return None;
    }
    if !is_valid_context(context) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::input::mapped_key() queried invalid context id={}",
            context.get()
        );
        return None;
    }

    let key = control_map().get_mapped_key(event_id, device, context);
    (key != ControlMap::kInvalid).then_some(key)
}

#[inline(always)]
pub fn button_name_from_user_event(event_id: &str, device: INPUT_DEVICE) -> Option<BSFixedString> {
    if !is_valid_device(device) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::input::button_name_from_user_event() queried invalid device={}",
            device as i32
        );
        return None;
    }

    let event_id = BSFixedString::from_str(event_id);
    let mut button_name = BSFixedString::default();
    control_map()
        .get_button_name_from_user_event(&event_id, device, &mut button_name)
        .then_some(button_name)
}

#[inline(always)]
pub fn user_event_name(
    button_id: u32,
    device: INPUT_DEVICE,
    context: INPUT_CONTEXT_ID,
) -> Option<BSFixedString> {
    if !is_valid_device(device) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::input::user_event_name() queried invalid device={}",
            device as i32
        );
        return None;
    }
    if !is_valid_context(context) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::input::user_event_name() queried invalid context id={}",
            context.get()
        );
        return None;
    }

    control_map()
        .get_user_event_name(button_id, device, context)
        .cloned()
}
