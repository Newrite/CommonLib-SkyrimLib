//! High-level Papyrus registration macros.
//!
//! The SDK re-exports the existing crate-level Papyrus macros here so plugin
//! code can migrate toward the `sdk` namespace without losing the current
//! macro surface.

pub use crate::{
    papyrus_class, papyrus_method_function, papyrus_method_latent_function,
    papyrus_method_long_function, papyrus_module, papyrus_register_function,
    papyrus_register_latent_function, papyrus_register_long_function, papyrus_static_function,
    papyrus_static_latent_function, papyrus_static_long_function,
};
