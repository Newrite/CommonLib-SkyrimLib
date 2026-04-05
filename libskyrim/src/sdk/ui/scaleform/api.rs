use alloc::ffi::NulError;

use crate::re::{FxDelegate, GFxMovieSetVarType, GFxMovieView, GFxValue, GPtr};
use crate::sdk::ui::menus;
use crate::sdk::ui::menus::NamedMenu;

use super::surface::MenuSurface;

/// Looks up the Scaleform surface for one open menu.
#[inline(always)]
pub fn surface(menu_name: &str) -> Option<MenuSurface> {
    MenuSurface::lookup(menu_name)
}

#[inline(always)]
fn surface_or_warn(menu_name: &str, _caller: &'static str) -> Option<MenuSurface> {
    let surface = surface(menu_name);
    if surface.is_none() {
        crate::defensive_sdk_warn!(
            "{} skipped because menu '{}' is not open or has no stable UI surface",
            _caller,
            menu_name
        );
    }
    surface
}

/// Typed variant of [`surface`].
#[inline(always)]
pub fn named_surface<M>() -> Option<MenuSurface>
where
    M: NamedMenu,
{
    MenuSurface::lookup_named::<M>()
}

/// Returns the top-most menu surface up to the given depth limit.
#[inline(always)]
pub fn top_most_surface(depth_limit: u32) -> Option<MenuSurface> {
    MenuSurface::top_most(depth_limit)
}

/// Returns the top-most menu surface using the default depth limit.
#[inline(always)]
pub fn top_most_surface_default() -> Option<MenuSurface> {
    MenuSurface::top_most_default()
}

/// Returns whether the named menu exposes a variable/path.
#[inline(always)]
pub fn is_available(menu_name: &str, path: &str) -> Result<bool, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::is_available()")
        .map_or(Ok(false), |surface| surface.is_available(path))
}

/// Typed variant of [`is_available`].
#[inline(always)]
pub fn named_is_available<M>(path: &str) -> Result<bool, NulError>
where
    M: NamedMenu,
{
    is_available(M::MENU_NAME, path)
}

/// Returns the named menu's variable value, if available.
#[inline(always)]
pub fn variable(menu_name: &str, path: &str) -> Result<Option<GFxValue>, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::variable()")
        .map_or(Ok(None), |surface| surface.variable(path))
}

/// Typed variant of [`variable`].
#[inline(always)]
pub fn named_variable<M>(path: &str) -> Result<Option<GFxValue>, NulError>
where
    M: NamedMenu,
{
    variable(M::MENU_NAME, path)
}

/// Sets a variable on the named menu surface.
#[inline(always)]
pub fn set_variable(
    menu_name: &str,
    path: &str,
    value: &GFxValue,
    set_type: GFxMovieSetVarType,
) -> Result<bool, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::set_variable()").map_or(Ok(false), |surface| {
        surface.set_variable(path, value, set_type)
    })
}

/// Bool convenience variant of [`set_variable`].
#[inline(always)]
pub fn set_variable_bool(
    menu_name: &str,
    path: &str,
    value: bool,
    set_type: GFxMovieSetVarType,
) -> Result<bool, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::set_variable_bool()")
        .map_or(Ok(false), |surface| {
            surface.set_variable_bool(path, value, set_type)
        })
}

/// Number convenience variant of [`set_variable`].
#[inline(always)]
pub fn set_variable_number(
    menu_name: &str,
    path: &str,
    value: f64,
    set_type: GFxMovieSetVarType,
) -> Result<bool, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::set_variable_number()")
        .map_or(Ok(false), |surface| {
            surface.set_variable_number(path, value, set_type)
        })
}

/// Invokes a Scaleform method on the named menu surface.
#[inline(always)]
pub fn invoke(
    menu_name: &str,
    method_name: &str,
    result: Option<&mut GFxValue>,
    args: &[GFxValue],
) -> Result<bool, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::invoke()").map_or(Ok(false), |surface| {
        surface.invoke(method_name, result, args)
    })
}

/// Invokes a Scaleform method and ignores the return value.
#[inline(always)]
pub fn invoke_no_return(
    menu_name: &str,
    method_name: &str,
    args: &[GFxValue],
) -> Result<bool, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::invoke_no_return()")
        .map_or(Ok(false), |surface| {
            surface.invoke_no_return(method_name, args)
        })
}

/// Returns the named menu's `GFxMovieView`, if available.
#[inline(always)]
pub fn movie_view(menu_name: &str) -> GPtr<GFxMovieView> {
    surface(menu_name)
        .map(|surface| surface.movie_view())
        .unwrap_or_default()
}

/// Typed variant of [`movie_view`].
#[inline(always)]
pub fn named_movie_view<M>() -> GPtr<GFxMovieView>
where
    M: NamedMenu,
{
    movie_view(M::MENU_NAME)
}

/// Returns the top-most menu's `GFxMovieView`, if available.
#[inline(always)]
pub fn top_most_movie_view(depth_limit: u32) -> GPtr<GFxMovieView> {
    top_most_surface(depth_limit)
        .map(|surface| surface.movie_view())
        .unwrap_or_default()
}

/// Returns the top-most menu's `GFxMovieView` using the default depth limit.
#[inline(always)]
pub fn top_most_movie_view_default() -> GPtr<GFxMovieView> {
    top_most_movie_view(menus::DEFAULT_TOP_MOST_MENU_DEPTH_LIMIT)
}

/// Returns the named menu's `FxDelegate`, if available.
#[inline(always)]
pub fn fx_delegate(menu_name: &str) -> GPtr<FxDelegate> {
    surface(menu_name)
        .map(|surface| surface.fx_delegate())
        .unwrap_or_default()
}

/// Typed variant of [`fx_delegate`].
#[inline(always)]
pub fn named_fx_delegate<M>() -> GPtr<FxDelegate>
where
    M: NamedMenu,
{
    fx_delegate(M::MENU_NAME)
}

/// Returns the top-most menu's `FxDelegate`, if available.
#[inline(always)]
pub fn top_most_fx_delegate(depth_limit: u32) -> GPtr<FxDelegate> {
    top_most_surface(depth_limit)
        .map(|surface| surface.fx_delegate())
        .unwrap_or_default()
}

/// Returns the top-most menu's `FxDelegate` using the default depth limit.
#[inline(always)]
pub fn top_most_fx_delegate_default() -> GPtr<FxDelegate> {
    top_most_fx_delegate(menus::DEFAULT_TOP_MOST_MENU_DEPTH_LIMIT)
}
