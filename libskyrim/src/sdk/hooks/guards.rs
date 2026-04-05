//! Named guard presets for high-level SDK hook attributes.
//!
//! These presets are intentionally small and composable. The proc-macro layer
//! recognizes the functions in this module syntactically when they are used as
//! `guard = hooks::guards::<preset>()` inside hook attributes.
//!
//! They exist for the common "strict hook arguments may fail adaptation"
//! problem. Instead of rewriting the fallback policy for every hook, authoring
//! code can select a preset that says whether invalid/null/unresolved inputs
//! should:
//!
//! - call the original target
//! - skip the original target
//! - return `Default::default()`
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::hooks;
//!
//! #[libskyrim::hook(
//!     offset = 0x123456,
//!     guard = hooks::guards::original(),
//! )]
//! fn example_hook(_target: libskyrim::sdk::core::GameRef<libskyrim::re::TESObjectREFR>) {}
//! ```

/// The base action to take when a strict hook argument fails adaptation.
///
/// These values are consumed by [`GuardPreset`] and then interpreted by the
/// proc-macro hook layer during generated fallback handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardAction {
    /// Fall back to the original target as if the hook had declined to act.
    Original,
    /// Skip the original target entirely.
    ///
    /// This is only valid for hooks that can safely suppress the original
    /// behavior, typically `()`-returning fire-and-forget hooks.
    Skip,
    /// Return `Default::default()` from the generated hook wrapper.
    ///
    /// This is useful when the hook can cheaply decline to act and the return
    /// type has a meaningful default value.
    Default,
}

/// A named preset describing the base invalidity policy and any per-kind
/// overrides.
///
/// Most hook sites use one of the ready-made presets from [`original`],
/// [`default`], or [`skip`]. Construct a custom preset when null,
/// unresolved-handle, and conversion-failure cases need different behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuardPreset {
    invalid: GuardAction,
    null: Option<GuardAction>,
    unresolved: Option<GuardAction>,
    convert_fail: Option<GuardAction>,
}

impl GuardPreset {
    /// Construct a preset with one default invalid-argument action.
    #[inline(always)]
    pub const fn new(invalid: GuardAction) -> Self {
        Self {
            invalid,
            null: None,
            unresolved: None,
            convert_fail: None,
        }
    }

    /// Default action used when no more specific override applies.
    ///
    /// In practice this is the policy for any adaptation failure kind that was
    /// not explicitly overridden through `with_*`.
    #[inline(always)]
    pub const fn invalid(self) -> GuardAction {
        self.invalid
    }

    /// Action used for null-argument failures, when overridden.
    #[inline(always)]
    pub const fn null(self) -> Option<GuardAction> {
        self.null
    }

    /// Action used for unresolved-handle failures, when overridden.
    #[inline(always)]
    pub const fn unresolved(self) -> Option<GuardAction> {
        self.unresolved
    }

    /// Action used for conversion failures, when overridden.
    #[inline(always)]
    pub const fn convert_fail(self) -> Option<GuardAction> {
        self.convert_fail
    }

    /// Override the null-argument action.
    ///
    /// Use this when null input should behave differently from other guard
    /// failures such as unresolved handles or failed conversions.
    #[inline(always)]
    pub const fn with_null(mut self, action: GuardAction) -> Self {
        self.null = Some(action);
        self
    }

    /// Override the unresolved-handle action.
    ///
    /// This is most useful when handle-backed SDK hook arguments should fall
    /// back to the original target without forcing the same behavior for every
    /// other invalidity case.
    #[inline(always)]
    pub const fn with_unresolved(mut self, action: GuardAction) -> Self {
        self.unresolved = Some(action);
        self
    }

    /// Override the conversion-failure action.
    ///
    /// Use this when a strict typed conversion failure should behave
    /// differently from null or unresolved-handle cases.
    #[inline(always)]
    pub const fn with_convert_fail(mut self, action: GuardAction) -> Self {
        self.convert_fail = Some(action);
        self
    }
}

/// Always fall back to the original target when a strict hook argument fails.
///
/// This is the safest general-purpose preset and usually the right default for
/// observational hooks.
#[inline(always)]
pub const fn original() -> GuardPreset {
    GuardPreset::new(GuardAction::Original)
}

/// Always return `Default::default()` when a strict hook argument fails.
///
/// Use this when the hook can cheaply decline to act and the return type has a
/// meaningful default value.
///
/// This is a good fit for read/query hooks where "no value" and the type's
/// default are close enough to be treated the same by plugin logic.
#[inline(always)]
pub const fn default() -> GuardPreset {
    GuardPreset::new(GuardAction::Default)
}

/// Always skip the original call when a strict hook argument fails.
///
/// This preset is only valid for hooks returning `()`.
/// It is most useful for "best effort" fire-and-forget hooks where invalid
/// arguments should suppress further work instead of calling back into the
/// original path.
#[inline(always)]
pub const fn skip() -> GuardPreset {
    GuardPreset::new(GuardAction::Skip)
}
