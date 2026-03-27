//! Named guard presets for high-level SDK hook attributes.
//!
//! These presets are intentionally small and composable. The proc-macro layer
//! recognizes the functions in this module syntactically when they are used as
//! `guard = hooks::guards::<preset>()` inside hook attributes.

/// The base action to take when a strict hook argument fails adaptation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardAction {
    Original,
    Skip,
    Default,
}

/// A named preset describing the base invalidity policy and any per-kind
/// overrides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuardPreset {
    invalid: GuardAction,
    null: Option<GuardAction>,
    unresolved: Option<GuardAction>,
    convert_fail: Option<GuardAction>,
}

impl GuardPreset {
    #[inline(always)]
    pub const fn new(invalid: GuardAction) -> Self {
        Self {
            invalid,
            null: None,
            unresolved: None,
            convert_fail: None,
        }
    }

    #[inline(always)]
    pub const fn invalid(self) -> GuardAction {
        self.invalid
    }

    #[inline(always)]
    pub const fn null(self) -> Option<GuardAction> {
        self.null
    }

    #[inline(always)]
    pub const fn unresolved(self) -> Option<GuardAction> {
        self.unresolved
    }

    #[inline(always)]
    pub const fn convert_fail(self) -> Option<GuardAction> {
        self.convert_fail
    }

    #[inline(always)]
    pub const fn with_null(mut self, action: GuardAction) -> Self {
        self.null = Some(action);
        self
    }

    #[inline(always)]
    pub const fn with_unresolved(mut self, action: GuardAction) -> Self {
        self.unresolved = Some(action);
        self
    }

    #[inline(always)]
    pub const fn with_convert_fail(mut self, action: GuardAction) -> Self {
        self.convert_fail = Some(action);
        self
    }
}

/// Always fall back to the original target when a strict hook argument fails.
#[inline(always)]
pub const fn original() -> GuardPreset {
    GuardPreset::new(GuardAction::Original)
}

/// Always return `Default::default()` when a strict hook argument fails.
#[inline(always)]
pub const fn default() -> GuardPreset {
    GuardPreset::new(GuardAction::Default)
}

/// Always skip the original call when a strict hook argument fails.
///
/// This preset is only valid for hooks returning `()`.
#[inline(always)]
pub const fn skip() -> GuardPreset {
    GuardPreset::new(GuardAction::Skip)
}
