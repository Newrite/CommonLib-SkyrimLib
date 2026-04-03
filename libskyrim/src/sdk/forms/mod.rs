//! Form lookup and form-centric utilities for plugin authors.

mod shared;

pub mod keywords;
pub mod lists;
pub mod lookup;
pub mod persistent;
pub mod settings;

pub use persistent::{PersistentForm, PersistentFormPtr};
