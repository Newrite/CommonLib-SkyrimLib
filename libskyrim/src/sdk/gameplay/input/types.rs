use core::fmt;

use core_util::EnumSet;

use crate::re::{INPUT_CONTEXT_ID, USER_EVENT_FLAG};

use super::contexts::pop_context;
use super::state::{allow_text_input, restore, snapshot};

/// All user-event flags known to the gameplay input layer.
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

/// Control flags that usually matter for gameplay-safe gating.
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

/// Flags associated with character movement.
pub const MOVEMENT_CONTROL_FLAGS: [USER_EVENT_FLAG; 2] =
    [USER_EVENT_FLAG::kMovement, USER_EVENT_FLAG::kJumping];

/// Flags associated with camera/look control.
pub const CAMERA_CONTROL_FLAGS: [USER_EVENT_FLAG; 3] = [
    USER_EVENT_FLAG::kLooking,
    USER_EVENT_FLAG::kPOVSwitch,
    USER_EVENT_FLAG::kWheelZoom,
];

/// Flags associated with menu-style input.
pub const MENU_CONTROL_FLAGS: [USER_EVENT_FLAG; 2] =
    [USER_EVENT_FLAG::kMenu, USER_EVENT_FLAG::kConsole];

/// Snapshot of the current input/control-map state.
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
    pub(crate) const fn new(
        enabled_controls: EnumSet<USER_EVENT_FLAG, u32>,
        stored_controls: EnumSet<USER_EVENT_FLAG, u32>,
        text_entry_count: i8,
        ignore_keyboard_mouse: bool,
        ignore_activate_disabled_events: bool,
        player_input_blocked: bool,
    ) -> Self {
        Self {
            enabled_controls,
            stored_controls,
            text_entry_count,
            ignore_keyboard_mouse,
            ignore_activate_disabled_events,
            player_input_blocked,
        }
    }

    /// Return the currently enabled control flags.
    #[inline(always)]
    pub fn enabled_controls(self) -> EnumSet<USER_EVENT_FLAG, u32> {
        self.enabled_controls
    }

    /// Return the stored pre-disable control flags when present.
    #[inline(always)]
    pub fn stored_controls(self) -> Option<EnumSet<USER_EVENT_FLAG, u32>> {
        (self.stored_controls.underlying() != USER_EVENT_FLAG::kInvalid as u32)
            .then_some(self.stored_controls)
    }

    #[inline(always)]
    pub(crate) const fn stored_controls_raw(self) -> EnumSet<USER_EVENT_FLAG, u32> {
        self.stored_controls
    }

    /// Return the text-entry nesting count tracked by `ControlMap`.
    #[inline(always)]
    pub fn text_entry_count(self) -> i8 {
        self.text_entry_count
    }

    /// Whether keyboard/mouse input is currently ignored.
    #[inline(always)]
    pub fn ignore_keyboard_mouse(self) -> bool {
        self.ignore_keyboard_mouse
    }

    /// Whether activate-disabled events are currently ignored.
    #[inline(always)]
    pub fn ignore_activate_disabled_events(self) -> bool {
        self.ignore_activate_disabled_events
    }

    /// Whether `PlayerControls` are currently hard-blocked.
    #[inline(always)]
    pub fn player_input_blocked(self) -> bool {
        self.player_input_blocked
    }
}

#[must_use = "dropping the guard restores the captured input state"]
/// Restore-on-drop snapshot guard for the current input state.
pub struct InputStateGuard {
    snapshot: InputStateSnapshot,
    active: bool,
}

impl InputStateGuard {
    /// Capture the current input state for later restoration.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            snapshot: snapshot(),
            active: true,
        }
    }

    /// Return the captured input-state snapshot.
    #[inline(always)]
    pub fn snapshot(&self) -> InputStateSnapshot {
        self.snapshot
    }

    /// Restore immediately and consume the guard.
    #[inline(always)]
    pub fn restore_now(mut self) {
        if self.active {
            restore(self.snapshot);
            self.active = false;
        }
    }

    /// Disable restore-on-drop and return the captured snapshot.
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
/// Guard returned by scoped context pushes.
pub struct ContextGuard {
    context: INPUT_CONTEXT_ID,
    active: bool,
}

impl ContextGuard {
    #[inline(always)]
    pub(crate) const fn new(context: INPUT_CONTEXT_ID, active: bool) -> Self {
        Self { context, active }
    }

    /// Return the context this guard will pop on drop.
    #[inline(always)]
    pub fn context(&self) -> INPUT_CONTEXT_ID {
        self.context
    }

    /// Disable pop-on-drop and return the captured context ID.
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
/// Guard returned by scoped text-input allowance helpers.
pub struct TextInputGuard {
    active: bool,
}

impl TextInputGuard {
    #[inline(always)]
    pub(crate) const fn new(active: bool) -> Self {
        Self { active }
    }

    /// Disable release-on-drop.
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
