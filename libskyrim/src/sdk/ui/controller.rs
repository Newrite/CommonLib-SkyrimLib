//! Higher-level controller recipes over `driver`.
//!
//! This layer mirrors the common plugin pattern where one manager/controller:
//!
//! - owns a menu/HUD driver
//! - reacts to per-cycle request bundles
//! - translates those bundles into menu-runtime methods such as
//!   `show/hide/refresh/set mode`

use crate::re::MenuOpenCloseEvent;
use crate::sdk::ui::controls::UiControlSnapshot;
use crate::sdk::ui::driver::{
    HudRuntimeDriveResult, HudRuntimeDriver, MenuWidgetDriveResult, MenuWidgetDriver,
};
use crate::sdk::ui::hud_runtime::{HudRefreshFlags, HudRuntimeRequests, HudVisibilityMode};
use crate::sdk::ui::menus::NamedMenu;
use crate::sdk::ui::widgets::{WidgetRuntimeRequests, WidgetVisibilityPolicy};

/// Plugin-implemented handler for [`WidgetRuntimeRequests`].
///
/// This trait is intentionally minimal and callback-style: the driver/runtime
/// stack computes request bundles, then a controller dispatches those requests
/// here using plugin-specific behavior.
pub trait WidgetRequestHandler {
    #[inline(always)]
    fn on_open_menu_requested(&mut self) {}

    #[inline(always)]
    fn on_close_menu_requested(&mut self) {}

    #[inline(always)]
    fn on_show_requested(&mut self) {}

    #[inline(always)]
    fn on_hide_requested(&mut self) {}

    #[inline(always)]
    fn on_refresh_requested(&mut self) {}
}

/// HUD-specific extension of [`WidgetRequestHandler`].
pub trait HudRequestHandler: WidgetRequestHandler {
    #[inline(always)]
    fn on_visibility_mode_requested(&mut self, _mode: HudVisibilityMode) {}

    #[inline(always)]
    fn on_refresh_flags_requested(&mut self, _flags: HudRefreshFlags) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Summary of which widget request callbacks actually fired.
pub struct WidgetRequestDispatch {
    pub open_menu: bool,
    pub close_menu: bool,
    pub show: bool,
    pub hide: bool,
    pub refresh: bool,
}

impl WidgetRequestDispatch {
    #[inline(always)]
    pub const fn has_any(self) -> bool {
        self.open_menu || self.close_menu || self.show || self.hide || self.refresh
    }
}

/// Dispatch one widget request bundle into a handler.
pub fn dispatch_widget_requests<H>(
    handler: &mut H,
    requests: WidgetRuntimeRequests,
) -> WidgetRequestDispatch
where
    H: WidgetRequestHandler + ?Sized,
{
    let mut dispatched = WidgetRequestDispatch::default();

    if requests.open_menu {
        handler.on_open_menu_requested();
        dispatched.open_menu = true;
    }

    if requests.close_menu {
        handler.on_close_menu_requested();
        dispatched.close_menu = true;
    }

    if requests.show {
        handler.on_show_requested();
        dispatched.show = true;
    }

    if requests.hide {
        handler.on_hide_requested();
        dispatched.hide = true;
    }

    if requests.refresh {
        handler.on_refresh_requested();
        dispatched.refresh = true;
    }

    dispatched
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Summary of which HUD request callbacks actually fired.
pub struct HudRequestDispatch {
    pub widget: WidgetRequestDispatch,
    pub visibility_mode: bool,
    pub refresh_flags: bool,
}

impl HudRequestDispatch {
    #[inline(always)]
    pub const fn has_any(self) -> bool {
        self.widget.has_any() || self.visibility_mode || self.refresh_flags
    }
}

/// Dispatch one HUD request bundle into a handler.
pub fn dispatch_hud_requests<H>(handler: &mut H, requests: HudRuntimeRequests) -> HudRequestDispatch
where
    H: HudRequestHandler + ?Sized,
{
    let widget = dispatch_widget_requests(handler, requests.widget);
    let mut dispatched = HudRequestDispatch {
        widget,
        visibility_mode: false,
        refresh_flags: false,
    };

    if let Some(mode) = requests.visibility_mode {
        handler.on_visibility_mode_requested(mode);
        dispatched.visibility_mode = true;
    }

    if !requests.refresh_flags.is_empty() {
        handler.on_refresh_flags_requested(requests.refresh_flags);
        dispatched.refresh_flags = true;
    }

    dispatched
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Result of one [`MenuWidgetController`] cycle.
pub struct MenuWidgetControllerResult {
    pub drive: MenuWidgetDriveResult,
    pub dispatched: WidgetRequestDispatch,
}

impl MenuWidgetControllerResult {
    #[inline(always)]
    pub const fn has_any(self) -> bool {
        self.drive.has_any() || self.dispatched.has_any()
    }
}

/// Higher-level controller that owns both a widget driver and a request
/// handler.
pub struct MenuWidgetController<M, H> {
    driver: MenuWidgetDriver<M>,
    handler: H,
}

impl<M, H> MenuWidgetController<M, H>
where
    M: NamedMenu,
    H: WidgetRequestHandler,
{
    #[inline(always)]
    /// Create a controller with a default driver.
    pub fn new(handler: H) -> Self {
        Self {
            driver: MenuWidgetDriver::new(),
            handler,
        }
    }

    #[inline(always)]
    /// Create a controller with one installed visibility policy.
    pub fn with_policy(handler: H, policy: WidgetVisibilityPolicy) -> Self {
        Self {
            driver: MenuWidgetDriver::with_policy(policy),
            handler,
        }
    }

    #[inline(always)]
    /// Create a controller around an existing driver.
    pub fn with_driver(driver: MenuWidgetDriver<M>, handler: H) -> Self {
        Self { driver, handler }
    }

    #[inline(always)]
    pub fn driver(&self) -> &MenuWidgetDriver<M> {
        &self.driver
    }

    #[inline(always)]
    pub fn driver_mut(&mut self) -> &mut MenuWidgetDriver<M> {
        &mut self.driver
    }

    #[inline(always)]
    pub fn handler(&self) -> &H {
        &self.handler
    }

    #[inline(always)]
    pub fn handler_mut(&mut self) -> &mut H {
        &mut self.handler
    }

    #[inline(always)]
    pub fn into_parts(self) -> (MenuWidgetDriver<M>, H) {
        (self.driver, self.handler)
    }

    #[inline(always)]
    fn finish_cycle(&mut self, drive: MenuWidgetDriveResult) -> MenuWidgetControllerResult {
        let dispatched = dispatch_widget_requests(&mut self.handler, drive.requests);
        MenuWidgetControllerResult { drive, dispatched }
    }

    #[inline(always)]
    /// Run one cycle without a menu event, using the live UI snapshot.
    pub fn drive(&mut self) -> MenuWidgetControllerResult {
        let drive = self.driver.drive();
        self.finish_cycle(drive)
    }

    #[inline(always)]
    /// Run one cycle without a menu event, using a supplied UI snapshot.
    pub fn drive_with_snapshot(
        &mut self,
        snapshot: UiControlSnapshot,
    ) -> MenuWidgetControllerResult {
        let drive = self.driver.drive_with_snapshot(snapshot);
        self.finish_cycle(drive)
    }

    #[inline(always)]
    /// Run one cycle that begins with a `MenuOpenCloseEvent`.
    pub fn drive_menu_event(&mut self, event: &MenuOpenCloseEvent) -> MenuWidgetControllerResult {
        let drive = self.driver.drive_menu_event(event);
        self.finish_cycle(drive)
    }

    #[inline(always)]
    /// Run one cycle that begins with a `MenuOpenCloseEvent` and uses a
    /// supplied UI snapshot.
    pub fn drive_menu_event_with_snapshot(
        &mut self,
        event: &MenuOpenCloseEvent,
        snapshot: UiControlSnapshot,
    ) -> MenuWidgetControllerResult {
        let drive = self.driver.drive_menu_event_with_snapshot(event, snapshot);
        self.finish_cycle(drive)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Result of one [`HudRuntimeController`] cycle.
pub struct HudRuntimeControllerResult {
    pub drive: HudRuntimeDriveResult,
    pub dispatched: HudRequestDispatch,
}

impl HudRuntimeControllerResult {
    #[inline(always)]
    pub const fn has_any(self) -> bool {
        self.drive.has_any() || self.dispatched.has_any()
    }
}

/// Higher-level controller that owns both a HUD driver and a HUD request
/// handler.
pub struct HudRuntimeController<M, H> {
    driver: HudRuntimeDriver<M>,
    handler: H,
}

impl<M, H> HudRuntimeController<M, H>
where
    M: NamedMenu,
    H: HudRequestHandler,
{
    #[inline(always)]
    /// Create a controller with a default HUD driver.
    pub fn new(handler: H) -> Self {
        Self {
            driver: HudRuntimeDriver::new(),
            handler,
        }
    }

    #[inline(always)]
    /// Create a controller with one installed visibility policy.
    pub fn with_policy(handler: H, policy: WidgetVisibilityPolicy) -> Self {
        Self {
            driver: HudRuntimeDriver::with_policy(policy),
            handler,
        }
    }

    #[inline(always)]
    /// Create a controller around an existing HUD driver.
    pub fn with_driver(driver: HudRuntimeDriver<M>, handler: H) -> Self {
        Self { driver, handler }
    }

    #[inline(always)]
    pub fn driver(&self) -> &HudRuntimeDriver<M> {
        &self.driver
    }

    #[inline(always)]
    pub fn driver_mut(&mut self) -> &mut HudRuntimeDriver<M> {
        &mut self.driver
    }

    #[inline(always)]
    pub fn handler(&self) -> &H {
        &self.handler
    }

    #[inline(always)]
    pub fn handler_mut(&mut self) -> &mut H {
        &mut self.handler
    }

    #[inline(always)]
    pub fn into_parts(self) -> (HudRuntimeDriver<M>, H) {
        (self.driver, self.handler)
    }

    #[inline(always)]
    fn finish_cycle(&mut self, drive: HudRuntimeDriveResult) -> HudRuntimeControllerResult {
        let dispatched = dispatch_hud_requests(&mut self.handler, drive.requests);
        HudRuntimeControllerResult { drive, dispatched }
    }

    #[inline(always)]
    /// Run one cycle without a menu event, using the live UI snapshot.
    pub fn drive(&mut self) -> HudRuntimeControllerResult {
        let drive = self.driver.drive();
        self.finish_cycle(drive)
    }

    #[inline(always)]
    /// Run one cycle without a menu event, using a supplied UI snapshot.
    pub fn drive_with_snapshot(
        &mut self,
        snapshot: UiControlSnapshot,
    ) -> HudRuntimeControllerResult {
        let drive = self.driver.drive_with_snapshot(snapshot);
        self.finish_cycle(drive)
    }

    #[inline(always)]
    /// Run one cycle that begins with a `MenuOpenCloseEvent`.
    pub fn drive_menu_event(&mut self, event: &MenuOpenCloseEvent) -> HudRuntimeControllerResult {
        let drive = self.driver.drive_menu_event(event);
        self.finish_cycle(drive)
    }

    #[inline(always)]
    /// Run one cycle that begins with a `MenuOpenCloseEvent` and uses a
    /// supplied UI snapshot.
    pub fn drive_menu_event_with_snapshot(
        &mut self,
        event: &MenuOpenCloseEvent,
        snapshot: UiControlSnapshot,
    ) -> HudRuntimeControllerResult {
        let drive = self.driver.drive_menu_event_with_snapshot(event, snapshot);
        self.finish_cycle(drive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::re::{BSFixedString, INPUT_CONTEXT_ID};

    #[derive(Default)]
    struct TestWidgetHandler {
        open_menu: usize,
        close_menu: usize,
        show: usize,
        hide: usize,
        refresh: usize,
    }

    impl WidgetRequestHandler for TestWidgetHandler {
        fn on_open_menu_requested(&mut self) {
            self.open_menu += 1;
        }

        fn on_close_menu_requested(&mut self) {
            self.close_menu += 1;
        }

        fn on_show_requested(&mut self) {
            self.show += 1;
        }

        fn on_hide_requested(&mut self) {
            self.hide += 1;
        }

        fn on_refresh_requested(&mut self) {
            self.refresh += 1;
        }
    }

    #[derive(Default)]
    struct TestHudHandler {
        widget: TestWidgetHandler,
        last_visibility_mode: Option<HudVisibilityMode>,
        last_refresh_flags: HudRefreshFlags,
    }

    impl WidgetRequestHandler for TestHudHandler {
        fn on_open_menu_requested(&mut self) {
            self.widget.on_open_menu_requested();
        }

        fn on_close_menu_requested(&mut self) {
            self.widget.on_close_menu_requested();
        }

        fn on_show_requested(&mut self) {
            self.widget.on_show_requested();
        }

        fn on_hide_requested(&mut self) {
            self.widget.on_hide_requested();
        }

        fn on_refresh_requested(&mut self) {
            self.widget.on_refresh_requested();
        }
    }

    impl HudRequestHandler for TestHudHandler {
        fn on_visibility_mode_requested(&mut self, mode: HudVisibilityMode) {
            self.last_visibility_mode = Some(mode);
        }

        fn on_refresh_flags_requested(&mut self, flags: HudRefreshFlags) {
            self.last_refresh_flags = flags;
        }
    }

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
    fn widget_dispatch_calls_matching_handler_methods() {
        let mut handler = TestWidgetHandler::default();
        let requests = WidgetRuntimeRequests {
            open_menu: true,
            close_menu: false,
            show: true,
            hide: false,
            refresh: true,
            desired_visible: true,
        };

        let dispatched = dispatch_widget_requests(&mut handler, requests);

        assert!(dispatched.open_menu);
        assert!(dispatched.show);
        assert!(dispatched.refresh);
        assert_eq!(handler.open_menu, 1);
        assert_eq!(handler.show, 1);
        assert_eq!(handler.refresh, 1);
    }

    #[test]
    fn hud_dispatch_includes_visibility_mode_and_refresh_flags() {
        let mut handler = TestHudHandler::default();
        let requests = HudRuntimeRequests {
            widget: WidgetRuntimeRequests {
                open_menu: false,
                close_menu: false,
                show: true,
                hide: false,
                refresh: true,
                desired_visible: true,
            },
            visibility_mode: Some(HudVisibilityMode::Partial),
            refresh_flags: HudRefreshFlags::bit(5),
        };

        let dispatched = dispatch_hud_requests(&mut handler, requests);

        assert!(dispatched.widget.show);
        assert!(dispatched.visibility_mode);
        assert!(dispatched.refresh_flags);
        assert_eq!(handler.widget.show, 1);
        assert_eq!(handler.widget.refresh, 1);
        assert_eq!(
            handler.last_visibility_mode,
            Some(HudVisibilityMode::Partial)
        );
        assert_eq!(handler.last_refresh_flags, HudRefreshFlags::bit(5));
    }

    #[test]
    fn widget_controller_drives_and_dispatches_through_handler() {
        let mut controller = MenuWidgetController::<TestMenu, TestWidgetHandler>::with_policy(
            TestWidgetHandler::default(),
            WidgetVisibilityPolicy::hud_like(),
        );

        let result = controller.drive_with_snapshot(gameplay_snapshot());

        assert!(result.drive.requests.open_menu);
        assert!(result.drive.requests.show);
        assert!(result.dispatched.open_menu);
        assert!(result.dispatched.show);
        assert_eq!(controller.handler().open_menu, 1);
        assert_eq!(controller.handler().show, 1);
    }

    #[test]
    fn hud_controller_reacts_to_menu_event_and_mode_change() {
        let mut controller = HudRuntimeController::<TestMenu, TestHudHandler>::with_policy(
            TestHudHandler::default(),
            WidgetVisibilityPolicy::hud_like(),
        );
        let event = MenuOpenCloseEvent {
            menu_name: BSFixedString::from_str(TestMenu::MENU_NAME),
            opening: true,
            pad09: 0,
            pad0a: 0,
            pad0c: 0,
        };

        let result = controller.drive_menu_event_with_snapshot(&event, gameplay_snapshot());

        assert!(result.drive.event_matched);
        assert_eq!(
            controller.handler().last_visibility_mode,
            Some(HudVisibilityMode::Visible)
        );
        assert_eq!(controller.handler().widget.show, 1);
    }
}
