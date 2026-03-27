//! Typed hook installer helpers and batch installation support.

use core::fmt;

use super::runtime::HookInstallError;

pub type HookInstallFn = fn() -> Result<(), HookInstallError>;
pub type HookInstallResult = Result<(), HookInstallError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HookBatchError {
    hook_name: &'static str,
    error: HookInstallError,
}

impl HookBatchError {
    #[inline(always)]
    pub const fn new(hook_name: &'static str, error: HookInstallError) -> Self {
        Self { hook_name, error }
    }

    #[inline(always)]
    pub const fn hook_name(self) -> &'static str {
        self.hook_name
    }

    #[inline(always)]
    pub const fn error(self) -> HookInstallError {
        self.error
    }

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

pub type HookBatchInstallResult = Result<(), HookBatchError>;

#[derive(Clone, Copy)]
pub struct HookInstaller {
    name: &'static str,
    install: HookInstallFn,
}

impl HookInstaller {
    #[inline(always)]
    pub const fn new(name: &'static str, install: HookInstallFn) -> Self {
        Self { name, install }
    }

    #[inline(always)]
    pub const fn name(self) -> &'static str {
        self.name
    }

    #[inline(always)]
    pub const fn install_fn(self) -> HookInstallFn {
        self.install
    }

    #[inline(always)]
    pub fn try_install(self) -> HookInstallResult {
        (self.install)()
    }

    #[inline(always)]
    pub fn install_or_fatal(self) {
        if let Err(error) = self.try_install() {
            error.install_or_fatal(self.name);
        }
    }
}

pub fn try_install_all(installers: &[HookInstaller]) -> HookBatchInstallResult {
    for installer in installers {
        installer
            .try_install()
            .map_err(|error| HookBatchError::new(installer.name(), error))?;
    }

    Ok(())
}

pub fn install_batch_or_fatal(installers: &[HookInstaller]) {
    if let Err(error) = try_install_all(installers) {
        error.install_or_fatal();
    }
}
