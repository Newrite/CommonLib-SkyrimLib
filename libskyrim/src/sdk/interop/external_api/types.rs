use core::ffi::c_void;
use core::fmt;

/// ABI used by `RequestPluginAPI`-style versioned exports.
pub type RequestPluginApiFn<V> = unsafe extern "system" fn(V) -> *mut c_void;

/// ABI used by getter-style custom exported service symbols.
pub type ExportedSymbolFn = unsafe extern "system" fn() -> *mut c_void;

/// Error returned when a target DLL is not currently loaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleError {
    NotLoaded,
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotLoaded => write!(f, "target module is not loaded"),
        }
    }
}

impl core::error::Error for ModuleError {}

/// Error returned when an exported symbol cannot be found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolError {
    Module(ModuleError),
    NotFound,
}

impl fmt::Display for SymbolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Module(error) => write!(f, "{}", error),
            Self::NotFound => write!(f, "requested export was not found"),
        }
    }
}

impl core::error::Error for SymbolError {}

impl From<ModuleError> for SymbolError {
    #[inline(always)]
    fn from(value: ModuleError) -> Self {
        Self::Module(value)
    }
}

/// Error returned when a `RequestPluginAPI` style call fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestApiError {
    Symbol(SymbolError),
    NullInterface,
}

impl fmt::Display for RequestApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Symbol(error) => write!(f, "{}", error),
            Self::NullInterface => write!(f, "plugin API request returned a null interface"),
        }
    }
}

impl core::error::Error for RequestApiError {}

impl From<SymbolError> for RequestApiError {
    #[inline(always)]
    fn from(value: SymbolError) -> Self {
        Self::Symbol(value)
    }
}

impl From<ModuleError> for RequestApiError {
    #[inline(always)]
    fn from(value: ModuleError) -> Self {
        Self::Symbol(value.into())
    }
}
