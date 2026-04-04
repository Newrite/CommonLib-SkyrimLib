use crate::sdk::core::{
    GamePtr, HandleFamilyTarget, ResolvableHandle, Resolved, ResolvedHandle, RuntimePhaseBlocker,
    RuntimePhaseSnapshot, snapshot_runtime_phase,
};

use super::shared::{capture_target_handle, queue_with_kind};

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
    pub const fn event() -> Self {
        Self::new(TaskQueueKind::Background, TaskHandoffOrigin::Event)
    }

    #[inline(always)]
    pub const fn ui_event() -> Self {
        Self::new(TaskQueueKind::Ui, TaskHandoffOrigin::Event)
    }

    #[inline(always)]
    pub const fn papyrus() -> Self {
        Self::new(TaskQueueKind::Background, TaskHandoffOrigin::Papyrus)
    }

    #[inline(always)]
    pub const fn ui_papyrus() -> Self {
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
