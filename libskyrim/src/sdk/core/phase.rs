//! Shared lifecycle and phase concepts for SDK domains.
//!
//! These enums intentionally stay independent from the raw SKSE messaging
//! layer. `sdk::events::skse::messages` maps message kinds into these types,
//! while other SDK domains may reuse the same lifecycle vocabulary later.

use core::fmt;

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
