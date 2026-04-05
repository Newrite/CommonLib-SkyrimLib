//! Domain-oriented Papyrus registry helpers.

use crate::re::IVirtualMachine;
use crate::skse::crash;

pub use crate::skse::papyrus::{
    ClassRegistry, FunctionOptions, ModuleRegistry, NamedClassRegistry, PapyrusClass,
    PapyrusModule, Registry, register, register_module,
};

/// Lightweight Papyrus binding recipe for the common
/// `Bind(VM*) -> RegisterFunction(...)` plugin pattern.
///
/// This intentionally sits one layer below `PapyrusModule`:
/// use `PapyrusScript` when a plugin only needs to bind a single script/class
/// name with a direct `Registry<'_>` and does not benefit from a richer
/// `ModuleRegistry` / `papyrus_module!` layout.
///
/// Typical use cases:
///
/// - one utility script with a few static functions
/// - direct migration from a small C++ `Bind(VM*)` helper
/// - plugins that do not need persistent Papyrus event registries
///
/// Reach for [`PapyrusModule`] instead when registration naturally spans
/// multiple classes or benefits from a more declarative module layout.
pub trait PapyrusScript {
    /// Script or class name exposed to Papyrus.
    const NAME: &'static str;

    /// Register one or more Papyrus functions for this script.
    ///
    /// Returning `false` marks the registration as failed. In practice most
    /// implementations call one or more `registry.register_*...` helpers and
    /// then return `registry.is_ok()`.
    fn register(registry: &mut Registry<'_>) -> bool;
}

/// Register a lightweight `PapyrusScript` binding recipe with SKSE.
///
/// This is the direct-registration companion to `register_module::<T>()` for
/// plugin code that mirrors the common C++ `Bind(VM*)` style.
///
/// Example:
///
/// ```rust,ignore
/// use libskyrim::sdk::papyrus;
///
/// fn get_version(_base: *mut libskyrim::re::StaticFunctionTag) -> i32 {
///     1
/// }
///
/// papyrus::papyrus_script! {
///     pub ExampleBindings("ExampleScript") {
///         fn "GetVersion"
///             => get_version
///             => fn(base: *mut libskyrim::re::StaticFunctionTag) -> i32,
///             callable_from_tasklets = true;
///     }
/// }
///
/// fn install_papyrus() -> bool {
///     papyrus::register_script::<ExampleBindings>()
/// }
/// ```
#[inline(always)]
pub fn register_script<S>() -> bool
where
    S: PapyrusScript,
{
    unsafe extern "system" fn register_impl<S>(vm: *mut IVirtualMachine) -> bool
    where
        S: PapyrusScript,
    {
        crash::guard("SKSE Papyrus register callback", || {
            let Some(vm) = (unsafe { vm.as_ref() }) else {
                return false;
            };

            let mut registry = Registry::new(vm, S::NAME);
            let ok = S::register(&mut registry);
            registry.note(ok);
            registry.finish()
        })
    }

    register(register_impl::<S>)
}

#[cfg(test)]
mod tests {
    use super::PapyrusScript;

    struct ExamplePapyrusScript;

    impl PapyrusScript for ExamplePapyrusScript {
        const NAME: &'static str = "ExamplePapyrus";

        fn register(_registry: &mut super::Registry<'_>) -> bool {
            true
        }
    }

    #[test]
    fn papyrus_script_trait_exposes_name() {
        fn assert_script<T: PapyrusScript>() {}

        assert_script::<ExamplePapyrusScript>();
        assert_eq!(ExamplePapyrusScript::NAME, "ExamplePapyrus");
    }
}
