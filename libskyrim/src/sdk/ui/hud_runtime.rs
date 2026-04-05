//! HUD-oriented runtime helpers layered over `widgets`.
//!
//! This module packages the repeated orchestration pattern seen in custom HUD
//! plugins:
//!
//! - a menu-backed widget runtime owns deferred tasks
//! - visibility mode changes are tracked separately from raw show/hide requests
//! - refresh work is aggregated until the plugin-side menu runtime consumes it

use crate::re::{GFxMovieSetVarType, GFxValue, MenuOpenCloseEvent};
use crate::sdk::ui::menus::NamedMenu;
use crate::sdk::ui::scaleform::MenuSurface;
use crate::sdk::ui::widgets::{
    MenuWidgetRuntime, WidgetRuntimeRequests, WidgetRuntimeSnapshot, WidgetTask,
    WidgetTaskDrainResult, WidgetTaskFlow, WidgetVisibilityPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Bitmask describing HUD refresh work that still needs to run.
pub struct HudRefreshFlags(u64);

impl HudRefreshFlags {
    #[inline(always)]
    pub const fn none() -> Self {
        Self(0)
    }

    #[inline(always)]
    pub const fn all() -> Self {
        Self(u64::MAX)
    }

    #[inline(always)]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    #[inline(always)]
    pub const fn bit(index: u32) -> Self {
        if index < 64 {
            Self(1u64 << index)
        } else {
            Self::none()
        }
    }

    #[inline(always)]
    pub const fn bits(self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub const fn lower_u32(self) -> u32 {
        self.0 as u32
    }

    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub const fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    #[inline(always)]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[inline(always)]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    #[inline(always)]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}

impl core::ops::BitOr for HudRefreshFlags {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for HudRefreshFlags {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for HudRefreshFlags {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Coarse-grained visibility state for a HUD surface.
pub enum HudVisibilityMode {
    Hidden = 0,
    Partial = 1,
    Visible = 2,
}

impl HudVisibilityMode {
    #[inline(always)]
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    #[inline(always)]
    pub const fn as_f64(self) -> f64 {
        self.as_u32() as f64
    }

    #[inline(always)]
    pub const fn is_visible(self) -> bool {
        !matches!(self, Self::Hidden)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Read-only snapshot of one [`HudRuntime`].
pub struct HudRuntimeSnapshot {
    pub widget: WidgetRuntimeSnapshot,
    pub visibility_mode: HudVisibilityMode,
    pub refresh_flags: HudRefreshFlags,
    pub visibility_mode_changed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Request bundle emitted by [`HudRuntime::take_requests`].
pub struct HudRuntimeRequests {
    pub widget: WidgetRuntimeRequests,
    pub visibility_mode: Option<HudVisibilityMode>,
    pub refresh_flags: HudRefreshFlags,
}

impl HudRuntimeRequests {
    #[inline(always)]
    pub const fn has_any(self) -> bool {
        self.widget.has_any() || self.visibility_mode.is_some() || !self.refresh_flags.is_empty()
    }
}

/// HUD-oriented runtime layered over [`MenuWidgetRuntime`].
///
/// This adds two HUD-specific concerns on top of the lower-level widget
/// runtime:
///
/// - visibility modes such as hidden/partial/visible
/// - aggregated refresh flags that can be consumed in batches
pub struct HudRuntime<M> {
    widgets: MenuWidgetRuntime<M>,
    visibility_mode: HudVisibilityMode,
    visibility_mode_changed: bool,
    refresh_flags: HudRefreshFlags,
}

impl<M> Default for HudRuntime<M>
where
    M: NamedMenu,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<M> HudRuntime<M>
where
    M: NamedMenu,
{
    #[inline(always)]
    /// Create one empty HUD runtime.
    pub fn new() -> Self {
        Self {
            widgets: MenuWidgetRuntime::new(),
            visibility_mode: HudVisibilityMode::Hidden,
            visibility_mode_changed: false,
            refresh_flags: HudRefreshFlags::none(),
        }
    }

    #[inline(always)]
    /// Create one HUD runtime and immediately apply a visibility policy.
    pub fn with_policy(policy: WidgetVisibilityPolicy) -> Self {
        let mut runtime = Self::new();
        runtime.sync_visibility_with_policy(policy);
        runtime
    }

    #[inline(always)]
    pub fn widgets(&self) -> &MenuWidgetRuntime<M> {
        &self.widgets
    }

    #[inline(always)]
    pub fn widgets_mut(&mut self) -> &mut MenuWidgetRuntime<M> {
        &mut self.widgets
    }

    #[inline(always)]
    pub const fn visibility_mode(&self) -> HudVisibilityMode {
        self.visibility_mode
    }

    #[inline(always)]
    pub const fn refresh_flags(&self) -> HudRefreshFlags {
        self.refresh_flags
    }

    #[inline(always)]
    pub const fn has_refresh_flags(&self) -> bool {
        !self.refresh_flags.is_empty()
    }

    #[inline(always)]
    pub const fn has_visibility_mode_change(&self) -> bool {
        self.visibility_mode_changed
    }

    #[inline(always)]
    /// Return a read-only snapshot of the current runtime state.
    pub fn snapshot(&self) -> HudRuntimeSnapshot {
        HudRuntimeSnapshot {
            widget: self.widgets.snapshot(),
            visibility_mode: self.visibility_mode,
            refresh_flags: self.refresh_flags,
            visibility_mode_changed: self.visibility_mode_changed,
        }
    }

    #[inline(always)]
    /// Reconcile cached menu state from one `MenuOpenCloseEvent`.
    pub fn update_from_open_close_event(&mut self, event: &MenuOpenCloseEvent) -> bool {
        self.widgets.update_from_open_close_event(event)
    }

    #[inline(always)]
    /// Drain pending tasks while refreshing cached menu state first.
    pub fn run_pending_tasks(&mut self) -> WidgetTaskDrainResult {
        self.widgets.run_pending_tasks()
    }

    #[inline(always)]
    /// Drain pending tasks while preserving cached menu state updated earlier
    /// in the same cycle.
    pub fn run_pending_tasks_with_cached_menu_state(&mut self) -> WidgetTaskDrainResult {
        self.widgets.run_pending_tasks_with_cached_menu_state()
    }

    #[inline(always)]
    pub fn queue_task(&mut self, task: WidgetTask) {
        self.widgets.queue_task(task);
    }

    #[inline(always)]
    pub fn queue_task_fn<F>(&mut self, task: F)
    where
        F: for<'a> FnMut(crate::sdk::ui::widgets::WidgetTaskContext<'a>) -> WidgetTaskFlow
            + 'static,
    {
        self.widgets.queue_task_fn(task);
    }

    #[inline(always)]
    pub fn queue_surface_task<F>(&mut self, task: F)
    where
        F: FnMut(&MenuSurface) -> WidgetTaskFlow + 'static,
    {
        self.widgets.queue_surface_task(task);
    }

    #[inline(always)]
    pub fn queue_available_surface_task<F>(&mut self, path: &str, task: F)
    where
        F: FnMut(&MenuSurface) -> WidgetTaskFlow + 'static,
    {
        self.widgets.queue_available_surface_task(path, task);
    }

    #[inline(always)]
    pub fn queue_invoke_no_return_task(&mut self, method_name: &str, args: &[GFxValue]) {
        self.widgets.queue_invoke_no_return_task(method_name, args);
    }

    #[inline(always)]
    pub fn queue_invoke_no_args_task(&mut self, method_name: &str) {
        self.widgets.queue_invoke_no_args_task(method_name);
    }

    #[inline(always)]
    pub fn queue_set_variable_task(
        &mut self,
        path: &str,
        value: &GFxValue,
        set_type: GFxMovieSetVarType,
    ) {
        self.widgets.queue_set_variable_task(path, value, set_type);
    }

    #[inline(always)]
    pub fn queue_set_variable_bool_task(
        &mut self,
        path: &str,
        value: bool,
        set_type: GFxMovieSetVarType,
    ) {
        self.widgets
            .queue_set_variable_bool_task(path, value, set_type);
    }

    #[inline(always)]
    pub fn queue_set_variable_number_task(
        &mut self,
        path: &str,
        value: f64,
        set_type: GFxMovieSetVarType,
    ) {
        self.widgets
            .queue_set_variable_number_task(path, value, set_type);
    }

    #[inline(always)]
    pub fn queue_set_visible_task(
        &mut self,
        path: &str,
        visible: bool,
        set_type: GFxMovieSetVarType,
    ) {
        self.widgets.queue_set_visible_task(path, visible, set_type);
    }

    /// Set one visibility mode and derive the matching widget visibility
    /// requests.
    pub fn set_visibility_mode(&mut self, mode: HudVisibilityMode) -> bool {
        if self.visibility_mode == mode {
            return false;
        }

        self.visibility_mode = mode;
        self.visibility_mode_changed = true;

        if mode.is_visible() {
            self.widgets.request_show();
        } else {
            self.widgets.request_hide();
        }

        true
    }

    #[inline(always)]
    /// Request a fully visible HUD state.
    pub fn request_show(&mut self) {
        self.set_visibility_mode(HudVisibilityMode::Visible);
    }

    #[inline(always)]
    /// Request a partially visible HUD state.
    pub fn request_partial_show(&mut self) {
        self.set_visibility_mode(HudVisibilityMode::Partial);
    }

    #[inline(always)]
    /// Request a hidden HUD state.
    pub fn request_hide(&mut self) {
        self.set_visibility_mode(HudVisibilityMode::Hidden);
    }

    /// Apply one visibility policy against the live UI snapshot.
    pub fn sync_visibility_with_policy(&mut self, policy: WidgetVisibilityPolicy) -> bool {
        if policy.evaluate() {
            self.set_visibility_mode(HudVisibilityMode::Visible)
        } else {
            self.set_visibility_mode(HudVisibilityMode::Hidden)
        }
    }

    /// Accumulate refresh flags and request a follow-up widget refresh pass.
    pub fn request_refresh_flags(&mut self, flags: HudRefreshFlags) {
        if flags.is_empty() {
            return;
        }

        self.refresh_flags.insert(flags);
        self.widgets.request_refresh();
    }

    #[inline(always)]
    pub fn request_refresh_all(&mut self) {
        self.request_refresh_flags(HudRefreshFlags::all());
    }

    #[inline(always)]
    pub fn request_refresh_bit(&mut self, index: u32) {
        self.request_refresh_flags(HudRefreshFlags::bit(index));
    }

    #[inline(always)]
    /// Clear the accumulated refresh-bit mask.
    pub fn clear_refresh_flags(&mut self) {
        self.refresh_flags = HudRefreshFlags::none();
    }

    #[inline(always)]
    /// Clear the pending "visibility mode changed" marker.
    pub fn clear_visibility_mode_change(&mut self) {
        self.visibility_mode_changed = false;
    }

    #[inline(always)]
    /// Take the accumulated refresh flags and clear them from runtime state.
    pub fn take_refresh_flags(&mut self) -> HudRefreshFlags {
        let flags = self.refresh_flags;
        self.refresh_flags = HudRefreshFlags::none();
        flags
    }

    #[inline(always)]
    /// Take the pending visibility-mode change, if any.
    pub fn take_visibility_mode_change(&mut self) -> Option<HudVisibilityMode> {
        let visibility_mode = self.visibility_mode_changed.then_some(self.visibility_mode);
        self.visibility_mode_changed = false;
        visibility_mode
    }

    /// Take the current HUD request bundle and clear the stored HUD-side state.
    pub fn take_requests(&mut self) -> HudRuntimeRequests {
        HudRuntimeRequests {
            widget: self.widgets.take_requests(),
            visibility_mode: self.take_visibility_mode_change(),
            refresh_flags: self.take_refresh_flags(),
        }
    }

    #[inline(always)]
    /// Queue one Scaleform variable write for the current visibility mode.
    pub fn queue_visibility_mode_task(&mut self, path: &str, set_type: GFxMovieSetVarType) {
        self.queue_set_variable_number_task(path, self.visibility_mode.as_f64(), set_type);
    }

    #[inline(always)]
    /// Queue one Scaleform method call carrying the current visibility mode.
    pub fn queue_invoke_visibility_mode_task(&mut self, method_name: &str) {
        self.queue_invoke_no_return_task(
            method_name,
            &[GFxValue::from(self.visibility_mode.as_f64())],
        );
    }

    #[inline(always)]
    /// Queue the common pair of visibility-state updates:
    /// one mode value and one boolean visible flag.
    pub fn queue_visibility_state_tasks(
        &mut self,
        mode_path: &str,
        visible_path: &str,
        set_type: GFxMovieSetVarType,
    ) {
        self.queue_visibility_mode_task(mode_path, set_type);
        self.queue_set_variable_bool_task(
            visible_path,
            self.visibility_mode.is_visible(),
            set_type,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::re::INPUT_CONTEXT_ID;
    use crate::sdk::ui::controls::UiControlSnapshot;

    struct TestHudMenu;

    impl NamedMenu for TestHudMenu {
        const MENU_NAME: &'static str = "TestHudMenu";
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
    fn refresh_flags_support_bitmask_style_operations() {
        let mut flags = HudRefreshFlags::bit(1);
        flags.insert(HudRefreshFlags::bit(4));

        assert!(flags.contains(HudRefreshFlags::bit(1)));
        assert!(flags.intersects(HudRefreshFlags::bit(4)));

        flags.remove(HudRefreshFlags::bit(1));
        assert!(!flags.contains(HudRefreshFlags::bit(1)));
        assert_eq!(flags.bits(), HudRefreshFlags::bit(4).bits());
    }

    #[test]
    fn visibility_mode_changes_drive_widget_show_hide_requests() {
        let mut runtime = HudRuntime::<TestHudMenu>::new();

        runtime.request_show();
        let requests = runtime.take_requests();
        assert_eq!(requests.visibility_mode, Some(HudVisibilityMode::Visible));
        assert!(requests.widget.open_menu);
        assert!(requests.widget.show);

        runtime.request_hide();
        let requests = runtime.take_requests();
        assert_eq!(requests.visibility_mode, Some(HudVisibilityMode::Hidden));
        assert!(requests.widget.hide);
    }

    #[test]
    fn partial_visibility_is_tracked_without_forcing_hidden_state() {
        let mut runtime = HudRuntime::<TestHudMenu>::new();

        runtime.request_partial_show();
        let requests = runtime.take_requests();

        assert_eq!(requests.visibility_mode, Some(HudVisibilityMode::Partial));
        assert!(requests.widget.show);
        assert!(requests.widget.desired_visible);
    }

    #[test]
    fn refresh_requests_accumulate_until_taken() {
        let mut runtime = HudRuntime::<TestHudMenu>::new();

        runtime.request_refresh_bit(2);
        runtime.request_refresh_bit(7);
        let requests = runtime.take_requests();

        assert!(requests.widget.refresh);
        assert!(requests.refresh_flags.contains(HudRefreshFlags::bit(2)));
        assert!(requests.refresh_flags.contains(HudRefreshFlags::bit(7)));
        assert!(runtime.refresh_flags().is_empty());
    }

    #[test]
    fn fine_grained_take_helpers_clear_only_hud_state() {
        let mut runtime = HudRuntime::<TestHudMenu>::new();

        runtime.request_partial_show();
        runtime.request_refresh_bit(9);

        assert!(runtime.has_visibility_mode_change());
        assert!(runtime.has_refresh_flags());
        assert_eq!(
            runtime.take_visibility_mode_change(),
            Some(HudVisibilityMode::Partial)
        );
        assert!(!runtime.has_visibility_mode_change());
        assert_eq!(runtime.take_refresh_flags(), HudRefreshFlags::bit(9));
        assert!(!runtime.has_refresh_flags());

        let requests = runtime.take_requests();
        assert!(requests.widget.open_menu);
        assert!(requests.widget.show);
        assert_eq!(requests.visibility_mode, None);
        assert!(requests.refresh_flags.is_empty());
    }

    #[test]
    fn visibility_scaleform_helpers_retry_until_surface_is_available() {
        let mut runtime = HudRuntime::<TestHudMenu>::new();
        runtime.request_partial_show();

        runtime.queue_visibility_mode_task("_root.widget.mode", GFxMovieSetVarType::kSticky);
        runtime.queue_invoke_visibility_mode_task("widget.setMode");
        runtime.queue_visibility_state_tasks(
            "_root.widget.mode",
            "_root.widget.visible",
            GFxMovieSetVarType::kSticky,
        );

        let result = runtime.run_pending_tasks();
        assert_eq!(result.completed, 0);
        assert_eq!(result.retried, 4);
        assert_eq!(runtime.widgets().pending_task_count(), 4);
    }

    #[test]
    fn policy_sync_maps_to_visible_and_hidden_modes() {
        let policy = WidgetVisibilityPolicy::hud_like();
        let gameplay = gameplay_snapshot();
        let mut hidden = gameplay;
        hidden.item_menu_open = true;
        hidden.inventory_open = true;

        assert!(policy.should_show(gameplay));
        assert!(!policy.should_show(hidden));
    }
}
