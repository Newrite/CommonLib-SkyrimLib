//! Shared lifecycle and phase concepts for SDK domains.
//!
//! These enums intentionally stay independent from the raw SKSE messaging
//! layer. `sdk::events::skse::messages` maps message kinds into these types,
//! while other SDK domains may reuse the same lifecycle vocabulary later.

use core::fmt;

use crate::sdk::ui::controls::UiControlSnapshot;

/// Plugin-level SKSE lifecycle phases that happen during plugin bootstrap.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PluginLifecyclePhase {
    PostLoad,
    PostPostLoad,
    InputLoaded,
    DataLoaded,
}

impl PluginLifecyclePhase {
    pub const ALL: [Self; 4] = [
        Self::PostLoad,
        Self::PostPostLoad,
        Self::InputLoaded,
        Self::DataLoaded,
    ];

    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostLoad => "post_load",
            Self::PostPostLoad => "post_post_load",
            Self::InputLoaded => "input_loaded",
            Self::DataLoaded => "data_loaded",
        }
    }
}

impl fmt::Display for PluginLifecyclePhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Save/load-oriented lifecycle phases driven by the active game state.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameLifecyclePhase {
    PreLoadGame,
    PostLoadGame,
    SaveGame,
    DeleteGame,
    NewGame,
}

impl GameLifecyclePhase {
    pub const ALL: [Self; 5] = [
        Self::PreLoadGame,
        Self::PostLoadGame,
        Self::SaveGame,
        Self::DeleteGame,
        Self::NewGame,
    ];

    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreLoadGame => "pre_load_game",
            Self::PostLoadGame => "post_load_game",
            Self::SaveGame => "save_game",
            Self::DeleteGame => "delete_game",
            Self::NewGame => "new_game",
        }
    }
}

impl fmt::Display for GameLifecyclePhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Unified lifecycle category for APIs that accept either plugin or
/// game-state phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LifecyclePhase {
    Plugin(PluginLifecyclePhase),
    Game(GameLifecyclePhase),
}

impl LifecyclePhase {
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Plugin(phase) => phase.as_str(),
            Self::Game(phase) => phase.as_str(),
        }
    }

    #[inline(always)]
    pub const fn plugin(self) -> Option<PluginLifecyclePhase> {
        match self {
            Self::Plugin(phase) => Some(phase),
            Self::Game(_) => None,
        }
    }

    #[inline(always)]
    pub const fn game(self) -> Option<GameLifecyclePhase> {
        match self {
            Self::Plugin(_) => None,
            Self::Game(phase) => Some(phase),
        }
    }
}

impl From<PluginLifecyclePhase> for LifecyclePhase {
    #[inline(always)]
    fn from(value: PluginLifecyclePhase) -> Self {
        Self::Plugin(value)
    }
}

impl From<GameLifecyclePhase> for LifecyclePhase {
    #[inline(always)]
    fn from(value: GameLifecyclePhase) -> Self {
        Self::Game(value)
    }
}

impl fmt::Display for LifecyclePhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Small reusable blockers for common UI/gameplay-sensitive runtime phases.
///
/// These are intentionally compact rather than perfectly exhaustive: the goal
/// is to give plugin code a named answer to "why should I bail out or defer
/// this gameplay work right now?" without inventing a whole state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuntimePhaseBlocker {
    LoadingScreen,
    Fader,
    GamePaused,
    MajorMenuOpen,
    MenuLikeContext,
    TextInput,
    KeyboardMouseIgnored,
    ActivateDisabledEventsIgnored,
    PlayerInputBlocked,
    GameplayControlsDisabled,
    MenusHidden,
}

impl RuntimePhaseBlocker {
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LoadingScreen => "loading_screen",
            Self::Fader => "fader",
            Self::GamePaused => "game_paused",
            Self::MajorMenuOpen => "major_menu_open",
            Self::MenuLikeContext => "menu_like_context",
            Self::TextInput => "text_input",
            Self::KeyboardMouseIgnored => "keyboard_mouse_ignored",
            Self::ActivateDisabledEventsIgnored => "activate_disabled_events_ignored",
            Self::PlayerInputBlocked => "player_input_blocked",
            Self::GameplayControlsDisabled => "gameplay_controls_disabled",
            Self::MenusHidden => "menus_hidden",
        }
    }
}

impl fmt::Display for RuntimePhaseBlocker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Cross-cutting runtime/UI phase snapshot for tiny gameplay and HUD guards.
///
/// This intentionally reuses the existing `sdk::ui::*` seams instead of
/// introducing a new raw state source. It is a compact answer to the recurring
/// "is this an unsafe phase for gameplay work?" pattern seen in plugins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimePhaseSnapshot {
    pub ui: UiControlSnapshot,
    pub pause_menu_disabled: bool,
    pub saving_allowed: bool,
    pub cursor_hidden_when_topmost: bool,
    pub custom_rendering_active: bool,
}

impl RuntimePhaseSnapshot {
    #[inline(always)]
    pub const fn is_loading_or_fading(self) -> bool {
        self.ui.has_loading_transition()
    }

    #[inline(always)]
    pub const fn has_major_menu_open(self) -> bool {
        self.ui.has_major_menu_open()
    }

    #[inline(always)]
    pub const fn gameplay_blocker(self) -> Option<RuntimePhaseBlocker> {
        if self.ui.loading_open {
            Some(RuntimePhaseBlocker::LoadingScreen)
        } else if self.ui.fader_open || self.ui.fader_active {
            Some(RuntimePhaseBlocker::Fader)
        } else if self.ui.game_paused {
            Some(RuntimePhaseBlocker::GamePaused)
        } else if self.ui.has_major_menu_open() {
            Some(RuntimePhaseBlocker::MajorMenuOpen)
        } else if self.ui.menu_like_context {
            Some(RuntimePhaseBlocker::MenuLikeContext)
        } else if self.ui.text_input_active {
            Some(RuntimePhaseBlocker::TextInput)
        } else if self.ui.keyboard_mouse_ignored {
            Some(RuntimePhaseBlocker::KeyboardMouseIgnored)
        } else if self.ui.activate_disabled_events_ignored {
            Some(RuntimePhaseBlocker::ActivateDisabledEventsIgnored)
        } else if self.ui.player_input_blocked {
            Some(RuntimePhaseBlocker::PlayerInputBlocked)
        } else if !self.ui.gameplay_controls_enabled {
            Some(RuntimePhaseBlocker::GameplayControlsDisabled)
        } else {
            None
        }
    }

    #[inline(always)]
    pub const fn hud_widget_blocker(self) -> Option<RuntimePhaseBlocker> {
        if !self.ui.menus_visible {
            Some(RuntimePhaseBlocker::MenusHidden)
        } else if self.ui.loading_open {
            Some(RuntimePhaseBlocker::LoadingScreen)
        } else if self.ui.fader_open || self.ui.fader_active {
            Some(RuntimePhaseBlocker::Fader)
        } else if self.ui.has_major_menu_open() {
            Some(RuntimePhaseBlocker::MajorMenuOpen)
        } else if self.ui.menu_like_context {
            Some(RuntimePhaseBlocker::MenuLikeContext)
        } else {
            None
        }
    }

    #[inline(always)]
    pub const fn is_safe_for_gameplay(self) -> bool {
        self.gameplay_blocker().is_none()
    }

    #[inline(always)]
    pub const fn is_unsafe_for_gameplay(self) -> bool {
        self.gameplay_blocker().is_some()
    }

    #[inline(always)]
    pub const fn should_defer_gameplay_work(self) -> bool {
        self.is_unsafe_for_gameplay()
    }

    #[inline(always)]
    pub const fn allows_hud_widgets(self) -> bool {
        self.hud_widget_blocker().is_none()
    }
}

#[inline(always)]
pub fn snapshot_runtime_phase() -> RuntimePhaseSnapshot {
    RuntimePhaseSnapshot {
        ui: crate::sdk::ui::controls::snapshot(),
        pause_menu_disabled: crate::sdk::ui::menus::is_pause_menu_disabled(),
        saving_allowed: crate::sdk::ui::menus::is_saving_allowed(),
        cursor_hidden_when_topmost: crate::sdk::ui::menus::is_cursor_hidden_when_topmost(),
        custom_rendering_active: crate::sdk::ui::menus::is_custom_rendering_active(),
    }
}

#[inline(always)]
pub fn is_loading_or_fading() -> bool {
    snapshot_runtime_phase().is_loading_or_fading()
}

#[inline(always)]
pub fn gameplay_phase_blocker() -> Option<RuntimePhaseBlocker> {
    snapshot_runtime_phase().gameplay_blocker()
}

#[inline(always)]
pub fn hud_widget_phase_blocker() -> Option<RuntimePhaseBlocker> {
    snapshot_runtime_phase().hud_widget_blocker()
}

#[inline(always)]
pub fn is_safe_for_gameplay() -> bool {
    snapshot_runtime_phase().is_safe_for_gameplay()
}

#[inline(always)]
pub fn is_unsafe_for_gameplay() -> bool {
    snapshot_runtime_phase().is_unsafe_for_gameplay()
}

#[inline(always)]
pub fn should_defer_gameplay_work() -> bool {
    snapshot_runtime_phase().should_defer_gameplay_work()
}

#[inline(always)]
pub fn allows_hud_widgets() -> bool {
    snapshot_runtime_phase().allows_hud_widgets()
}

#[cfg(test)]
mod tests {
    use super::{RuntimePhaseBlocker, RuntimePhaseSnapshot};
    use crate::re::INPUT_CONTEXT_ID;
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

    fn gameplay_phase_snapshot() -> RuntimePhaseSnapshot {
        RuntimePhaseSnapshot {
            ui: gameplay_ui_snapshot(),
            pause_menu_disabled: false,
            saving_allowed: true,
            cursor_hidden_when_topmost: false,
            custom_rendering_active: false,
        }
    }

    #[test]
    fn gameplay_snapshot_reports_safe_phase_and_visible_hud() {
        let snapshot = gameplay_phase_snapshot();

        assert!(!snapshot.is_loading_or_fading());
        assert_eq!(snapshot.gameplay_blocker(), None);
        assert!(snapshot.is_safe_for_gameplay());
        assert!(!snapshot.is_unsafe_for_gameplay());
        assert!(!snapshot.should_defer_gameplay_work());
        assert_eq!(snapshot.hud_widget_blocker(), None);
        assert!(snapshot.allows_hud_widgets());
    }

    #[test]
    fn loading_and_fader_take_priority_as_phase_blockers() {
        let mut loading = gameplay_phase_snapshot();
        loading.ui.loading_open = true;
        assert_eq!(
            loading.gameplay_blocker(),
            Some(RuntimePhaseBlocker::LoadingScreen)
        );
        assert_eq!(
            loading.hud_widget_blocker(),
            Some(RuntimePhaseBlocker::LoadingScreen)
        );

        let mut fader = gameplay_phase_snapshot();
        fader.ui.fader_active = true;
        assert_eq!(fader.gameplay_blocker(), Some(RuntimePhaseBlocker::Fader));
        assert_eq!(fader.hud_widget_blocker(), Some(RuntimePhaseBlocker::Fader));
    }

    #[test]
    fn paused_and_menu_states_block_gameplay() {
        let mut paused = gameplay_phase_snapshot();
        paused.ui.game_paused = true;
        assert_eq!(
            paused.gameplay_blocker(),
            Some(RuntimePhaseBlocker::GamePaused)
        );

        let mut menu = gameplay_phase_snapshot();
        menu.ui.inventory_open = true;
        menu.ui.item_menu_open = true;
        assert_eq!(
            menu.gameplay_blocker(),
            Some(RuntimePhaseBlocker::MajorMenuOpen)
        );
        assert_eq!(
            menu.hud_widget_blocker(),
            Some(RuntimePhaseBlocker::MajorMenuOpen)
        );
    }

    #[test]
    fn input_level_blockers_fire_after_ui_phase_checks() {
        let mut snapshot = gameplay_phase_snapshot();
        snapshot.ui.text_input_active = true;
        assert_eq!(
            snapshot.gameplay_blocker(),
            Some(RuntimePhaseBlocker::TextInput)
        );

        snapshot.ui.text_input_active = false;
        snapshot.ui.player_input_blocked = true;
        assert_eq!(
            snapshot.gameplay_blocker(),
            Some(RuntimePhaseBlocker::PlayerInputBlocked)
        );

        snapshot.ui.player_input_blocked = false;
        snapshot.ui.gameplay_controls_enabled = false;
        assert_eq!(
            snapshot.gameplay_blocker(),
            Some(RuntimePhaseBlocker::GameplayControlsDisabled)
        );
    }

    #[test]
    fn hidden_menus_only_block_hud_widgets() {
        let mut snapshot = gameplay_phase_snapshot();
        snapshot.ui.menus_visible = false;

        assert_eq!(snapshot.gameplay_blocker(), None);
        assert_eq!(
            snapshot.hud_widget_blocker(),
            Some(RuntimePhaseBlocker::MenusHidden)
        );
        assert!(!snapshot.allows_hud_widgets());
    }
}
