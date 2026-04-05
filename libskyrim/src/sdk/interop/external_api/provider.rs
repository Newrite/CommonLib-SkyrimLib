use core::ffi::c_void;

/// Convert a plugin-owned `'static` API table into an exported C ABI pointer.
#[inline(always)]
pub fn export_api<T>(api: &'static T) -> *mut c_void {
    (api as *const T).cast_mut().cast::<c_void>()
}

/// Convert an optional plugin-owned API table into an exported C ABI pointer.
///
/// `None` maps to a null pointer, matching the common `RequestPluginAPI`
/// convention for unsupported versions or unavailable services.
#[inline(always)]
pub fn export_api_option<T>(api: Option<&'static T>) -> *mut c_void {
    match api {
        Some(api) => export_api(api),
        None => core::ptr::null_mut(),
    }
}

/// Return the API table only when the requested version matches exactly.
#[inline(always)]
pub fn select_api_for_version<T, V>(
    requested: V,
    supported: V,
    api: &'static T,
) -> Option<&'static T>
where
    V: PartialEq,
{
    (requested == supported).then_some(api)
}

/// Return the first API table whose version matches the requested value.
#[inline(always)]
pub fn select_api_for_versions<T, V, const N: usize>(
    requested: V,
    supported: &[(V, &'static T); N],
) -> Option<&'static T>
where
    V: Copy + PartialEq,
{
    supported
        .iter()
        .find_map(|(version, api)| (*version == requested).then_some(*api))
}

/// Small provider-side wrapper around a single exported service table.
///
/// This is useful when a plugin wants to model one capability as:
///
/// - available with a concrete `'static` table
/// - unavailable and therefore exported as null
///
/// without rebuilding the same `Option<&'static T> -> *mut c_void` glue in
/// every export function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExportedService<T: 'static> {
    service: Option<&'static T>,
}

impl<T: 'static> ExportedService<T> {
    #[inline(always)]
    pub const fn new(service: &'static T) -> Self {
        Self {
            service: Some(service),
        }
    }

    #[inline(always)]
    pub const fn optional(service: Option<&'static T>) -> Self {
        Self { service }
    }

    #[inline(always)]
    pub const fn unavailable() -> Self {
        Self { service: None }
    }

    #[inline(always)]
    pub const fn is_available(&self) -> bool {
        self.service.is_some()
    }

    #[inline(always)]
    pub const fn get(&self) -> Option<&'static T> {
        self.service
    }

    #[inline(always)]
    pub fn export(&self) -> *mut c_void {
        export_api_option(self.service)
    }
}

impl<T: 'static> From<&'static T> for ExportedService<T> {
    #[inline(always)]
    fn from(value: &'static T) -> Self {
        Self::new(value)
    }
}

impl<T: 'static> From<Option<&'static T>> for ExportedService<T> {
    #[inline(always)]
    fn from(value: Option<&'static T>) -> Self {
        Self::optional(value)
    }
}

/// Provider-side registry for versioned exported service tables.
///
/// This is the provider-side companion to `RequestPluginAPI`-style consumers:
/// a plugin can keep one small table of supported versions and then export the
/// matching table for the requested version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VersionedServiceRegistry<V, T: 'static, const N: usize> {
    supported: [(V, &'static T); N],
}

impl<V, T: 'static, const N: usize> VersionedServiceRegistry<V, T, N> {
    #[inline(always)]
    pub const fn new(supported: [(V, &'static T); N]) -> Self {
        Self { supported }
    }

    #[inline(always)]
    pub const fn len(&self) -> usize {
        N
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    #[inline(always)]
    pub const fn as_slice(&self) -> &[(V, &'static T); N] {
        &self.supported
    }

    #[inline(always)]
    pub fn select(&self, requested: V) -> Option<&'static T>
    where
        V: Copy + PartialEq,
    {
        select_api_for_versions(requested, &self.supported)
    }

    #[inline(always)]
    pub fn supports(&self, requested: V) -> bool
    where
        V: Copy + PartialEq,
    {
        self.select(requested).is_some()
    }

    #[inline(always)]
    pub fn export(&self, requested: V) -> *mut c_void
    where
        V: Copy + PartialEq,
    {
        export_api_option(self.select(requested))
    }
}
