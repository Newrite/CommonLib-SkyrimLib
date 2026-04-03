use core::fmt;

use crate::re::{
    AttackBlockHandler, AttackBlockHandlerData, LookHandler, MovementHandler, PlayerInputHandler,
};

use super::access::player_controls;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputHandlerKind {
    Movement,
    Look,
    Attack,
}

impl InputHandlerKind {
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Movement => "movement",
            Self::Look => "look",
            Self::Attack => "attack",
        }
    }
}

impl fmt::Display for InputHandlerKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackInputSnapshot {
    pub held_state_active: bool,
    pub trigger_release_event: bool,
    pub ignore: bool,
    pub held_left: bool,
    pub held_right: bool,
    pub attack_count: u8,
}

impl AttackInputSnapshot {
    #[inline(always)]
    const fn from_data(
        held_state_active: bool,
        trigger_release_event: bool,
        data: &AttackBlockHandlerData,
    ) -> Self {
        Self {
            held_state_active,
            trigger_release_event,
            ignore: data.ignore,
            held_left: data.held_left,
            held_right: data.held_right,
            attack_count: data.attack_count,
        }
    }

    #[inline(always)]
    pub const fn has_held_attack(self) -> bool {
        self.held_left || self.held_right
    }
}

#[must_use = "dropping the guard restores the previous handler enabled state"]
pub struct InputHandlerGuard {
    kind: InputHandlerKind,
    restore_enabled: bool,
    active: bool,
}

impl InputHandlerGuard {
    #[inline(always)]
    fn new(kind: InputHandlerKind, restore_enabled: bool, active: bool) -> Self {
        Self {
            kind,
            restore_enabled,
            active,
        }
    }

    #[inline(always)]
    pub fn kind(&self) -> InputHandlerKind {
        self.kind
    }

    #[inline(always)]
    pub fn disarm(mut self) -> (InputHandlerKind, bool) {
        self.active = false;
        (self.kind, self.restore_enabled)
    }
}

impl Drop for InputHandlerGuard {
    fn drop(&mut self) {
        if self.active {
            let _ = set_handler_enabled(self.kind, self.restore_enabled);
        }
    }
}

impl fmt::Debug for InputHandlerGuard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InputHandlerGuard")
            .field("kind", &self.kind)
            .field("restore_enabled", &self.restore_enabled)
            .field("active", &self.active)
            .finish()
    }
}

#[inline(always)]
fn warn_missing_handler(_kind: InputHandlerKind, _caller: &str) {
    crate::defensive_sdk_warn!(
        "sdk::gameplay::input::{}() could not access {} handler",
        _caller,
        _kind.as_str()
    );
}

#[inline(always)]
fn handler_ptr(kind: InputHandlerKind) -> *mut PlayerInputHandler {
    let controls = player_controls();
    match kind {
        InputHandlerKind::Movement => controls.movement_handler.cast(),
        InputHandlerKind::Look => controls.look_handler.cast(),
        InputHandlerKind::Attack => controls.attack_block_handler.cast(),
    }
}

#[inline(always)]
fn with_handler<R>(
    kind: InputHandlerKind,
    caller: &str,
    f: impl FnOnce(&PlayerInputHandler) -> R,
) -> Option<R> {
    let Some(handler) = (unsafe { handler_ptr(kind).as_ref() }) else {
        warn_missing_handler(kind, caller);
        return None;
    };
    Some(f(handler))
}

#[inline(always)]
fn with_handler_mut<R>(
    kind: InputHandlerKind,
    caller: &str,
    f: impl FnOnce(&mut PlayerInputHandler) -> R,
) -> Option<R> {
    let Some(handler) = (unsafe { handler_ptr(kind).as_mut() }) else {
        warn_missing_handler(kind, caller);
        return None;
    };
    Some(f(handler))
}

#[inline(always)]
pub fn has_handler(kind: InputHandlerKind) -> bool {
    !handler_ptr(kind).is_null()
}

#[inline(always)]
pub fn is_handler_enabled(kind: InputHandlerKind) -> bool {
    with_handler(
        kind,
        "is_handler_enabled",
        PlayerInputHandler::is_input_event_handling_enabled,
    )
    .unwrap_or(false)
}

#[inline(always)]
pub fn set_handler_enabled(kind: InputHandlerKind, enabled: bool) -> bool {
    with_handler_mut(kind, "set_handler_enabled", |handler| {
        let previous = handler.is_input_event_handling_enabled();
        handler.set_input_event_handling_enabled(enabled);
        previous
    })
    .unwrap_or(false)
}

pub fn scoped_handler_disabled(kind: InputHandlerKind) -> InputHandlerGuard {
    let Some(previous) = with_handler_mut(kind, "scoped_handler_disabled", |handler| {
        let previous = handler.is_input_event_handling_enabled();
        handler.set_input_event_handling_enabled(false);
        previous
    }) else {
        return InputHandlerGuard::new(kind, false, false);
    };

    InputHandlerGuard::new(kind, previous, true)
}

#[inline(always)]
pub fn movement_input_guard() -> InputHandlerGuard {
    scoped_handler_disabled(InputHandlerKind::Movement)
}

#[inline(always)]
pub fn look_input_guard() -> InputHandlerGuard {
    scoped_handler_disabled(InputHandlerKind::Look)
}

#[inline(always)]
pub fn attack_input_guard() -> InputHandlerGuard {
    scoped_handler_disabled(InputHandlerKind::Attack)
}

#[inline(always)]
pub fn scoped_movement_and_attack_input_guards() -> (InputHandlerGuard, InputHandlerGuard) {
    (movement_input_guard(), attack_input_guard())
}

#[inline(always)]
pub fn has_movement_handler() -> bool {
    has_handler(InputHandlerKind::Movement)
}

#[inline(always)]
pub fn has_look_handler() -> bool {
    has_handler(InputHandlerKind::Look)
}

#[inline(always)]
pub fn has_attack_handler() -> bool {
    has_handler(InputHandlerKind::Attack)
}

#[inline(always)]
pub fn is_movement_handler_enabled() -> bool {
    is_handler_enabled(InputHandlerKind::Movement)
}

#[inline(always)]
pub fn is_look_handler_enabled() -> bool {
    is_handler_enabled(InputHandlerKind::Look)
}

#[inline(always)]
pub fn is_attack_handler_enabled() -> bool {
    is_handler_enabled(InputHandlerKind::Attack)
}

#[inline(always)]
pub fn set_movement_handler_enabled(enabled: bool) -> bool {
    set_handler_enabled(InputHandlerKind::Movement, enabled)
}

#[inline(always)]
pub fn set_look_handler_enabled(enabled: bool) -> bool {
    set_handler_enabled(InputHandlerKind::Look, enabled)
}

#[inline(always)]
pub fn set_attack_handler_enabled(enabled: bool) -> bool {
    set_handler_enabled(InputHandlerKind::Attack, enabled)
}

#[inline(always)]
pub fn scoped_movement_handler_disabled() -> InputHandlerGuard {
    movement_input_guard()
}

#[inline(always)]
pub fn scoped_look_handler_disabled() -> InputHandlerGuard {
    look_input_guard()
}

#[inline(always)]
pub fn scoped_attack_handler_disabled() -> InputHandlerGuard {
    attack_input_guard()
}

pub fn attack_input_snapshot() -> Option<AttackInputSnapshot> {
    let handler = player_controls().attack_block_handler;
    let Some(handler) = (unsafe { handler.as_ref() }) else {
        warn_missing_handler(InputHandlerKind::Attack, "attack_input_snapshot");
        return None;
    };

    let held_state = handler.base.data();
    let data = handler.data();
    Some(AttackInputSnapshot::from_data(
        held_state.held_state_active,
        held_state.trigger_release_event,
        data,
    ))
}

#[inline(always)]
pub fn movement_handler() -> *mut MovementHandler {
    player_controls().movement_handler
}

#[inline(always)]
pub fn look_handler() -> *mut LookHandler {
    player_controls().look_handler
}

#[inline(always)]
pub fn attack_handler() -> *mut AttackBlockHandler {
    player_controls().attack_block_handler
}
