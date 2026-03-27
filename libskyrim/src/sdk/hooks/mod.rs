//! High-level hook and patch helper layer.
//!
//! This module builds the ergonomic SDK authoring surface on top of the raw
//! relocation hook layer:
//!
//! - attribute-driven hook definitions
//! - typed hook arguments (`GameRef`, `Resolved`, `ResolvedHandle`, ...)
//! - named guard presets
//! - typed installers and batch installation helpers
//!
//! The raw escape hatch remains `crate::hook! { ... }` from the relocation
//! layer for unsupported or intentionally low-level scenarios.

pub mod guards;
pub mod install;
pub mod patch;
pub mod patterns;
pub mod runtime;
pub mod trampoline;

pub use crate::sdk::core::{GameRef, GameRefMut, Resolved, ResolvedHandle};
pub use install::{
    HookBatchError, HookBatchInstallResult, HookInstallFn, HookInstallResult, HookInstaller,
    install_batch_or_fatal, try_install_all,
};
pub use libskyrim_macros::{call_hook, function_hook, vcall_hook, vtable_hook};
pub use runtime::{HookArg, HookGuardFailure, HookInstallError, HookNullAbi, Original};

#[macro_export]
macro_rules! __libskyrim_sdk_hooks_install_all {
    ($($hook:path),+ $(,)?) => {{
        $crate::sdk::hooks::try_install_all(&[
            $(
                {
                    use $hook as __sdk_hook_mod;
                    __sdk_hook_mod::INSTALLER
                }
            ),+
        ])
    }};
}

pub use crate::__libskyrim_sdk_hooks_install_all as install_all;

#[macro_export]
macro_rules! __libskyrim_sdk_hooks_install_all_or_fatal {
    ($($hook:path),+ $(,)?) => {{
        $crate::sdk::hooks::install_batch_or_fatal(&[
            $(
                {
                    use $hook as __sdk_hook_mod;
                    __sdk_hook_mod::INSTALLER
                }
            ),+
        ])
    }};
}

pub use crate::__libskyrim_sdk_hooks_install_all_or_fatal as install_all_or_fatal;

#[cfg(test)]
mod tests {
    use crate::re::Actor;
    use crate::sdk::hooks::{self, GameRef, Resolved};

    #[crate::sdk::hooks::function_hook(target = 0usize, guard = crate::sdk::hooks::guards::default())]
    fn sdk_function_example(
        original: crate::sdk::hooks::Original<fn(GameRef<'_, Actor>, u32) -> u32>,
        actor: GameRef<'_, Actor>,
        value: u32,
    ) -> u32 {
        original.call(actor, value)
    }

    #[crate::sdk::hooks::call_hook(
        target = 0usize,
        offset = 0usize,
        size = 5,
        guard = crate::sdk::hooks::guards::skip()
    )]
    fn sdk_call_example(
        original: crate::sdk::hooks::Original<fn(GameRef<'_, Actor>)>,
        actor: GameRef<'_, Actor>,
    ) {
        original.call(actor);
    }

    #[crate::sdk::hooks::vtable_hook(vtable = 0usize, slot = 0usize, invalid = original)]
    fn sdk_vtable_example(
        original: crate::sdk::hooks::Original<fn(&mut Actor, u32) -> bool>,
        actor: &mut Actor,
        value: u32,
    ) -> bool {
        original.call(actor, value)
    }

    #[crate::sdk::hooks::vcall_hook(
        target = 0usize,
        offset = 0usize,
        size = 6,
        receiver = this,
        index = 0usize,
        invalid = return_(0u32)
    )]
    fn sdk_vcall_example(
        original: crate::sdk::hooks::Original<fn(*mut Actor, Resolved<Actor>, u32) -> u32>,
        this: *mut Actor,
        actor: Resolved<Actor>,
        value: u32,
    ) -> u32 {
        original.call(this, actor, value)
    }

    #[test]
    fn attribute_hook_modules_compile() {
        let _ = sdk_function_example_hook::is_installed as fn() -> bool;
        let _ = sdk_function_example_hook::try_install
            as fn() -> Result<(), crate::sdk::hooks::HookInstallError>;
        let _ = sdk_function_example_hook::install_or_fatal as fn();

        let _ = sdk_call_example_hook::is_installed as fn() -> bool;
        let _ = sdk_call_example_hook::try_install
            as fn() -> Result<(), crate::sdk::hooks::HookInstallError>;

        let _ = sdk_vtable_example_hook::is_installed as fn() -> bool;
        let _ = sdk_vtable_example_hook::try_install
            as fn() -> Result<(), crate::sdk::hooks::HookInstallError>;

        let _ = sdk_vcall_example_hook::is_installed as fn() -> bool;
        let _ = sdk_vcall_example_hook::try_install
            as fn() -> Result<(), crate::sdk::hooks::HookInstallError>;
        let _ = sdk_vcall_example_hook::INSTALLER;

        let _ = hooks::install_all![sdk_function_example_hook, sdk_call_example_hook]
            as Result<(), hooks::HookBatchError>;
    }
}
