//! UI-facing helpers for menus, notifications, and input/UI coordination.
//!
//! This module is intentionally layered from low-level menu/movie access up to
//! menu-owned widget and HUD runtime orchestration:
//!
//! - [`menus`] for queued UI messages, menu-state queries, and typed payloads
//! - [`scaleform`] for `GFxMovieView` / `FxDelegate` access and movie helpers
//! - [`controls`] for UI/input capture and gameplay/HUD visibility state
//! - [`notifications`] for HUD message builders
//! - [`widgets`] for deferred menu-surface tasks and widget runtime state
//! - [`hud_runtime`] for visibility modes and aggregated refresh requests
//! - [`driver`] for event/policy/task orchestration
//! - [`controller`] for controller-style request translation recipes
//!
//! This stack mirrors repeated patterns from UI-heavy plugins such as custom
//! HUDs, menu-owned widgets, and Scaleform-driven overlays.
//!
//! Decision guide:
//!
//! - start at [`menus`] when the plugin only needs menu state, queued
//!   `UIMessage`s, or typed `IUIMessageData`
//! - add [`scaleform`] when the plugin needs `GFxMovieView`, variable writes,
//!   or `invoke(...)`
//! - add [`widgets`] when menu work must be deferred until a real movie surface
//!   exists
//! - add [`hud_runtime`] when visibility mode and refresh aggregation belong to
//!   the plugin's runtime state
//! - add [`driver`] when menu events, cached menu state, and pending tasks must
//!   be reconciled together
//! - add [`controller`] when request bundles should be translated into concrete
//!   show/hide/refresh actions through one handler object
//!
//! Real UI-heavy plugins often need most of this stack together. The SDK keeps
//! the layers split so smaller plugins can stop earlier instead of rebuilding a
//! full menu-owned runtime by default.
//!
//! Example controller recipe:
//!
//! ```rust,ignore
//! use libskyrim::re::MenuOpenCloseEvent;
//! use libskyrim::sdk::ui;
//!
//! struct MyHudMenu;
//!
//! impl ui::menus::NamedMenu for MyHudMenu {
//!     const MENU_NAME: &'static str = "MyHudMenu";
//! }
//!
//! #[derive(Default)]
//! struct MyHudHandler;
//!
//! impl ui::controller::WidgetRequestHandler for MyHudHandler {}
//! impl ui::controller::HudRequestHandler for MyHudHandler {}
//!
//! fn on_menu_event(event: &MenuOpenCloseEvent) {
//!     let mut controller =
//!         ui::controller::HudRuntimeController::<MyHudMenu, MyHudHandler>::with_policy(
//!             MyHudHandler::default(),
//!             ui::widgets::WidgetVisibilityPolicy::hud_like(),
//!         );
//!
//!     let _cycle = controller.drive_menu_event(event);
//! }
//! ```
//!
//! Typical menu-owned HUD flow:
//!
//! 1. subscribe to menu open/close events
//! 2. drive cached menu state through [`driver`]
//! 3. queue or drain deferred movie work through [`widgets`]
//! 4. aggregate visibility/refresh requests through [`hud_runtime`]
//! 5. translate request bundles into concrete actions through [`controller`]

pub mod controller;
pub mod controls;
pub mod driver;
pub mod hud_runtime;
pub mod menus;
pub mod notifications;
pub mod scaleform;
pub mod widgets;
