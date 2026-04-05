//! Menu/HUD runtime driver glue layered over `widgets` and `hud_runtime`.
//!
//! This module mirrors the repeated orchestration pattern seen in UI-heavy
//! plugins:
//!
//! - a menu open/close event updates cached runtime state
//! - visibility policy is re-evaluated against a UI snapshot
//! - deferred widget tasks run while request flags are still visible
//! - the plugin consumes a single per-cycle request bundle afterwards

use crate::re::MenuOpenCloseEvent;
use crate::sdk::ui::controls::{self, UiControlSnapshot};
use crate::sdk::ui::hud_runtime::{
    HudRuntime, HudRuntimeRequests, HudRuntimeSnapshot, HudVisibilityMode,
};
use crate::sdk::ui::menus::NamedMenu;
use crate::sdk::ui::widgets::{
    MenuWidgetRuntime, WidgetRuntimeRequests, WidgetRuntimeSnapshot, WidgetTaskDrainResult,
    WidgetVisibilityPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Result of one [`MenuWidgetDriver`] cycle.
pub struct MenuWidgetDriveResult {
    pub event_matched: bool,
    pub visibility_synced: bool,
    pub snapshot: WidgetRuntimeSnapshot,
    pub requests: WidgetRuntimeRequests,
    pub tasks: WidgetTaskDrainResult,
}

impl MenuWidgetDriveResult {
    #[inline(always)]
    pub const fn has_any(self) -> bool {
        self.event_matched
            || self.visibility_synced
            || self.requests.has_any()
            || self.tasks.completed != 0
            || self.tasks.retried != 0
    }
}

/// Driver that orchestrates one [`MenuWidgetRuntime`] through a typical UI
/// cycle.
pub struct MenuWidgetDriver<M> {
    runtime: MenuWidgetRuntime<M>,
    visibility_policy: Option<WidgetVisibilityPolicy>,
}

impl<M> Default for MenuWidgetDriver<M>
where
    M: NamedMenu,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<M> MenuWidgetDriver<M>
where
    M: NamedMenu,
{
    #[inline(always)]
    /// Create a driver with an empty runtime and no visibility policy.
    pub fn new() -> Self {
        Self {
            runtime: MenuWidgetRuntime::new(),
            visibility_policy: None,
        }
    }

    #[inline(always)]
    /// Create a driver around an existing runtime instance.
    pub fn with_runtime(runtime: MenuWidgetRuntime<M>) -> Self {
        Self {
            runtime,
            visibility_policy: None,
        }
    }

    #[inline(always)]
    /// Create a driver with one installed visibility policy.
    pub fn with_policy(policy: WidgetVisibilityPolicy) -> Self {
        Self {
            runtime: MenuWidgetRuntime::new(),
            visibility_policy: Some(policy),
        }
    }

    #[inline(always)]
    pub fn runtime(&self) -> &MenuWidgetRuntime<M> {
        &self.runtime
    }

    #[inline(always)]
    pub fn runtime_mut(&mut self) -> &mut MenuWidgetRuntime<M> {
        &mut self.runtime
    }

    #[inline(always)]
    pub const fn visibility_policy(&self) -> Option<WidgetVisibilityPolicy> {
        self.visibility_policy
    }

    #[inline(always)]
    pub fn set_visibility_policy(
        &mut self,
        policy: WidgetVisibilityPolicy,
    ) -> Option<WidgetVisibilityPolicy> {
        self.visibility_policy.replace(policy)
    }

    #[inline(always)]
    pub fn clear_visibility_policy(&mut self) -> Option<WidgetVisibilityPolicy> {
        self.visibility_policy.take()
    }

    #[inline(always)]
    /// Feed one `MenuOpenCloseEvent` into the underlying runtime.
    pub fn on_menu_event(&mut self, event: &MenuOpenCloseEvent) -> bool {
        self.runtime.update_from_open_close_event(event)
    }

    /// Re-evaluate the installed visibility policy against one supplied UI
    /// snapshot.
    pub fn sync_visibility_with_snapshot(&mut self, snapshot: UiControlSnapshot) -> bool {
        let Some(policy) = self.visibility_policy else {
            return false;
        };

        self.runtime
            .request_visibility(policy.should_show(snapshot))
    }

    #[inline(always)]
    /// Re-evaluate the installed visibility policy against the live UI state.
    pub fn sync_visibility_policy(&mut self) -> bool {
        self.sync_visibility_with_snapshot(controls::control_snapshot())
    }

    #[inline(always)]
    fn finish_drive(
        &mut self,
        event_matched: bool,
        visibility_synced: bool,
    ) -> MenuWidgetDriveResult {
        let tasks = if event_matched {
            self.runtime.run_pending_tasks_with_cached_menu_state()
        } else {
            self.runtime.run_pending_tasks()
        };
        let snapshot = self.runtime.snapshot();
        let requests = self.runtime.take_requests();

        MenuWidgetDriveResult {
            event_matched,
            visibility_synced,
            snapshot,
            requests,
            tasks,
        }
    }

    #[inline(always)]
    /// Run one cycle without a menu event, using the live UI snapshot.
    pub fn drive(&mut self) -> MenuWidgetDriveResult {
        let visibility_synced = self.sync_visibility_policy();
        self.finish_drive(false, visibility_synced)
    }

    #[inline(always)]
    /// Run one cycle without a menu event, using a supplied UI snapshot.
    pub fn drive_with_snapshot(&mut self, snapshot: UiControlSnapshot) -> MenuWidgetDriveResult {
        let visibility_synced = self.sync_visibility_with_snapshot(snapshot);
        self.finish_drive(false, visibility_synced)
    }

    #[inline(always)]
    /// Run one cycle that begins with a `MenuOpenCloseEvent`.
    pub fn drive_menu_event(&mut self, event: &MenuOpenCloseEvent) -> MenuWidgetDriveResult {
        let event_matched = self.on_menu_event(event);
        let visibility_synced = self.sync_visibility_policy();
        self.finish_drive(event_matched, visibility_synced)
    }

    #[inline(always)]
    /// Run one cycle that begins with a `MenuOpenCloseEvent` and uses a
    /// supplied UI snapshot.
    pub fn drive_menu_event_with_snapshot(
        &mut self,
        event: &MenuOpenCloseEvent,
        snapshot: UiControlSnapshot,
    ) -> MenuWidgetDriveResult {
        let event_matched = self.on_menu_event(event);
        let visibility_synced = self.sync_visibility_with_snapshot(snapshot);
        self.finish_drive(event_matched, visibility_synced)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Result of one [`HudRuntimeDriver`] cycle.
pub struct HudRuntimeDriveResult {
    pub event_matched: bool,
    pub visibility_synced: bool,
    pub snapshot: HudRuntimeSnapshot,
    pub requests: HudRuntimeRequests,
    pub tasks: WidgetTaskDrainResult,
}

impl HudRuntimeDriveResult {
    #[inline(always)]
    pub const fn has_any(self) -> bool {
        self.event_matched
            || self.visibility_synced
            || self.requests.has_any()
            || self.tasks.completed != 0
            || self.tasks.retried != 0
    }
}

/// Driver that orchestrates one [`HudRuntime`] through a typical UI cycle.
pub struct HudRuntimeDriver<M> {
    runtime: HudRuntime<M>,
    visibility_policy: Option<WidgetVisibilityPolicy>,
}

impl<M> Default for HudRuntimeDriver<M>
where
    M: NamedMenu,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<M> HudRuntimeDriver<M>
where
    M: NamedMenu,
{
    #[inline(always)]
    /// Create a driver with an empty HUD runtime and no visibility policy.
    pub fn new() -> Self {
        Self {
            runtime: HudRuntime::new(),
            visibility_policy: None,
        }
    }

    #[inline(always)]
    /// Create a driver around an existing HUD runtime instance.
    pub fn with_runtime(runtime: HudRuntime<M>) -> Self {
        Self {
            runtime,
            visibility_policy: None,
        }
    }

    #[inline(always)]
    /// Create a driver with one installed visibility policy.
    pub fn with_policy(policy: WidgetVisibilityPolicy) -> Self {
        Self {
            runtime: HudRuntime::new(),
            visibility_policy: Some(policy),
        }
    }

    #[inline(always)]
    pub fn runtime(&self) -> &HudRuntime<M> {
        &self.runtime
    }

    #[inline(always)]
    pub fn runtime_mut(&mut self) -> &mut HudRuntime<M> {
        &mut self.runtime
    }

    #[inline(always)]
    pub const fn visibility_policy(&self) -> Option<WidgetVisibilityPolicy> {
        self.visibility_policy
    }

    #[inline(always)]
    pub fn set_visibility_policy(
        &mut self,
        policy: WidgetVisibilityPolicy,
    ) -> Option<WidgetVisibilityPolicy> {
        self.visibility_policy.replace(policy)
    }

    #[inline(always)]
    pub fn clear_visibility_policy(&mut self) -> Option<WidgetVisibilityPolicy> {
        self.visibility_policy.take()
    }

    #[inline(always)]
    /// Feed one `MenuOpenCloseEvent` into the underlying HUD runtime.
    pub fn on_menu_event(&mut self, event: &MenuOpenCloseEvent) -> bool {
        self.runtime.update_from_open_close_event(event)
    }

    /// Re-evaluate the installed visibility policy against one supplied UI
    /// snapshot.
    pub fn sync_visibility_with_snapshot(&mut self, snapshot: UiControlSnapshot) -> bool {
        let Some(policy) = self.visibility_policy else {
            return false;
        };

        if policy.should_show(snapshot) {
            self.runtime.set_visibility_mode(HudVisibilityMode::Visible)
        } else {
            self.runtime.set_visibility_mode(HudVisibilityMode::Hidden)
        }
    }

    #[inline(always)]
    /// Re-evaluate the installed visibility policy against the live UI state.
    pub fn sync_visibility_policy(&mut self) -> bool {
        self.sync_visibility_with_snapshot(controls::control_snapshot())
    }

    #[inline(always)]
    fn finish_drive(
        &mut self,
        event_matched: bool,
        visibility_synced: bool,
    ) -> HudRuntimeDriveResult {
        let tasks = if event_matched {
            self.runtime.run_pending_tasks_with_cached_menu_state()
        } else {
            self.runtime.run_pending_tasks()
        };
        let snapshot = self.runtime.snapshot();
        let requests = self.runtime.take_requests();

        HudRuntimeDriveResult {
            event_matched,
            visibility_synced,
            snapshot,
            requests,
            tasks,
        }
    }

    #[inline(always)]
    /// Run one cycle without a menu event, using the live UI snapshot.
    pub fn drive(&mut self) -> HudRuntimeDriveResult {
        let visibility_synced = self.sync_visibility_policy();
        self.finish_drive(false, visibility_synced)
    }

    #[inline(always)]
    /// Run one cycle without a menu event, using a supplied UI snapshot.
    pub fn drive_with_snapshot(&mut self, snapshot: UiControlSnapshot) -> HudRuntimeDriveResult {
        let visibility_synced = self.sync_visibility_with_snapshot(snapshot);
        self.finish_drive(false, visibility_synced)
    }

    #[inline(always)]
    /// Run one cycle that begins with a `MenuOpenCloseEvent`.
    pub fn drive_menu_event(&mut self, event: &MenuOpenCloseEvent) -> HudRuntimeDriveResult {
        let event_matched = self.on_menu_event(event);
        let visibility_synced = self.sync_visibility_policy();
        self.finish_drive(event_matched, visibility_synced)
    }

    #[inline(always)]
    /// Run one cycle that begins with a `MenuOpenCloseEvent` and uses a
    /// supplied UI snapshot.
    pub fn drive_menu_event_with_snapshot(
        &mut self,
        event: &MenuOpenCloseEvent,
        snapshot: UiControlSnapshot,
    ) -> HudRuntimeDriveResult {
        let event_matched = self.on_menu_event(event);
        let visibility_synced = self.sync_visibility_with_snapshot(snapshot);
        self.finish_drive(event_matched, visibility_synced)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::re::{BSFixedString, INPUT_CONTEXT_ID};
    use alloc::rc::Rc;
    use core::cell::Cell;

    struct TestMenu;

    impl NamedMenu for TestMenu {
        const MENU_NAME: &'static str = "TestMenu";
    }

    fn gameplay_snapshot() -> UiControlSnapshot {
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

    #[test]
    fn widget_driver_syncs_policy_and_drains_requests() {
        let mut driver =
            MenuWidgetDriver::<TestMenu>::with_policy(WidgetVisibilityPolicy::hud_like());

        let result = driver.drive_with_snapshot(gameplay_snapshot());

        assert!(result.visibility_synced);
        assert!(result.requests.open_menu);
        assert!(result.requests.show);
        assert!(result.requests.desired_visible);
        assert!(result.has_any());
    }

    #[test]
    fn widget_driver_runs_tasks_before_request_drain() {
        let mut driver = MenuWidgetDriver::<TestMenu>::new();
        let saw_refresh = Rc::new(Cell::new(false));
        let saw_refresh_flag = Rc::clone(&saw_refresh);

        driver.runtime_mut().request_refresh();
        driver.runtime_mut().queue_task_fn(move |context| {
            saw_refresh_flag.set(context.refresh_requested);
            crate::sdk::ui::widgets::WidgetTaskFlow::Complete
        });

        let result = driver.drive_with_snapshot(gameplay_snapshot());

        assert_eq!(result.tasks.completed, 1);
        assert!(saw_refresh.get());
        assert!(result.requests.refresh);
    }

    #[test]
    fn widget_driver_reconciles_open_event_before_request_drain() {
        let mut driver =
            MenuWidgetDriver::<TestMenu>::with_policy(WidgetVisibilityPolicy::hud_like());
        let mut event = MenuOpenCloseEvent {
            menu_name: BSFixedString::from_str(TestMenu::MENU_NAME),
            opening: true,
            pad09: 0,
            pad0a: 0,
            pad0c: 0,
        };

        let initial = driver.drive_with_snapshot(gameplay_snapshot());
        assert!(initial.requests.open_menu);

        let result = driver.drive_menu_event_with_snapshot(&event, gameplay_snapshot());
        assert!(result.event_matched);
        assert!(result.snapshot.menu_open);
        assert!(!result.requests.open_menu);

        event.opening = false;
        let result = driver.drive_menu_event_with_snapshot(&event, gameplay_snapshot());
        assert!(result.event_matched);
        assert!(!result.snapshot.menu_open);
    }

    #[test]
    fn hud_driver_syncs_policy_to_visible_and_hidden_modes() {
        let mut driver =
            HudRuntimeDriver::<TestMenu>::with_policy(WidgetVisibilityPolicy::hud_like());
        let mut hidden = gameplay_snapshot();
        hidden.item_menu_open = true;
        hidden.inventory_open = true;

        let visible = driver.drive_with_snapshot(gameplay_snapshot());
        assert_eq!(
            visible.requests.visibility_mode,
            Some(HudVisibilityMode::Visible)
        );
        assert!(visible.requests.widget.show);

        let hidden = driver.drive_with_snapshot(hidden);
        assert_eq!(
            hidden.requests.visibility_mode,
            Some(HudVisibilityMode::Hidden)
        );
        assert!(hidden.requests.widget.hide);
    }
}
