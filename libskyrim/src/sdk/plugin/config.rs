//! Plugin-facing config helpers.
//!
//! This domain is intended to gather the repetitive SKSE plugin patterns around
//! loading ini/toml/json settings, log levels, and startup configuration flow.

pub use crate::ini::Ini;

#[inline(always)]
pub fn load_ini(path: &str) -> Result<Ini, ()> {
    Ini::from_path_str(path)
}

#[inline(always)]
pub fn write_ini(ini: &Ini, path: &str) -> Result<(), core::fmt::Error> {
    ini.write_file_str(path)
}
