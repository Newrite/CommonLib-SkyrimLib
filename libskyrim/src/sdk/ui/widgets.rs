//! Higher-level widget/menu runtime helpers built on top of `menus`,
//! `scaleform`, and `controls`.
//!
//! This layer intentionally mirrors the repeated structure seen in UI-heavy
//! plugins:
//!
//! - a custom menu owns a widget runtime
//! - the runtime queues deferred work until the menu surface is ready
//! - visibility is driven by a reusable HUD/menu policy
//! - `show` / `hide` / `refresh` are requested separately from raw menu open
//!   and close messages

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::format;
use alloc::string::ToString;
use core::marker::PhantomData;

use crate::re::{GFxMovieSetVarType, GFxValue, MenuOpenCloseEvent};
use crate::sdk::ui::controls::{self, UiControlSnapshot};
use crate::sdk::ui::menus::NamedMenu;
use crate::sdk::ui::scaleform::MenuSurface;

#[inline(always)]
fn runtime_is_named_menu_open<M>() -> bool
where
    M: NamedMenu,
{
    #[cfg(test)]
    {
        false
    }

    #[cfg(not(test))]
    {
        crate::sdk::ui::menus::is_named_menu_open::<M>()
    }
}

#[inline(always)]
fn runtime_named_surface<M>() -> Option<MenuSurface>
where
    M: NamedMenu,
{
    #[cfg(test)]
    {
        None
    }

    #[cfg(not(test))]
    {
        crate::sdk::ui::scaleform::named_surface::<M>()
    }
}

#[inline(always)]
fn runtime_open_named_menu<M>()
where
    M: NamedMenu,
{
    #[cfg(not(test))]
    {
        crate::sdk::ui::menus::open_named_menu::<M>();
    }
}

#[inline(always)]
fn runtime_close_named_menu<M>()
where
    M: NamedMenu,
{
    #[cfg(not(test))]
    {
        crate::sdk::ui::menus::close_named_menu::<M>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Result requested by one deferred widget task.
pub enum WidgetTaskFlow {
    Complete,
    Retry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Summary of one pending-task drain pass.
pub struct WidgetTaskDrainResult {
    pub completed: usize,
    pub retried: usize,
}

impl WidgetTaskDrainResult {
    #[inline(always)]
    pub const fn attempted(self) -> usize {
        self.completed + self.retried
    }
}

#[derive(Debug, Clone, Copy)]
/// Snapshot of the current runtime state passed into one queued widget task.
pub struct WidgetTaskContext<'a> {
    pub menu_name: &'static str,
    pub menu_open: bool,
    pub desired_visible: bool,
    pub refresh_requested: bool,
    pub surface: Option<&'a MenuSurface>,
}

/// Deferred widget task stored by [`MenuWidgetRuntime`].
pub type WidgetTask = Box<dyn for<'a> FnMut(WidgetTaskContext<'a>) -> WidgetTaskFlow + 'static>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Request bundle emitted by [`MenuWidgetRuntime::take_requests`].
pub struct WidgetRuntimeRequests {
    pub open_menu: bool,
    pub close_menu: bool,
    pub show: bool,
    pub hide: bool,
    pub refresh: bool,
    pub desired_visible: bool,
}

impl WidgetRuntimeRequests {
    #[inline(always)]
    pub const fn has_any(self) -> bool {
        self.open_menu || self.close_menu || self.show || self.hide || self.refresh
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Read-only state snapshot of one [`MenuWidgetRuntime`].
pub struct WidgetRuntimeSnapshot {
    pub menu_open: bool,
    pub surface_ready: bool,
    pub desired_visible: bool,
    pub open_menu_requested: bool,
    pub close_menu_requested: bool,
    pub show_requested: bool,
    pub hide_requested: bool,
    pub refresh_requested: bool,
    pub pending_task_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Reusable visibility policy for menu-owned widgets or HUD surfaces.
pub struct WidgetVisibilityPolicy {
    pub require_menus_visible: bool,
    pub hide_when_game_paused: bool,
    pub hide_when_major_menu_open: bool,
    pub hide_when_loading_transition: bool,
    pub hide_when_menu_like_context: bool,
    pub hide_when_ui_capturing_input: bool,
    pub hide_when_gameplay_input_suppressed: bool,
}

impl WidgetVisibilityPolicy {
    #[inline(always)]
    /// Visibility policy that behaves like a typical gameplay HUD widget.
    pub const fn hud_like() -> Self {
        Self {
            require_menus_visible: true,
            hide_when_game_paused: false,
            hide_when_major_menu_open: true,
            hide_when_loading_transition: true,
            hide_when_menu_like_context: true,
            hide_when_ui_capturing_input: false,
            hide_when_gameplay_input_suppressed: false,
        }
    }

    #[inline(always)]
    /// Visibility policy for overlay-like surfaces that may stay visible during
    /// more UI states than a classic HUD widget.
    pub const fn overlay_like() -> Self {
        Self {
            require_menus_visible: true,
            hide_when_game_paused: false,
            hide_when_major_menu_open: false,
            hide_when_loading_transition: true,
            hide_when_menu_like_context: false,
            hide_when_ui_capturing_input: false,
            hide_when_gameplay_input_suppressed: false,
        }
    }

    #[inline(always)]
    /// Visibility policy that only hides the widget when the whole UI is
    /// hidden.
    pub const fn hidden_only_when_ui_hidden() -> Self {
        Self {
            require_menus_visible: true,
            hide_when_game_paused: false,
            hide_when_major_menu_open: false,
            hide_when_loading_transition: false,
            hide_when_menu_like_context: false,
            hide_when_ui_capturing_input: false,
            hide_when_gameplay_input_suppressed: false,
        }
    }

    #[inline(always)]
    /// Evaluate this policy against one supplied UI snapshot.
    pub const fn should_show(self, snapshot: UiControlSnapshot) -> bool {
        if self.require_menus_visible && !snapshot.menus_visible {
            return false;
        }

        if self.hide_when_game_paused && snapshot.game_paused {
            return false;
        }

        if self.hide_when_major_menu_open && snapshot.has_major_menu_open() {
            return false;
        }

        if self.hide_when_loading_transition && snapshot.has_loading_transition() {
            return false;
        }

        if self.hide_when_menu_like_context && snapshot.menu_like_context {
            return false;
        }

        if self.hide_when_ui_capturing_input && snapshot.is_ui_capturing_input() {
            return false;
        }

        if self.hide_when_gameplay_input_suppressed && snapshot.is_gameplay_input_suppressed() {
            return false;
        }

        true
    }

    #[inline(always)]
    /// Evaluate this policy against the live UI state.
    pub fn evaluate(self) -> bool {
        self.should_show(controls::control_snapshot())
    }
}

impl Default for WidgetVisibilityPolicy {
    #[inline(always)]
    fn default() -> Self {
        Self::hud_like()
    }
}

/// Menu-owned widget runtime with deferred surface tasks and request tracking.
pub struct MenuWidgetRuntime<M> {
    menu_open: bool,
    desired_visible: bool,
    open_menu_requested: bool,
    close_menu_requested: bool,
    show_requested: bool,
    hide_requested: bool,
    refresh_requested: bool,
    pending_tasks: VecDeque<WidgetTask>,
    _marker: PhantomData<fn() -> M>,
}

impl<M> Default for MenuWidgetRuntime<M>
where
    M: NamedMenu,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<M> MenuWidgetRuntime<M>
where
    M: NamedMenu,
{
    #[inline(always)]
    fn movie_surface_flow(surface: &MenuSurface) -> Option<WidgetTaskFlow> {
        if surface.has_movie_view() {
            None
        } else {
            Some(WidgetTaskFlow::Retry)
        }
    }

    #[inline(always)]
    /// Create a runtime seeded from the menu's current open state.
    pub fn new() -> Self {
        Self {
            menu_open: runtime_is_named_menu_open::<M>(),
            desired_visible: false,
            open_menu_requested: false,
            close_menu_requested: false,
            show_requested: false,
            hide_requested: false,
            refresh_requested: false,
            pending_tasks: VecDeque::new(),
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn menu_name(&self) -> &'static str {
        M::MENU_NAME
    }

    #[inline(always)]
    pub const fn is_menu_open(&self) -> bool {
        self.menu_open
    }

    #[inline(always)]
    pub const fn desired_visible(&self) -> bool {
        self.desired_visible
    }

    #[inline(always)]
    pub fn pending_task_count(&self) -> usize {
        self.pending_tasks.len()
    }

    #[inline(always)]
    pub fn has_pending_tasks(&self) -> bool {
        !self.pending_tasks.is_empty()
    }

    #[inline(always)]
    pub fn surface(&self) -> Option<MenuSurface> {
        runtime_named_surface::<M>()
    }

    #[inline(always)]
    pub fn has_surface(&self) -> bool {
        self.surface().is_some()
    }

    /// Return a read-only snapshot of the current runtime state.
    pub fn snapshot(&self) -> WidgetRuntimeSnapshot {
        WidgetRuntimeSnapshot {
            menu_open: self.menu_open,
            surface_ready: self.menu_open && self.has_surface(),
            desired_visible: self.desired_visible,
            open_menu_requested: self.open_menu_requested,
            close_menu_requested: self.close_menu_requested,
            show_requested: self.show_requested,
            hide_requested: self.hide_requested,
            refresh_requested: self.refresh_requested,
            pending_task_count: self.pending_task_count(),
        }
    }

    #[inline(always)]
    /// Refresh the cached open/closed state from the live UI singleton.
    pub fn sync_menu_state(&mut self) -> bool {
        let was_open = self.menu_open;
        self.menu_open = runtime_is_named_menu_open::<M>();
        was_open != self.menu_open
    }

    /// Reconcile cached menu state from one `MenuOpenCloseEvent`.
    ///
    /// Returns `false` when the event belongs to a different menu.
    pub fn update_from_open_close_event(&mut self, event: &MenuOpenCloseEvent) -> bool {
        if event.menu_name.as_str() != M::MENU_NAME {
            return false;
        }

        self.menu_open = event.opening;
        if event.opening {
            self.open_menu_requested = false;
        } else {
            self.close_menu_requested = false;
        }
        true
    }

    #[inline(always)]
    /// Queue one deferred widget task.
    pub fn queue_task(&mut self, task: WidgetTask) {
        self.pending_tasks.push_back(task);
    }

    #[inline(always)]
    /// Queue one deferred widget task as a closure.
    pub fn queue_task_fn<F>(&mut self, task: F)
    where
        F: for<'a> FnMut(WidgetTaskContext<'a>) -> WidgetTaskFlow + 'static,
    {
        self.queue_task(Box::new(task));
    }

    #[inline(always)]
    /// Queue work that only runs once the menu surface exists.
    pub fn queue_surface_task<F>(&mut self, mut task: F)
    where
        F: FnMut(&MenuSurface) -> WidgetTaskFlow + 'static,
    {
        self.queue_task_fn(move |context| {
            let Some(surface) = context.surface else {
                return WidgetTaskFlow::Retry;
            };
            task(surface)
        });
    }

    #[inline(always)]
    /// Queue work that waits for a particular Scaleform path to become
    /// available before running.
    pub fn queue_available_surface_task<F>(&mut self, path: &str, mut task: F)
    where
        F: FnMut(&MenuSurface) -> WidgetTaskFlow + 'static,
    {
        let path = path.to_string();

        self.queue_surface_task(move |surface| {
            if let Some(flow) = Self::movie_surface_flow(surface) {
                return flow;
            }

            match surface.is_available(&path) {
                Ok(true) => task(surface),
                Ok(false) => WidgetTaskFlow::Retry,
                Err(_) => WidgetTaskFlow::Complete,
            }
        });
    }

    #[inline(always)]
    /// Queue one fire-and-forget Scaleform invocation.
    pub fn queue_invoke_no_return_task(&mut self, method_name: &str, args: &[GFxValue]) {
        let method_name = method_name.to_string();
        let args = args.to_vec();

        self.queue_surface_task(move |surface| {
            if let Some(flow) = Self::movie_surface_flow(surface) {
                return flow;
            }

            let _ = surface.invoke_no_return(&method_name, &args);
            WidgetTaskFlow::Complete
        });
    }

    #[inline(always)]
    /// Queue one fire-and-forget Scaleform invocation with no arguments.
    pub fn queue_invoke_no_args_task(&mut self, method_name: &str) {
        self.queue_invoke_no_return_task(method_name, &[]);
    }

    #[inline(always)]
    /// Queue one generic Scaleform variable write.
    pub fn queue_set_variable_task(
        &mut self,
        path: &str,
        value: &GFxValue,
        set_type: GFxMovieSetVarType,
    ) {
        let path = path.to_string();
        let value = value.clone();

        self.queue_surface_task(move |surface| {
            if let Some(flow) = Self::movie_surface_flow(surface) {
                return flow;
            }

            let _ = surface.set_variable(&path, &value, set_type);
            WidgetTaskFlow::Complete
        });
    }

    #[inline(always)]
    /// Queue one boolean Scaleform variable write.
    pub fn queue_set_variable_bool_task(
        &mut self,
        path: &str,
        value: bool,
        set_type: GFxMovieSetVarType,
    ) {
        let path = path.to_string();

        self.queue_surface_task(move |surface| {
            if let Some(flow) = Self::movie_surface_flow(surface) {
                return flow;
            }

            let _ = surface.set_variable_bool(&path, value, set_type);
            WidgetTaskFlow::Complete
        });
    }

    #[inline(always)]
    /// Queue one numeric Scaleform variable write.
    pub fn queue_set_variable_number_task(
        &mut self,
        path: &str,
        value: f64,
        set_type: GFxMovieSetVarType,
    ) {
        let path = path.to_string();

        self.queue_surface_task(move |surface| {
            if let Some(flow) = Self::movie_surface_flow(surface) {
                return flow;
            }

            let _ = surface.set_variable_number(&path, value, set_type);
            WidgetTaskFlow::Complete
        });
    }

    #[inline(always)]
    /// Queue one `_visible` property update for a Scaleform object path.
    pub fn queue_set_visible_task(
        &mut self,
        path: &str,
        visible: bool,
        set_type: GFxMovieSetVarType,
    ) {
        self.queue_set_variable_bool_task(&format!("{path}._visible"), visible, set_type);
    }

    fn drain_pending_tasks(&mut self, sync_menu_state: bool) -> WidgetTaskDrainResult {
        if sync_menu_state {
            self.sync_menu_state();
        }

        let surface = if self.menu_open { self.surface() } else { None };
        let mut remaining = VecDeque::new();
        let mut result = WidgetTaskDrainResult::default();

        while let Some(mut task) = self.pending_tasks.pop_front() {
            let flow = task(WidgetTaskContext {
                menu_name: M::MENU_NAME,
                menu_open: self.menu_open,
                desired_visible: self.desired_visible,
                refresh_requested: self.refresh_requested,
                surface: surface.as_ref(),
            });

            match flow {
                WidgetTaskFlow::Complete => result.completed += 1,
                WidgetTaskFlow::Retry => {
                    result.retried += 1;
                    remaining.push_back(task);
                }
            }
        }

        self.pending_tasks = remaining;
        result
    }

    #[inline(always)]
    /// Drain pending tasks, refreshing cached menu state first.
    pub fn run_pending_tasks(&mut self) -> WidgetTaskDrainResult {
        self.drain_pending_tasks(true)
    }

    #[inline(always)]
    /// Drain pending tasks while preserving menu state already updated from a
    /// menu event in the same cycle.
    pub fn run_pending_tasks_with_cached_menu_state(&mut self) -> WidgetTaskDrainResult {
        self.drain_pending_tasks(false)
    }

    /// Request that the menu be opened if it is not already open.
    pub fn ensure_menu_open(&mut self) {
        if self.menu_open || self.open_menu_requested {
            return;
        }

        self.open_menu_requested = true;
        self.close_menu_requested = false;
        runtime_open_named_menu::<M>();
    }

    /// Request that the menu be closed.
    pub fn request_menu_close(&mut self) {
        if self.close_menu_requested {
            return;
        }

        self.close_menu_requested = true;
        self.open_menu_requested = false;
        runtime_close_named_menu::<M>();
    }

    /// Request that the widget become visible.
    pub fn request_show(&mut self) {
        self.desired_visible = true;
        self.show_requested = true;
        self.hide_requested = false;
        self.ensure_menu_open();
    }

    /// Request that the widget become hidden.
    pub fn request_hide(&mut self) {
        self.desired_visible = false;
        self.hide_requested = true;
        self.show_requested = false;
    }

    /// Request one plugin-side refresh pass.
    pub fn request_refresh(&mut self) {
        self.refresh_requested = true;
        self.ensure_menu_open();
    }

    /// Request one desired visibility state and derive the matching
    /// show/hide/open requests.
    pub fn request_visibility(&mut self, visible: bool) -> bool {
        if self.desired_visible == visible {
            return false;
        }

        if visible {
            self.request_show();
        } else {
            self.request_hide();
        }

        true
    }

    /// Apply one visibility policy against the current live UI snapshot.
    pub fn sync_visibility_with_policy(&mut self, policy: WidgetVisibilityPolicy) -> bool {
        self.request_visibility(policy.evaluate())
    }

    #[inline(always)]
    /// Clear all pending request bits without touching queued tasks.
    pub fn clear_requests(&mut self) {
        self.open_menu_requested = false;
        self.close_menu_requested = false;
        self.show_requested = false;
        self.hide_requested = false;
        self.refresh_requested = false;
    }

    /// Take the current request bundle and clear the stored request bits.
    pub fn take_requests(&mut self) -> WidgetRuntimeRequests {
        let requests = WidgetRuntimeRequests {
            open_menu: self.open_menu_requested,
            close_menu: self.close_menu_requested,
            show: self.show_requested,
            hide: self.hide_requested,
            refresh: self.refresh_requested,
            desired_visible: self.desired_visible,
        };
        self.clear_requests();
        requests
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::re::{INPUT_CONTEXT_ID, MenuOpenCloseEvent};
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
    fn hud_policy_matches_expected_visibility_rules() {
        let policy = WidgetVisibilityPolicy::hud_like();
        let gameplay = gameplay_snapshot();
        let mut paused = gameplay;
        paused.item_menu_open = true;
        paused.inventory_open = true;
        let mut loading = gameplay;
        loading.loading_open = true;

        assert!(policy.should_show(gameplay));
        assert!(!policy.should_show(paused));
        assert!(!policy.should_show(loading));
    }

    #[test]
    fn runtime_tracks_show_hide_refresh_requests() {
        let mut runtime = MenuWidgetRuntime::<TestMenu>::new();

        runtime.request_show();
        let requests = runtime.take_requests();
        assert!(requests.open_menu);
        assert!(requests.show);
        assert!(!requests.hide);
        assert!(requests.desired_visible);

        runtime.request_refresh();
        runtime.request_hide();
        let requests = runtime.take_requests();
        assert!(requests.refresh);
        assert!(requests.hide);
        assert!(!requests.show);
        assert!(!requests.desired_visible);
    }

    #[test]
    fn open_close_event_updates_cached_menu_state() {
        let mut runtime = MenuWidgetRuntime::<TestMenu>::new();
        let mut event = MenuOpenCloseEvent {
            menu_name: crate::re::BSFixedString::from_str(TestMenu::MENU_NAME),
            opening: true,
            pad09: 0,
            pad0a: 0,
            pad0c: 0,
        };

        assert!(runtime.update_from_open_close_event(&event));
        assert!(runtime.is_menu_open());

        event.opening = false;
        assert!(runtime.update_from_open_close_event(&event));
        assert!(!runtime.is_menu_open());
    }

    #[test]
    fn surface_tasks_retry_until_surface_is_available() {
        let mut runtime = MenuWidgetRuntime::<TestMenu>::new();

        runtime.queue_surface_task(|_| WidgetTaskFlow::Complete);

        let result = runtime.run_pending_tasks();
        assert_eq!(result.completed, 0);
        assert_eq!(result.retried, 1);
        assert_eq!(runtime.pending_task_count(), 1);
    }

    #[test]
    fn generic_tasks_can_complete_without_surface() {
        let mut runtime = MenuWidgetRuntime::<TestMenu>::new();
        let completed = Rc::new(Cell::new(false));
        let completed_flag = Rc::clone(&completed);

        runtime.queue_task_fn(move |_| {
            completed_flag.set(true);
            WidgetTaskFlow::Complete
        });

        let result = runtime.run_pending_tasks();
        assert_eq!(result.completed, 1);
        assert_eq!(result.retried, 0);
        assert!(completed.get());
        assert!(!runtime.has_pending_tasks());
    }

    #[test]
    fn scaleform_helper_tasks_retry_until_surface_is_available() {
        let mut runtime = MenuWidgetRuntime::<TestMenu>::new();

        runtime.queue_invoke_no_args_task("widget.refresh");
        runtime.queue_set_variable_bool_task(
            "_root.widget.visible",
            true,
            GFxMovieSetVarType::kSticky,
        );
        runtime.queue_set_variable_number_task(
            "_root.widget.alpha",
            80.0,
            GFxMovieSetVarType::kSticky,
        );
        runtime.queue_set_visible_task("_root.widget", true, GFxMovieSetVarType::kSticky);
        runtime.queue_available_surface_task("_root.widget", |_| WidgetTaskFlow::Complete);

        let result = runtime.run_pending_tasks();
        assert_eq!(result.completed, 0);
        assert_eq!(result.retried, 5);
        assert_eq!(runtime.pending_task_count(), 5);
    }
}
