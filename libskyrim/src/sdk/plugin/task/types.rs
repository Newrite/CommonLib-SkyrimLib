use crate::sdk::core::{
    GamePtr, HandleFamilyTarget, ResolvableHandle, Resolved, ResolvedHandle, RuntimePhaseBlocker,
    RuntimePhaseSnapshot, snapshot_runtime_phase,
};

use super::shared::{capture_target_handle, queue_with_kind};

/// Target SKSE task queue for a deferred handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskQueueKind {
    /// Queue work onto the regular background SKSE task interface.
    Background,
    /// Queue work onto the UI task interface.
    Ui,
}

impl TaskQueueKind {
    /// Stable string label used in diagnostics.
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Background => "background",
            Self::Ui => "ui",
        }
    }
}

/// High-level provenance label for deferred work.
///
/// This is diagnostic and authoring metadata: it helps plugin code express why
/// work is being deferred without changing the actual queue mechanics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskHandoffOrigin {
    /// Generic deferred work with no stronger provenance.
    Generic,
    /// Work deferred out of an event listener or dispatcher callback.
    Event,
    /// Work deferred out of Papyrus registration/callback code.
    Papyrus,
}

impl TaskHandoffOrigin {
    /// Stable string label used in diagnostics.
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::Event => "event",
            Self::Papyrus => "papyrus",
        }
    }
}

/// Declarative description of how deferred work should be queued.
///
/// This is the core policy object behind the `sdk::plugin::task` helpers:
///
/// - which queue should run the work
/// - what high-level origin label it carries
/// - whether it should be suppressed unless the current phase is gameplay-safe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskHandoff {
    /// Target SKSE queue used for dispatch.
    pub queue: TaskQueueKind,
    /// High-level provenance of the deferred work.
    pub origin: TaskHandoffOrigin,
    /// Whether the closure should be skipped unless the current runtime phase
    /// is gameplay-safe.
    pub require_safe_gameplay_phase: bool,
}

impl TaskHandoff {
    /// Build a new handoff description from an explicit queue/origin pair.
    #[inline(always)]
    pub const fn new(queue: TaskQueueKind, origin: TaskHandoffOrigin) -> Self {
        Self {
            queue,
            origin,
            require_safe_gameplay_phase: false,
        }
    }

    /// Generic background work with no special origin label.
    #[inline(always)]
    pub const fn background() -> Self {
        Self::new(TaskQueueKind::Background, TaskHandoffOrigin::Generic)
    }

    /// Generic UI-bound work with no special origin label.
    #[inline(always)]
    pub const fn ui() -> Self {
        Self::new(TaskQueueKind::Ui, TaskHandoffOrigin::Generic)
    }

    /// Background work coming from an event callback.
    #[inline(always)]
    pub const fn event() -> Self {
        Self::new(TaskQueueKind::Background, TaskHandoffOrigin::Event)
    }

    /// UI work coming from an event callback.
    #[inline(always)]
    pub const fn ui_event() -> Self {
        Self::new(TaskQueueKind::Ui, TaskHandoffOrigin::Event)
    }

    /// Background work coming from Papyrus registration/callback code.
    #[inline(always)]
    pub const fn papyrus() -> Self {
        Self::new(TaskQueueKind::Background, TaskHandoffOrigin::Papyrus)
    }

    /// UI work coming from Papyrus registration/callback code.
    #[inline(always)]
    pub const fn ui_papyrus() -> Self {
        Self::new(TaskQueueKind::Ui, TaskHandoffOrigin::Papyrus)
    }

    /// Require the closure to run only during a gameplay-safe phase.
    ///
    /// Use this when the callback may arrive at any time, but the actual
    /// gameplay mutation should not happen during loading, fading, or other
    /// unsafe runtime phases.
    #[inline(always)]
    pub const fn requiring_safe_gameplay_phase(mut self) -> Self {
        self.require_safe_gameplay_phase = true;
        self
    }

    /// Return the runtime-phase blocker that would currently prevent this
    /// handoff from running.
    ///
    /// This is mainly useful for diagnostics or advanced policy decisions;
    /// most plugin code will use [`Self::can_run`] or the higher-level queue
    /// helpers instead.
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

    /// Check whether this handoff can run under the supplied phase snapshot.
    ///
    /// This is the query form of [`Self::phase_blocker`].
    #[inline(always)]
    pub const fn can_run(self, snapshot: RuntimePhaseSnapshot) -> bool {
        self.phase_blocker(snapshot).is_none()
    }

    /// Queue a closure according to the handoff settings.
    ///
    /// Prefer the higher-level `queue_*` helpers when they already match your
    /// workflow. Reach for this method directly when plugin code wants to keep
    /// one reusable handoff policy and dispatch multiple closures through it.
    pub fn dispatch(self, f: impl FnOnce() + Send + 'static) {
        queue_with_kind(self.queue, move || {
            if let Some(blocker) = self.phase_blocker(snapshot_runtime_phase()) {
                let _blocker_name = blocker.as_str();
                crate::defensive_sdk_warn!(
                    "sdk::plugin::task::{:?} {} handoff skipped during {} phase",
                    self.queue,
                    self.origin.as_str(),
                    _blocker_name
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
        // Capture now, resolve later is the main reason this helper exists:
        // callbacks can cheaply retain a handle without borrowing the live
        // object across queued execution.
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

    /// Capture a handle-family target now and resolve it inside the queued
    /// closure when it runs.
    ///
    /// Use this when the callback naturally has a target pointer/object now,
    /// but the queued task should resolve a fresh retained `Resolved<T>` later.
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

    /// Capture a handle from a borrowed target reference and resolve it inside
    /// the queued closure when it runs.
    ///
    /// This is the narrow borrowed-reference variant of
    /// [`Self::dispatch_resolving_target`].
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
