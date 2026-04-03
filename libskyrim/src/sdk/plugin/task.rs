//! High-level deferred-task helpers built on top of `libskyrim::skse::task`.
//!
//! This layer is intentionally small but more expressive than raw
//! `add_task(...)`:
//!
//! - name the handoff origin (`event`, `papyrus`, or generic)
//! - optionally gate execution behind the current gameplay-safe phase
//! - capture handles now and resolve them later inside the queued task

use crate::sdk::core::{
    GamePtr, HandleFamilyTarget, ResolvableHandle, Resolved, ResolvedHandle, RuntimePhaseBlocker,
    RuntimePhaseSnapshot, snapshot_runtime_phase,
};

pub use crate::skse::task::{add_task, add_ui_task};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskQueueKind {
    Background,
    Ui,
}

impl TaskQueueKind {
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Background => "background",
            Self::Ui => "ui",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskHandoffOrigin {
    Generic,
    Event,
    Papyrus,
}

impl TaskHandoffOrigin {
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::Event => "event",
            Self::Papyrus => "papyrus",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskHandoff {
    pub queue: TaskQueueKind,
    pub origin: TaskHandoffOrigin,
    pub require_safe_gameplay_phase: bool,
}

impl TaskHandoff {
    #[inline(always)]
    pub const fn new(queue: TaskQueueKind, origin: TaskHandoffOrigin) -> Self {
        Self {
            queue,
            origin,
            require_safe_gameplay_phase: false,
        }
    }

    #[inline(always)]
    pub const fn background() -> Self {
        Self::new(TaskQueueKind::Background, TaskHandoffOrigin::Generic)
    }

    #[inline(always)]
    pub const fn ui() -> Self {
        Self::new(TaskQueueKind::Ui, TaskHandoffOrigin::Generic)
    }

    #[inline(always)]
    pub const fn from_event() -> Self {
        Self::new(TaskQueueKind::Background, TaskHandoffOrigin::Event)
    }

    #[inline(always)]
    pub const fn ui_from_event() -> Self {
        Self::new(TaskQueueKind::Ui, TaskHandoffOrigin::Event)
    }

    #[inline(always)]
    pub const fn from_papyrus() -> Self {
        Self::new(TaskQueueKind::Background, TaskHandoffOrigin::Papyrus)
    }

    #[inline(always)]
    pub const fn ui_from_papyrus() -> Self {
        Self::new(TaskQueueKind::Ui, TaskHandoffOrigin::Papyrus)
    }

    #[inline(always)]
    pub const fn requiring_safe_gameplay_phase(mut self) -> Self {
        self.require_safe_gameplay_phase = true;
        self
    }

    #[inline(always)]
    pub const fn phase_blocker(
        self,
        snapshot: RuntimePhaseSnapshot,
    ) -> Option<RuntimePhaseBlocker> {
        if self.require_safe_gameplay_phase {
            snapshot.gameplay_blocker()
        } else {
            None
        }
    }

    #[inline(always)]
    pub const fn can_run(self, snapshot: RuntimePhaseSnapshot) -> bool {
        self.phase_blocker(snapshot).is_none()
    }

    pub fn dispatch(self, f: impl FnOnce() + Send + 'static) {
        queue_with_kind(self.queue, move || {
            if let Some(_blocker) = self.phase_blocker(snapshot_runtime_phase()) {
                crate::defensive_sdk_warn!(
                    "sdk::plugin::task::{:?} {} handoff skipped during {} phase",
                    self.queue,
                    self.origin.as_str(),
                    _blocker.as_str()
                );
                return;
            }

            f();
        });
    }

    pub fn dispatch_resolving_handle<H>(
        self,
        handle: H,
        f: impl FnOnce(ResolvedHandle<H>) + Send + 'static,
    ) -> bool
    where
        H: ResolvableHandle + Send + 'static,
    {
        if handle.is_null() {
            crate::defensive_sdk_warn!(
                "sdk::plugin::task::dispatch_resolving_handle() rejected null {} handoff handle",
                self.origin.as_str()
            );
            return false;
        }

        self.dispatch(move || {
            let Some(resolved) = ResolvedHandle::from_handle(handle) else {
                crate::defensive_sdk_warn!(
                    "sdk::plugin::task::dispatch_resolving_handle() could not resolve {} handoff handle",
                    self.origin.as_str()
                );
                return;
            };

            f(resolved);
        });
        true
    }

    pub fn dispatch_resolving_target<T>(
        self,
        target: impl Into<GamePtr<T>>,
        f: impl FnOnce(Resolved<T>) + Send + 'static,
    ) -> bool
    where
        T: HandleFamilyTarget,
        T::Handle: Send + 'static,
    {
        let Some(handle) =
            capture_target_handle("dispatch_resolving_target", self.origin, target.into())
        else {
            return false;
        };

        self.dispatch(move || {
            let Some(resolved) = Resolved::from_handle(handle) else {
                crate::defensive_sdk_warn!(
                    "sdk::plugin::task::dispatch_resolving_target() could not resolve {} handoff target",
                    self.origin.as_str()
                );
                return;
            };

            f(resolved);
        });
        true
    }

    pub fn dispatch_resolving_target_ref<T>(
        self,
        target: &T,
        f: impl FnOnce(Resolved<T>) + Send + 'static,
    ) -> bool
    where
        T: HandleFamilyTarget,
        T::Handle: Send + 'static,
    {
        let handle = T::canonical_handle(target as *const T as *mut T);
        if handle.is_null() {
            crate::defensive_sdk_warn!(
                "sdk::plugin::task::dispatch_resolving_target_ref() could not capture {} handoff target handle",
                self.origin.as_str()
            );
            return false;
        }

        self.dispatch(move || {
            let Some(resolved) = Resolved::from_handle(handle) else {
                crate::defensive_sdk_warn!(
                    "sdk::plugin::task::dispatch_resolving_target_ref() could not resolve {} handoff target",
                    self.origin.as_str()
                );
                return;
            };

            f(resolved);
        });
        true
    }
}

#[inline(always)]
fn queue_with_kind(queue: TaskQueueKind, f: impl FnOnce() + Send + 'static) {
    match queue {
        TaskQueueKind::Background => add_task(f),
        TaskQueueKind::Ui => add_ui_task(f),
    }
}

#[inline(always)]
fn capture_target_handle<T>(
    _caller: &str,
    _origin: TaskHandoffOrigin,
    target: GamePtr<T>,
) -> Option<T::Handle>
where
    T: HandleFamilyTarget,
{
    if target.is_null() {
        crate::defensive_sdk_warn!(
            "sdk::plugin::task::{}() rejected null {} handoff target",
            _caller,
            _origin.as_str()
        );
        return None;
    }

    let handle = T::canonical_handle(target.as_ptr());
    if handle.is_null() {
        crate::defensive_sdk_warn!(
            "sdk::plugin::task::{}() could not capture {} handoff target handle",
            _caller,
            _origin.as_str()
        );
        return None;
    }

    Some(handle)
}

#[inline(always)]
pub const fn handoff() -> TaskHandoff {
    TaskHandoff::background()
}

#[inline(always)]
pub const fn ui_handoff() -> TaskHandoff {
    TaskHandoff::ui()
}

#[inline(always)]
pub const fn handoff_from_event() -> TaskHandoff {
    TaskHandoff::from_event()
}

#[inline(always)]
pub const fn ui_handoff_from_event() -> TaskHandoff {
    TaskHandoff::ui_from_event()
}

#[inline(always)]
pub const fn handoff_from_papyrus() -> TaskHandoff {
    TaskHandoff::from_papyrus()
}

#[inline(always)]
pub const fn ui_handoff_from_papyrus() -> TaskHandoff {
    TaskHandoff::ui_from_papyrus()
}

#[inline(always)]
pub const fn gameplay_handoff() -> TaskHandoff {
    handoff().requiring_safe_gameplay_phase()
}

#[inline(always)]
pub const fn gameplay_handoff_from_event() -> TaskHandoff {
    handoff_from_event().requiring_safe_gameplay_phase()
}

#[inline(always)]
pub const fn gameplay_handoff_from_papyrus() -> TaskHandoff {
    handoff_from_papyrus().requiring_safe_gameplay_phase()
}

#[inline(always)]
pub fn add_event_task(f: impl FnOnce() + Send + 'static) {
    handoff_from_event().dispatch(f)
}

#[inline(always)]
pub fn add_ui_event_task(f: impl FnOnce() + Send + 'static) {
    ui_handoff_from_event().dispatch(f)
}

#[inline(always)]
pub fn add_papyrus_task(f: impl FnOnce() + Send + 'static) {
    handoff_from_papyrus().dispatch(f)
}

#[inline(always)]
pub fn add_ui_papyrus_task(f: impl FnOnce() + Send + 'static) {
    ui_handoff_from_papyrus().dispatch(f)
}

#[inline(always)]
pub fn add_gameplay_task(f: impl FnOnce() + Send + 'static) {
    gameplay_handoff().dispatch(f)
}

#[inline(always)]
pub fn add_gameplay_event_task(f: impl FnOnce() + Send + 'static) {
    gameplay_handoff_from_event().dispatch(f)
}

#[inline(always)]
pub fn add_gameplay_papyrus_task(f: impl FnOnce() + Send + 'static) {
    gameplay_handoff_from_papyrus().dispatch(f)
}

#[inline(always)]
pub fn add_task_resolving_handle<H>(
    handle: H,
    f: impl FnOnce(ResolvedHandle<H>) + Send + 'static,
) -> bool
where
    H: ResolvableHandle + Send + 'static,
{
    handoff().dispatch_resolving_handle(handle, f)
}

#[inline(always)]
pub fn add_ui_task_resolving_handle<H>(
    handle: H,
    f: impl FnOnce(ResolvedHandle<H>) + Send + 'static,
) -> bool
where
    H: ResolvableHandle + Send + 'static,
{
    ui_handoff().dispatch_resolving_handle(handle, f)
}

#[inline(always)]
pub fn add_gameplay_task_resolving_handle<H>(
    handle: H,
    f: impl FnOnce(ResolvedHandle<H>) + Send + 'static,
) -> bool
where
    H: ResolvableHandle + Send + 'static,
{
    gameplay_handoff().dispatch_resolving_handle(handle, f)
}

#[inline(always)]
pub fn add_task_resolving_target<T>(
    target: impl Into<GamePtr<T>>,
    f: impl FnOnce(Resolved<T>) + Send + 'static,
) -> bool
where
    T: HandleFamilyTarget,
    T::Handle: Send + 'static,
{
    handoff().dispatch_resolving_target(target, f)
}

#[inline(always)]
pub fn add_ui_task_resolving_target<T>(
    target: impl Into<GamePtr<T>>,
    f: impl FnOnce(Resolved<T>) + Send + 'static,
) -> bool
where
    T: HandleFamilyTarget,
    T::Handle: Send + 'static,
{
    ui_handoff().dispatch_resolving_target(target, f)
}

#[inline(always)]
pub fn add_gameplay_task_resolving_target<T>(
    target: impl Into<GamePtr<T>>,
    f: impl FnOnce(Resolved<T>) + Send + 'static,
) -> bool
where
    T: HandleFamilyTarget,
    T::Handle: Send + 'static,
{
    gameplay_handoff().dispatch_resolving_target(target, f)
}

#[cfg(test)]
mod tests {
    use super::{
        TaskHandoff, TaskHandoffOrigin, TaskQueueKind, gameplay_handoff,
        gameplay_handoff_from_event, handoff_from_papyrus, ui_handoff_from_event,
    };
    use crate::re::INPUT_CONTEXT_ID;
    use crate::sdk::core::RuntimePhaseBlocker;
    use crate::sdk::ui::controls::UiControlSnapshot;

    fn gameplay_ui_snapshot() -> UiControlSnapshot {
        UiControlSnapshot {
            top_context: Some(INPUT_CONTEXT_ID::kGameplay),
            menus_visible: true,
            game_paused: false,
            application_menu_open: false,
            item_menu_open: false,
            modal_menu_open: false,
            console_open: false,
            inventory_open: false,
            map_open: false,
            journal_open: false,
            loading_open: false,
            fader_open: false,
            fader_active: false,
            menu_like_context: false,
            text_input_active: false,
            keyboard_mouse_ignored: false,
            activate_disabled_events_ignored: false,
            player_input_blocked: false,
            gameplay_controls_enabled: true,
            menu_controls_enabled: true,
            console_controls_enabled: true,
        }
    }

    fn gameplay_phase_snapshot() -> crate::sdk::core::RuntimePhaseSnapshot {
        crate::sdk::core::RuntimePhaseSnapshot {
            ui: gameplay_ui_snapshot(),
            pause_menu_disabled: false,
            saving_allowed: true,
            cursor_hidden_when_topmost: false,
            custom_rendering_active: false,
        }
    }

    #[test]
    fn handoff_constructors_pick_expected_queue_origin_and_policy() {
        assert_eq!(TaskQueueKind::Background.as_str(), "background");
        assert_eq!(TaskQueueKind::Ui.as_str(), "ui");
        assert_eq!(TaskHandoffOrigin::Event.as_str(), "event");
        assert_eq!(TaskHandoffOrigin::Papyrus.as_str(), "papyrus");

        assert_eq!(
            handoff_from_papyrus(),
            TaskHandoff {
                queue: TaskQueueKind::Background,
                origin: TaskHandoffOrigin::Papyrus,
                require_safe_gameplay_phase: false,
            }
        );
        assert_eq!(
            ui_handoff_from_event(),
            TaskHandoff {
                queue: TaskQueueKind::Ui,
                origin: TaskHandoffOrigin::Event,
                require_safe_gameplay_phase: false,
            }
        );
        assert_eq!(
            gameplay_handoff_from_event(),
            TaskHandoff {
                queue: TaskQueueKind::Background,
                origin: TaskHandoffOrigin::Event,
                require_safe_gameplay_phase: true,
            }
        );
    }

    #[test]
    fn gameplay_handoff_is_gated_by_phase_snapshot() {
        let safe = gameplay_phase_snapshot();
        assert!(gameplay_handoff().can_run(safe));
        assert_eq!(gameplay_handoff().phase_blocker(safe), None);

        let mut blocked = gameplay_phase_snapshot();
        blocked.ui.loading_open = true;
        assert!(!gameplay_handoff().can_run(blocked));
        assert_eq!(
            gameplay_handoff().phase_blocker(blocked),
            Some(RuntimePhaseBlocker::LoadingScreen)
        );
    }

    #[test]
    fn plain_handoff_ignores_gameplay_phase_gate() {
        let mut blocked = gameplay_phase_snapshot();
        blocked.ui.player_input_blocked = true;

        let handoff = TaskHandoff::from_event();
        assert!(handoff.can_run(blocked));
        assert_eq!(handoff.phase_blocker(blocked), None);
    }
}
