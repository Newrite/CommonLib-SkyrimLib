//! Ergonomic wrappers over `BSInputDeviceManager` and `InputEvent*` chains.

mod api;
mod chain;

pub use api::{prepend, subscribe};
pub use chain::{InputEventIter, InputEventIterMut, InputEvents};

#[cfg(test)]
mod tests;
