//! Typed hook installer helpers and batch installation support.

use core::fmt;

use super::runtime::HookInstallError;

/// Function ABI used by generated hook installers.
///
/// Each installer is expected to be idempotent only at the plugin-policy
/// level; repeated installation usually surfaces as
/// [`HookInstallError::AlreadyInstalled`].
pub type HookInstallFn = fn() -> Result<(), HookInstallError>;

/// Standard result type for hook installation.
pub type HookInstallResult = Result<(), HookInstallError>;

/// Error reported when one hook installer inside a batch fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HookBatchError {
    hook_name: &'static str,
    error: HookInstallError,
}

impl HookBatchError {
    /// Build a batch error from one hook installer name and underlying cause.
    #[inline(always)]
    pub const fn new(hook_name: &'static str, error: HookInstallError) -> Self {
        Self { hook_name, error }
    }

    /// Name of the hook installer that failed.
    #[inline(always)]
    pub const fn hook_name(self) -> &'static str {
        self.hook_name
    }

    /// Underlying hook-install error.
    #[inline(always)]
    pub const fn error(self) -> HookInstallError {
        self.error
    }

    /// Abort plugin startup with a fatal runtime error.
    #[inline(always)]
    pub fn install_or_fatal(self) -> ! {
        match self.error {
            HookInstallError::AlreadyInstalled(_) => self.error.install_or_fatal(self.hook_name),
            _ => self.error.install_or_fatal(self.hook_name),
        }
    }
}

impl fmt::Display for HookBatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to install hook `{}`: {}",
            self.hook_name, self.error
        )
    }
}

impl core::error::Error for HookBatchError {}

/// Result type used by ordered hook-batch installation.
pub type HookBatchInstallResult = Result<(), HookBatchError>;

/// Typed description of one installable hook recipe.
///
/// These values are what `install_all`-style bootstrap flows operate on when a
/// plugin wants a stable ordered list of named hook installers.
#[derive(Clone, Copy)]
pub struct HookInstaller {
    name: &'static str,
    install: HookInstallFn,
}

impl HookInstaller {
    /// Construct one hook installer from a stable name and install function.
    #[inline(always)]
    pub const fn new(name: &'static str, install: HookInstallFn) -> Self {
        Self { name, install }
    }

    /// Stable human-readable hook installer name.
    #[inline(always)]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Raw install function pointer.
    #[inline(always)]
    pub const fn install_fn(self) -> HookInstallFn {
        self.install
    }

    /// Attempts installation and returns the raw hook-install result.
    ///
    /// Prefer this when bootstrap wants to surface errors normally instead of
    /// terminating plugin startup immediately.
    #[inline(always)]
    pub fn try_install(self) -> HookInstallResult {
        (self.install)()
    }

    /// Installs the hook or fails fatally.
    ///
    /// Use this only when there is no sensible degraded mode without the hook.
    #[inline(always)]
    pub fn install_or_fatal(self) {
        if let Err(error) = self.try_install() {
            error.install_or_fatal(self.name);
        }
    }
}

/// Tries to install every hook installer in order.
///
/// This is the normal batch entrypoint for plugin bootstrap code that wants a
/// deterministic ordered hook list and structured error handling.
pub fn try_install_all(installers: &[HookInstaller]) -> HookBatchInstallResult {
    for installer in installers {
        installer
            .try_install()
            .map_err(|error| HookBatchError::new(installer.name(), error))?;
    }

    Ok(())
}

/// Installs all hooks or terminates plugin startup on failure.
pub fn install_batch_or_fatal(installers: &[HookInstaller]) {
    if let Err(error) = try_install_all(installers) {
        error.install_or_fatal();
    }
}
