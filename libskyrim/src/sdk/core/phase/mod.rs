//! Shared lifecycle and phase concepts for SDK domains.
//!
//! These enums intentionally stay independent from the raw SKSE messaging
//! layer. `sdk::events::skse::messages` maps message kinds into these types,
//! while other SDK domains may reuse the same lifecycle vocabulary later.
//!
//! The runtime side of this module is the common answer to questions like:
//!
//! - "is it safe to touch gameplay state right now?"
//! - "should this work be deferred until loading/fade finishes?"
//! - "may my HUD/controller code run this frame?"
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::core::phase::{
//!     RuntimePhaseBlocker, gameplay_phase_blocker, hud_widget_phase_blocker,
//! };
//!
//! fn should_update_custom_hud() -> bool {
//!     match hud_widget_phase_blocker() {
//!         None => true,
//!         Some(RuntimePhaseBlocker::LoadingMenu)
//!         | Some(RuntimePhaseBlocker::LoadingScreen)
//!         | Some(RuntimePhaseBlocker::FaderMenu) => false,
//!         Some(_) => gameplay_phase_blocker().is_none(),
//!     }
//! }
//! ```

mod lifecycle;
mod runtime;

#[cfg(test)]
mod tests;

pub use lifecycle::{GameLifecyclePhase, LifecyclePhase, PluginLifecyclePhase};
pub use runtime::{
    RuntimePhaseBlocker, RuntimePhaseSnapshot, allows_hud_widgets, gameplay_phase_blocker,
    hud_widget_phase_blocker, is_loading_or_fading, is_safe_for_gameplay, is_unsafe_for_gameplay,
    should_defer_gameplay_work, snapshot_runtime_phase,
};
