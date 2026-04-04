use crate::sdk::ui::controls::UiControlSnapshot;

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

impl core::fmt::Display for RuntimePhaseBlocker {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[inline(always)]
const fn ui_phase_blocker(ui: UiControlSnapshot) -> Option<RuntimePhaseBlocker> {
    if ui.loading_open {
        Some(RuntimePhaseBlocker::LoadingScreen)
    } else if ui.fader_open || ui.fader_active {
        Some(RuntimePhaseBlocker::Fader)
    } else if ui.has_major_menu_open() {
        Some(RuntimePhaseBlocker::MajorMenuOpen)
    } else if ui.menu_like_context {
        Some(RuntimePhaseBlocker::MenuLikeContext)
    } else {
        None
    }
}

#[inline(always)]
const fn gameplay_input_blocker(ui: UiControlSnapshot) -> Option<RuntimePhaseBlocker> {
    if ui.game_paused {
        Some(RuntimePhaseBlocker::GamePaused)
    } else if ui.text_input_active {
        Some(RuntimePhaseBlocker::TextInput)
    } else if ui.keyboard_mouse_ignored {
        Some(RuntimePhaseBlocker::KeyboardMouseIgnored)
    } else if ui.activate_disabled_events_ignored {
        Some(RuntimePhaseBlocker::ActivateDisabledEventsIgnored)
    } else if ui.player_input_blocked {
        Some(RuntimePhaseBlocker::PlayerInputBlocked)
    } else if !ui.gameplay_controls_enabled {
        Some(RuntimePhaseBlocker::GameplayControlsDisabled)
    } else {
        None
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
        if self.ui.game_paused {
            Some(RuntimePhaseBlocker::GamePaused)
        } else if let Some(blocker) = ui_phase_blocker(self.ui) {
            Some(blocker)
        } else {
            gameplay_input_blocker(self.ui)
        }
    }

    #[inline(always)]
    pub const fn hud_widget_blocker(self) -> Option<RuntimePhaseBlocker> {
        if !self.ui.menus_visible {
            Some(RuntimePhaseBlocker::MenusHidden)
        } else {
            ui_phase_blocker(self.ui)
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
        ui: crate::sdk::ui::controls::control_snapshot(),
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
