//! High-level Papyrus registration macros.
//!
//! The SDK re-exports the existing crate-level Papyrus macros here so plugin
//! code can migrate toward the `sdk` namespace without losing the current
//! macro surface.

/// Implement `sdk::papyrus::PapyrusEventCollection` for a struct that stores
/// one or more persistent Papyrus event registries as fields.
///
/// Example:
///
/// ```ignore
/// struct MyPapyrusEvents {
///     actor_defeated: libskyrim::sdk::papyrus::PapyrusEventRegistry<ActorDefeatedArgs>,
///     player_death: libskyrim::sdk::papyrus::PapyrusTargetedEventRegistry<PlayerDeathArgs>,
/// }
///
/// libskyrim::sdk::papyrus::papyrus_event_collection! {
///     impl MyPapyrusEvents { actor_defeated, player_death }
/// }
/// ```
#[macro_export]
macro_rules! papyrus_event_collection {
    (impl $ty:ty { $($field:ident),+ $(,)? }) => {
        impl $crate::sdk::papyrus::PapyrusEventCollection for $ty {
            fn save_events(&self, serialization: &$crate::skse::SerializationInterface) -> bool {
                let registries: [&dyn $crate::sdk::papyrus::PapyrusPersistentEventRegistry;
                    $crate::papyrus_event_collection!(@count $($field),+)] = [
                    $(&self.$field),+
                ];
                $crate::sdk::papyrus::save_registries(serialization, &registries)
            }

            fn load_event_record(
                &mut self,
                ty: u32,
                serialization: &$crate::skse::SerializationInterface,
            ) -> $crate::sdk::papyrus::PapyrusEventLoadStatus {
                let mut registries: [&mut dyn $crate::sdk::papyrus::PapyrusPersistentEventRegistry;
                    $crate::papyrus_event_collection!(@count $($field),+)] = [
                    $(&mut self.$field),+
                ];
                $crate::sdk::papyrus::load_record(ty, serialization, &mut registries)
            }

            fn revert_events(
                &mut self,
                serialization: Option<&$crate::skse::SerializationInterface>,
            ) {
                let mut registries: [&mut dyn $crate::sdk::papyrus::PapyrusPersistentEventRegistry;
                    $crate::papyrus_event_collection!(@count $($field),+)] = [
                    $(&mut self.$field),+
                ];
                $crate::sdk::papyrus::revert_registries(serialization, &mut registries);
            }

            fn form_delete_events(&mut self, handle: $crate::re::VMHandle) -> usize {
                let mut registries: [&mut dyn $crate::sdk::papyrus::PapyrusPersistentEventRegistry;
                    $crate::papyrus_event_collection!(@count $($field),+)] = [
                    $(&mut self.$field),+
                ];
                $crate::sdk::papyrus::form_delete_registries(handle, &mut registries)
            }
        }
    };
    (@count $($field:ident),+) => {
        <[()]>::len(&[$($crate::papyrus_event_collection!(@unit $field)),+])
    };
    (@unit $field:ident) => {
        ()
    };
}

/// Generate Papyrus-facing register / unregister wrappers for SDK event
/// registries installed through `sdk::papyrus::register_event_set(...)`.
///
/// This is intentionally a thin sugar layer on top of the existing
/// `papyrus_module!` / `papyrus_*function!` macros:
///
/// - this macro generates the repetitive Rust callback items
/// - `papyrus_module!` still owns the actual VM registration surface
///
/// Example:
///
/// ```ignore
/// libskyrim::sdk::papyrus::papyrus_event_functions! {
///     pub(crate) for MyPapyrusEvents {
///         registry actor_defeated {
///             form(form) => register_for_actor_defeated, unregister_for_actor_defeated;
///             alias(alias) => register_for_actor_defeated_alias, unregister_for_actor_defeated_alias;
///             active_effect(effect) => register_for_actor_defeated_mgeff, unregister_for_actor_defeated_mgeff;
///         }
///     }
/// }
///
/// libskyrim::sdk::papyrus::papyrus_module! {
///     pub MyPapyrusModule {
///         class_named "Acheron" {
///             static fn "RegisterForActorDefeated"
///                 => register_for_actor_defeated
///                 => fn(form: libskyrim::sdk::papyrus::GamePtr<libskyrim::re::TESForm>) -> bool;
///             static fn "UnregisterForActorDefeated"
///                 => unregister_for_actor_defeated
///                 => fn(form: libskyrim::sdk::papyrus::GamePtr<libskyrim::re::TESForm>) -> bool;
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! papyrus_event_functions {
    (
        $vis:vis for $events:ty {
            $(
                registry $registry:ident {
                    $(
                        $kind:ident ($param:ident) => $register:ident, $unregister:ident;
                    )*
                }
            )*
        }
    ) => {
        $(
            $(
                $crate::papyrus_event_functions!(
                    @emit $vis, $events, $registry, $kind, $param, $register, register
                );
                $crate::papyrus_event_functions!(
                    @emit $vis, $events, $registry, $kind, $param, $unregister, unregister
                );
            )*
        )*
    };
    (@emit $vis:vis, $events:ty, $registry:ident, form, $param:ident, $fn_name:ident, register) => {
        $vis fn $fn_name(
            $param: $crate::sdk::papyrus::GamePtr<$crate::re::TESForm>,
        ) -> bool {
            $crate::sdk::papyrus::with_events_mut::<$events, _>(|events| {
                events.$registry.register_form($param)
            })
            .unwrap_or(false)
        }
    };
    (@emit $vis:vis, $events:ty, $registry:ident, form, $param:ident, $fn_name:ident, unregister) => {
        $vis fn $fn_name(
            $param: $crate::sdk::papyrus::GamePtr<$crate::re::TESForm>,
        ) -> bool {
            $crate::sdk::papyrus::with_events_mut::<$events, _>(|events| {
                events.$registry.unregister_form($param)
            })
            .unwrap_or(false)
        }
    };
    (@emit $vis:vis, $events:ty, $registry:ident, base_alias, $param:ident, $fn_name:ident, register) => {
        $vis fn $fn_name(
            $param: $crate::sdk::papyrus::GamePtr<$crate::re::BGSBaseAlias>,
        ) -> bool {
            $crate::sdk::papyrus::with_events_mut::<$events, _>(|events| {
                events.$registry.register_alias($param)
            })
            .unwrap_or(false)
        }
    };
    (@emit $vis:vis, $events:ty, $registry:ident, base_alias, $param:ident, $fn_name:ident, unregister) => {
        $vis fn $fn_name(
            $param: $crate::sdk::papyrus::GamePtr<$crate::re::BGSBaseAlias>,
        ) -> bool {
            $crate::sdk::papyrus::with_events_mut::<$events, _>(|events| {
                events.$registry.unregister_alias($param)
            })
            .unwrap_or(false)
        }
    };
    (@emit $vis:vis, $events:ty, $registry:ident, alias, $param:ident, $fn_name:ident, $action:ident) => {
        $crate::papyrus_event_functions!(
            @emit $vis, $events, $registry, ref_alias, $param, $fn_name, $action
        );
    };
    (@emit $vis:vis, $events:ty, $registry:ident, ref_alias, $param:ident, $fn_name:ident, register) => {
        $vis fn $fn_name(
            $param: $crate::sdk::papyrus::GamePtr<$crate::re::BGSRefAlias>,
        ) -> bool {
            $crate::sdk::papyrus::with_events_mut::<$events, _>(|events| {
                events.$registry.register_ref_alias($param)
            })
            .unwrap_or(false)
        }
    };
    (@emit $vis:vis, $events:ty, $registry:ident, ref_alias, $param:ident, $fn_name:ident, unregister) => {
        $vis fn $fn_name(
            $param: $crate::sdk::papyrus::GamePtr<$crate::re::BGSRefAlias>,
        ) -> bool {
            $crate::sdk::papyrus::with_events_mut::<$events, _>(|events| {
                events.$registry.unregister_ref_alias($param)
            })
            .unwrap_or(false)
        }
    };
    (@emit $vis:vis, $events:ty, $registry:ident, active_effect, $param:ident, $fn_name:ident, register) => {
        $vis fn $fn_name(
            $param: $crate::sdk::papyrus::GamePtr<$crate::re::ActiveEffect>,
        ) -> bool {
            $crate::sdk::papyrus::with_events_mut::<$events, _>(|events| {
                events.$registry.register_active_effect($param)
            })
            .unwrap_or(false)
        }
    };
    (@emit $vis:vis, $events:ty, $registry:ident, active_effect, $param:ident, $fn_name:ident, unregister) => {
        $vis fn $fn_name(
            $param: $crate::sdk::papyrus::GamePtr<$crate::re::ActiveEffect>,
        ) -> bool {
            $crate::sdk::papyrus::with_events_mut::<$events, _>(|events| {
                events.$registry.unregister_active_effect($param)
            })
            .unwrap_or(false)
        }
    };
}

/// Generate both Papyrus event wrapper functions and a `PapyrusModule`
/// registration block from one event-focused spec.
///
/// This is the SDK-facing "full boilerplate" companion to
/// `papyrus_event_functions!`: the event registry wrapper functions are
/// generated first, then a `papyrus_module!` is emitted that registers them as
/// static Papyrus functions.
///
/// Example:
///
/// ```ignore
/// libskyrim::sdk::papyrus::papyrus_event_module! {
///     pub AcheronPapyrus for AcheronEvents {
///         callbacks[pub(crate)];
///         class_named "Acheron" {
///             registry actor_defeated {
///                 form("RegisterForActorDefeated", "UnregisterForActorDefeated", form)
///                     => register_for_actor_defeated, unregister_for_actor_defeated,
///                     callable_from_tasklets = true;
///                 alias("RegisterForActorDefeated_Alias", "UnregisterForActorDefeated_Alias", alias)
///                     => register_for_actor_defeated_alias, unregister_for_actor_defeated_alias,
///                     callable_from_tasklets = true;
///             }
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! papyrus_event_module {
    (
        $(#[$meta:meta])*
        $module_vis:vis $module_name:ident for $events:ty {
            callbacks $callbacks_vis:tt;
            class_named $class_name:literal {
                $(
                    registry $registry:ident {
                        $(
                            $kind:ident ($register_name:literal, $unregister_name:literal, $param:ident)
                                => $register:ident, $unregister:ident
                                $(, callable_from_tasklets = $callable:expr)?;
                        )*
                    }
                )*
            }
        }
    ) => {
        $(
            $(
                $crate::papyrus_event_module!(
                    @emit_with_vis $callbacks_vis, $events, $registry, $kind, $param, $register, register
                );
                $crate::papyrus_event_module!(
                    @emit_with_vis $callbacks_vis, $events, $registry, $kind, $param, $unregister, unregister
                );
            )*
        )*

        $(#[$meta])*
        $module_vis struct $module_name;

        impl $crate::sdk::papyrus::PapyrusModule for $module_name {
            fn register(module: &mut $crate::skse::papyrus::ModuleRegistry<'_>) -> bool {
                {
                    let mut class = module.class_named($class_name);
                    $(
                        $(
                            $crate::papyrus_event_module!(
                                @register_static class, $kind, $register_name, $register, $param
                                $(, $callable)?
                            );
                            $crate::papyrus_event_module!(
                                @register_static class, $kind, $unregister_name, $unregister, $param
                                $(, $callable)?
                            );
                        )*
                    )*
                }
                module.is_ok()
            }
        }
    };
    (
        $(#[$meta:meta])*
        $module_vis:vis $module_name:ident for $events:ty {
            callbacks $callbacks_vis:tt;
            class $class:ty {
                $(
                    registry $registry:ident {
                        $(
                            $kind:ident ($register_name:literal, $unregister_name:literal, $param:ident)
                                => $register:ident, $unregister:ident
                                $(, callable_from_tasklets = $callable:expr)?;
                        )*
                    }
                )*
            }
        }
    ) => {
        $(
            $(
                $crate::papyrus_event_module!(
                    @emit_with_vis $callbacks_vis, $events, $registry, $kind, $param, $register, register
                );
                $crate::papyrus_event_module!(
                    @emit_with_vis $callbacks_vis, $events, $registry, $kind, $param, $unregister, unregister
                );
            )*
        )*

        $(#[$meta])*
        $module_vis struct $module_name;

        impl $crate::sdk::papyrus::PapyrusModule for $module_name {
            fn register(module: &mut $crate::skse::papyrus::ModuleRegistry<'_>) -> bool {
                {
                    let mut class = module.class::<$class>();
                    $(
                        $(
                            $crate::papyrus_event_module!(
                                @register_static class, $kind, $register_name, $register, $param
                                $(, $callable)?
                            );
                            $crate::papyrus_event_module!(
                                @register_static class, $kind, $unregister_name, $unregister, $param
                                $(, $callable)?
                            );
                        )*
                    )*
                }
                module.is_ok()
            }
        }
    };
    (@emit_with_vis [$($callbacks_vis:tt)*], $events:ty, $registry:ident, $kind:ident, $param:ident, $callback:ident, $action:ident) => {
        $crate::papyrus_event_functions!(
            @emit $($callbacks_vis)*, $events, $registry, $kind, $param, $callback, $action
        );
    };
    (@register_static $class:ident, form, $fn_name:literal, $callback:path, $param:ident $(, $callable:expr)?) => {
        let _ = $crate::papyrus_static_function!(
            $class,
            $fn_name,
            $callback => fn($param: $crate::sdk::papyrus::GamePtr<$crate::re::TESForm>) -> bool
            $(, callable_from_tasklets = $callable)?
        );
    };
    (@register_static $class:ident, base_alias, $fn_name:literal, $callback:path, $param:ident $(, $callable:expr)?) => {
        let _ = $crate::papyrus_static_function!(
            $class,
            $fn_name,
            $callback => fn($param: $crate::sdk::papyrus::GamePtr<$crate::re::BGSBaseAlias>) -> bool
            $(, callable_from_tasklets = $callable)?
        );
    };
    (@register_static $class:ident, alias, $fn_name:literal, $callback:path, $param:ident $(, $callable:expr)?) => {
        $crate::papyrus_event_module!(
            @register_static $class, ref_alias, $fn_name, $callback, $param $(, $callable)?
        );
    };
    (@register_static $class:ident, ref_alias, $fn_name:literal, $callback:path, $param:ident $(, $callable:expr)?) => {
        let _ = $crate::papyrus_static_function!(
            $class,
            $fn_name,
            $callback => fn($param: $crate::sdk::papyrus::GamePtr<$crate::re::BGSRefAlias>) -> bool
            $(, callable_from_tasklets = $callable)?
        );
    };
    (@register_static $class:ident, active_effect, $fn_name:literal, $callback:path, $param:ident $(, $callable:expr)?) => {
        let _ = $crate::papyrus_static_function!(
            $class,
            $fn_name,
            $callback => fn($param: $crate::sdk::papyrus::GamePtr<$crate::re::ActiveEffect>) -> bool
            $(, callable_from_tasklets = $callable)?
        );
    };
}

pub use crate::{
    papyrus_class, papyrus_event_collection, papyrus_event_functions, papyrus_event_module,
    papyrus_method_function, papyrus_method_latent_function, papyrus_method_long_function,
    papyrus_module, papyrus_register_function, papyrus_register_latent_function,
    papyrus_register_long_function, papyrus_static_function, papyrus_static_latent_function,
    papyrus_static_long_function,
};

#[cfg(test)]
mod tests {
    use crate::re::{ActiveEffect, BGSBaseAlias, BGSRefAlias, TESForm};
    use crate::sdk::papyrus::{
        GamePtr, PapyrusEventRegistry, PapyrusModule, PapyrusTargetedEventRegistry,
    };

    struct MacroTestEvents {
        regular: PapyrusEventRegistry<()>,
        targeted: PapyrusTargetedEventRegistry<()>,
    }

    crate::papyrus_event_collection! {
        impl MacroTestEvents { regular, targeted }
    }

    papyrus_event_functions! {
        pub(crate) for MacroTestEvents {
            registry regular {
                form(form) => register_regular_form, unregister_regular_form;
                base_alias(alias) => register_regular_base_alias, unregister_regular_base_alias;
                ref_alias(alias) => register_regular_ref_alias, unregister_regular_ref_alias;
                active_effect(effect) => register_regular_effect, unregister_regular_effect;
            }
            registry targeted {
                form(form) => register_targeted_form, unregister_targeted_form;
                alias(alias) => register_targeted_alias, unregister_targeted_alias;
                active_effect(effect) => register_targeted_effect, unregister_targeted_effect;
            }
        }
    }

    crate::papyrus_event_module! {
        pub(crate) MacroGeneratedModule for MacroTestEvents {
            callbacks[pub(crate)];
            class_named "MacroGenerated" {
                registry regular {
                    form("RegisterRegularForm2", "UnregisterRegularForm2", form)
                        => generated_register_regular_form, generated_unregister_regular_form;
                    base_alias("RegisterRegularBaseAlias2", "UnregisterRegularBaseAlias2", alias)
                        => generated_register_regular_base_alias, generated_unregister_regular_base_alias;
                    ref_alias("RegisterRegularRefAlias2", "UnregisterRegularRefAlias2", alias)
                        => generated_register_regular_ref_alias, generated_unregister_regular_ref_alias;
                    active_effect("RegisterRegularEffect2", "UnregisterRegularEffect2", effect)
                        => generated_register_regular_effect, generated_unregister_regular_effect;
                }
                registry targeted {
                    form("RegisterTargetedForm2", "UnregisterTargetedForm2", form)
                        => generated_register_targeted_form, generated_unregister_targeted_form,
                        callable_from_tasklets = true;
                    alias("RegisterTargetedAlias2", "UnregisterTargetedAlias2", alias)
                        => generated_register_targeted_alias, generated_unregister_targeted_alias;
                    active_effect("RegisterTargetedEffect2", "UnregisterTargetedEffect2", effect)
                        => generated_register_targeted_effect, generated_unregister_targeted_effect;
                }
            }
        }
    }

    #[test]
    fn generated_functions_fail_soft_without_registered_runtime() {
        assert!(!register_regular_form(GamePtr::<TESForm>::null()));
        assert!(!unregister_regular_form(GamePtr::<TESForm>::null()));
        assert!(!register_regular_base_alias(GamePtr::<BGSBaseAlias>::null()));
        assert!(!unregister_regular_base_alias(
            GamePtr::<BGSBaseAlias>::null()
        ));
        assert!(!register_regular_ref_alias(GamePtr::<BGSRefAlias>::null()));
        assert!(!unregister_regular_ref_alias(GamePtr::<BGSRefAlias>::null()));
        assert!(!register_regular_effect(GamePtr::<ActiveEffect>::null()));
        assert!(!unregister_regular_effect(GamePtr::<ActiveEffect>::null()));
        assert!(!register_targeted_form(GamePtr::<TESForm>::null()));
        assert!(!unregister_targeted_form(GamePtr::<TESForm>::null()));
        assert!(!register_targeted_alias(GamePtr::<BGSRefAlias>::null()));
        assert!(!unregister_targeted_alias(GamePtr::<BGSRefAlias>::null()));
        assert!(!register_targeted_effect(GamePtr::<ActiveEffect>::null()));
        assert!(!unregister_targeted_effect(GamePtr::<ActiveEffect>::null()));
    }

    #[test]
    fn generated_event_module_implements_papyrus_module() {
        fn assert_module<T: PapyrusModule>() {}
        assert_module::<MacroGeneratedModule>();
    }

    #[test]
    fn generated_event_module_functions_fail_soft_without_registered_runtime() {
        assert!(!generated_register_regular_form(GamePtr::<TESForm>::null()));
        assert!(!generated_unregister_regular_form(
            GamePtr::<TESForm>::null()
        ));
        assert!(!generated_register_regular_base_alias(GamePtr::<
            BGSBaseAlias,
        >::null()));
        assert!(!generated_unregister_regular_base_alias(GamePtr::<
            BGSBaseAlias,
        >::null()));
        assert!(!generated_register_regular_ref_alias(
            GamePtr::<BGSRefAlias>::null()
        ));
        assert!(!generated_unregister_regular_ref_alias(GamePtr::<
            BGSRefAlias,
        >::null()));
        assert!(!generated_register_regular_effect(
            GamePtr::<ActiveEffect>::null()
        ));
        assert!(!generated_unregister_regular_effect(
            GamePtr::<ActiveEffect>::null()
        ));
        assert!(!generated_register_targeted_form(GamePtr::<TESForm>::null()));
        assert!(!generated_unregister_targeted_form(
            GamePtr::<TESForm>::null()
        ));
        assert!(!generated_register_targeted_alias(
            GamePtr::<BGSRefAlias>::null()
        ));
        assert!(!generated_unregister_targeted_alias(
            GamePtr::<BGSRefAlias>::null()
        ));
        assert!(!generated_register_targeted_effect(
            GamePtr::<ActiveEffect>::null()
        ));
        assert!(!generated_unregister_targeted_effect(
            GamePtr::<ActiveEffect>::null()
        ));
    }
}
