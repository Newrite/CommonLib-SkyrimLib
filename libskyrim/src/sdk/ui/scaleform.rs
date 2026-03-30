//! Scaleform-oriented SDK helpers.
//!
//! Current support focuses on the menu-side Scaleform surfaces we can already
//! model honestly:
//!
//! - access to open menu `GFxMovieView` instances
//! - access to menu `FxDelegate` pointers
//! - top-most menu Scaleform lookup
//! - menu movie `invoke` / `get_variable` / `set_variable` helpers
//!
//! ActionScript object traversal sugar and C++ callback registration remain
//! intentionally narrow; the raw `GFxValue` surface is now present, but we keep
//! SDK helpers centered on the repeated menu-side workflows plugins actually
//! need first.

use alloc::ffi::{CString, NulError};
use core::ffi::CStr;
use core::fmt;
use core::ops::Deref;

use crate::re::{FxDelegate, GFxMovieSetVarType, GFxMovieView, GFxValue, GPtr, IMenu};

use super::menus;
use super::menus::NamedMenu;

/// Owner-backed view of one currently open menu and its Scaleform surfaces.
#[derive(Clone, PartialEq, Eq)]
#[must_use]
pub struct MenuSurface {
    menu: GPtr<IMenu>,
}

impl MenuSurface {
    #[inline(always)]
    pub fn from_menu(menu: GPtr<IMenu>) -> Option<Self> {
        if menu.is_null() {
            None
        } else {
            Some(Self { menu })
        }
    }

    #[inline(always)]
    pub fn lookup(menu_name: &str) -> Option<Self> {
        Self::from_menu(menus::menu(menu_name))
    }

    #[inline(always)]
    pub fn lookup_named<M>() -> Option<Self>
    where
        M: NamedMenu,
    {
        Self::lookup(M::MENU_NAME)
    }

    #[inline(always)]
    pub fn top_most(depth_limit: u32) -> Option<Self> {
        Self::from_menu(menus::top_most_menu(depth_limit))
    }

    #[inline(always)]
    pub fn top_most_default() -> Option<Self> {
        Self::top_most(menus::DEFAULT_TOP_MOST_MENU_DEPTH_LIMIT)
    }

    #[inline(always)]
    pub fn menu_owner(&self) -> &GPtr<IMenu> {
        &self.menu
    }

    #[inline(always)]
    pub fn menu(&self) -> &IMenu {
        &self.menu
    }

    #[inline(always)]
    pub fn menu_ptr(&self) -> *mut IMenu {
        self.menu.as_ptr()
    }

    #[inline(always)]
    pub fn into_menu(self) -> GPtr<IMenu> {
        self.menu
    }

    #[inline(always)]
    pub fn is_open(&self) -> bool {
        self.menu.on_stack()
    }

    #[inline(always)]
    pub fn pauses_game(&self) -> bool {
        self.menu.pauses_game()
    }

    #[inline(always)]
    pub fn uses_cursor(&self) -> bool {
        self.menu.uses_cursor()
    }

    #[inline(always)]
    pub fn update_uses_cursor(&self) -> bool {
        self.menu.update_uses_cursor()
    }

    #[inline(always)]
    pub fn modal(&self) -> bool {
        self.menu.modal()
    }

    #[inline(always)]
    pub fn has_movie_view(&self) -> bool {
        !self.menu.ui_movie.is_null()
    }

    #[inline(always)]
    pub fn movie_view(&self) -> GPtr<GFxMovieView> {
        self.menu.ui_movie.clone()
    }

    #[inline(always)]
    pub fn movie_view_ptr(&self) -> *mut GFxMovieView {
        self.menu.ui_movie.as_ptr()
    }

    #[inline(always)]
    pub fn has_delegate(&self) -> bool {
        !self.menu.fx_delegate.is_null()
    }

    #[inline(always)]
    pub fn fx_delegate(&self) -> GPtr<FxDelegate> {
        self.menu.fx_delegate.clone()
    }

    #[inline(always)]
    pub fn fx_delegate_ptr(&self) -> *mut FxDelegate {
        self.menu.fx_delegate.as_ptr()
    }

    #[inline(always)]
    pub fn is_available_c_str(&self, path: &CStr) -> bool {
        let movie = self.movie_view();
        if movie.is_null() {
            crate::defensive_sdk_warn!(
                "sdk::ui::scaleform::MenuSurface::is_available_c_str() skipped because the menu has no GFxMovieView"
            );
            false
        } else {
            movie.is_available(path.as_ptr())
        }
    }

    #[inline(always)]
    pub fn is_available_str(&self, path: &str) -> Result<bool, NulError> {
        let path = match CString::new(path) {
            Ok(path) => path,
            Err(err) => {
                crate::defensive_sdk_warn!(
                    "sdk::ui::scaleform::MenuSurface::is_available_str() rejected path with interior NUL"
                );
                return Err(err);
            }
        };
        Ok(self.is_available_c_str(&path))
    }

    #[inline(always)]
    pub fn get_variable_c_str(&self, path: &CStr, value: &mut GFxValue) -> bool {
        let movie = self.movie_view();
        if movie.is_null() {
            crate::defensive_sdk_warn!(
                "sdk::ui::scaleform::MenuSurface::get_variable_c_str() skipped because the menu has no GFxMovieView"
            );
            false
        } else {
            movie.get_variable(value, path.as_ptr())
        }
    }

    #[inline(always)]
    pub fn get_variable_str(&self, path: &str, value: &mut GFxValue) -> Result<bool, NulError> {
        let path = match CString::new(path) {
            Ok(path) => path,
            Err(err) => {
                crate::defensive_sdk_warn!(
                    "sdk::ui::scaleform::MenuSurface::get_variable_str() rejected path with interior NUL"
                );
                return Err(err);
            }
        };
        Ok(self.get_variable_c_str(&path, value))
    }

    #[inline(always)]
    pub fn variable_c_str(&self, path: &CStr) -> Option<GFxValue> {
        let mut value = GFxValue::default();
        self.get_variable_c_str(path, &mut value).then_some(value)
    }

    #[inline(always)]
    pub fn variable_str(&self, path: &str) -> Result<Option<GFxValue>, NulError> {
        let path = match CString::new(path) {
            Ok(path) => path,
            Err(err) => {
                crate::defensive_sdk_warn!(
                    "sdk::ui::scaleform::MenuSurface::variable_str() rejected path with interior NUL"
                );
                return Err(err);
            }
        };
        Ok(self.variable_c_str(&path))
    }

    #[inline(always)]
    pub fn set_variable_c_str(
        &self,
        path: &CStr,
        value: &GFxValue,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        let mut movie = self.movie_view();
        if movie.is_null() {
            crate::defensive_sdk_warn!(
                "sdk::ui::scaleform::MenuSurface::set_variable_c_str() skipped because the menu has no GFxMovieView"
            );
            false
        } else {
            movie.set_variable(path.as_ptr(), value, set_type)
        }
    }

    #[inline(always)]
    pub fn set_variable_str(
        &self,
        path: &str,
        value: &GFxValue,
        set_type: GFxMovieSetVarType,
    ) -> Result<bool, NulError> {
        let path = match CString::new(path) {
            Ok(path) => path,
            Err(err) => {
                crate::defensive_sdk_warn!(
                    "sdk::ui::scaleform::MenuSurface::set_variable_str() rejected path with interior NUL"
                );
                return Err(err);
            }
        };
        Ok(self.set_variable_c_str(&path, value, set_type))
    }

    #[inline(always)]
    pub fn set_variable_bool_c_str(
        &self,
        path: &CStr,
        value: bool,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        self.set_variable_c_str(path, &GFxValue::from_bool(value), set_type)
    }

    #[inline(always)]
    pub fn set_variable_bool_str(
        &self,
        path: &str,
        value: bool,
        set_type: GFxMovieSetVarType,
    ) -> Result<bool, NulError> {
        let path = match CString::new(path) {
            Ok(path) => path,
            Err(err) => {
                crate::defensive_sdk_warn!(
                    "sdk::ui::scaleform::MenuSurface::set_variable_bool_str() rejected path with interior NUL"
                );
                return Err(err);
            }
        };
        Ok(self.set_variable_bool_c_str(&path, value, set_type))
    }

    #[inline(always)]
    pub fn set_variable_number_c_str(
        &self,
        path: &CStr,
        value: f64,
        set_type: GFxMovieSetVarType,
    ) -> bool {
        self.set_variable_c_str(path, &GFxValue::from_number(value), set_type)
    }

    #[inline(always)]
    pub fn set_variable_number_str(
        &self,
        path: &str,
        value: f64,
        set_type: GFxMovieSetVarType,
    ) -> Result<bool, NulError> {
        let path = match CString::new(path) {
            Ok(path) => path,
            Err(err) => {
                crate::defensive_sdk_warn!(
                    "sdk::ui::scaleform::MenuSurface::set_variable_number_str() rejected path with interior NUL"
                );
                return Err(err);
            }
        };
        Ok(self.set_variable_number_c_str(&path, value, set_type))
    }

    #[inline(always)]
    pub fn invoke_c_str(
        &self,
        method_name: &CStr,
        result: Option<&mut GFxValue>,
        args: &[GFxValue],
    ) -> bool {
        let mut movie = self.movie_view();
        if movie.is_null() {
            crate::defensive_sdk_warn!(
                "sdk::ui::scaleform::MenuSurface::invoke_c_str() skipped because the menu has no GFxMovieView"
            );
            false
        } else {
            movie.invoke(
                method_name.as_ptr(),
                result.map_or(core::ptr::null_mut(), core::ptr::from_mut),
                args.as_ptr(),
                args.len() as u32,
            )
        }
    }

    #[inline(always)]
    pub fn invoke_str(
        &self,
        method_name: &str,
        result: Option<&mut GFxValue>,
        args: &[GFxValue],
    ) -> Result<bool, NulError> {
        let method_name = match CString::new(method_name) {
            Ok(method_name) => method_name,
            Err(err) => {
                crate::defensive_sdk_warn!(
                    "sdk::ui::scaleform::MenuSurface::invoke_str() rejected method name with interior NUL"
                );
                return Err(err);
            }
        };
        Ok(self.invoke_c_str(&method_name, result, args))
    }

    #[inline(always)]
    pub fn invoke_no_args_c_str(&self, method_name: &CStr, result: Option<&mut GFxValue>) -> bool {
        self.invoke_c_str(method_name, result, &[])
    }

    #[inline(always)]
    pub fn invoke_no_args_str(
        &self,
        method_name: &str,
        result: Option<&mut GFxValue>,
    ) -> Result<bool, NulError> {
        let method_name = match CString::new(method_name) {
            Ok(method_name) => method_name,
            Err(err) => {
                crate::defensive_sdk_warn!(
                    "sdk::ui::scaleform::MenuSurface::invoke_no_args_str() rejected method name with interior NUL"
                );
                return Err(err);
            }
        };
        Ok(self.invoke_no_args_c_str(&method_name, result))
    }

    #[inline(always)]
    pub fn invoke_no_return_c_str(&self, method_name: &CStr, args: &[GFxValue]) -> bool {
        let movie = self.movie_view();
        if movie.is_null() {
            crate::defensive_sdk_warn!(
                "sdk::ui::scaleform::MenuSurface::invoke_no_return_c_str() skipped because the menu has no GFxMovieView"
            );
            return false;
        }

        movie.invoke_no_return(method_name.as_ptr(), args.as_ptr(), args.len() as u32);
        true
    }

    #[inline(always)]
    pub fn invoke_no_return_str(
        &self,
        method_name: &str,
        args: &[GFxValue],
    ) -> Result<bool, NulError> {
        let method_name = match CString::new(method_name) {
            Ok(method_name) => method_name,
            Err(err) => {
                crate::defensive_sdk_warn!(
                    "sdk::ui::scaleform::MenuSurface::invoke_no_return_str() rejected method name with interior NUL"
                );
                return Err(err);
            }
        };
        Ok(self.invoke_no_return_c_str(&method_name, args))
    }
}

impl fmt::Debug for MenuSurface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MenuSurface")
            .field("menu_ptr", &self.menu_ptr())
            .field("movie_view_ptr", &self.movie_view_ptr())
            .field("fx_delegate_ptr", &self.fx_delegate_ptr())
            .finish()
    }
}

impl AsRef<IMenu> for MenuSurface {
    #[inline(always)]
    fn as_ref(&self) -> &IMenu {
        self.menu()
    }
}

impl Deref for MenuSurface {
    type Target = IMenu;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.menu()
    }
}

#[inline(always)]
pub fn surface(menu_name: &str) -> Option<MenuSurface> {
    MenuSurface::lookup(menu_name)
}

#[inline(always)]
fn surface_or_warn(menu_name: &str, caller: &'static str) -> Option<MenuSurface> {
    let surface = surface(menu_name);
    if surface.is_none() {
        crate::defensive_sdk_warn!(
            "{} skipped because menu '{}' is not open or has no stable UI surface",
            caller,
            menu_name
        );
    }
    surface
}

#[inline(always)]
pub fn named_surface<M>() -> Option<MenuSurface>
where
    M: NamedMenu,
{
    MenuSurface::lookup_named::<M>()
}

#[inline(always)]
pub fn top_most_surface(depth_limit: u32) -> Option<MenuSurface> {
    MenuSurface::top_most(depth_limit)
}

#[inline(always)]
pub fn top_most_surface_default() -> Option<MenuSurface> {
    MenuSurface::top_most_default()
}

#[inline(always)]
pub fn is_available(menu_name: &str, path: &str) -> Result<bool, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::is_available()")
        .map_or(Ok(false), |surface| surface.is_available_str(path))
}

#[inline(always)]
pub fn named_is_available<M>(path: &str) -> Result<bool, NulError>
where
    M: NamedMenu,
{
    is_available(M::MENU_NAME, path)
}

#[inline(always)]
pub fn variable(menu_name: &str, path: &str) -> Result<Option<GFxValue>, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::variable()")
        .map_or(Ok(None), |surface| surface.variable_str(path))
}

#[inline(always)]
pub fn named_variable<M>(path: &str) -> Result<Option<GFxValue>, NulError>
where
    M: NamedMenu,
{
    variable(M::MENU_NAME, path)
}

#[inline(always)]
pub fn set_variable(
    menu_name: &str,
    path: &str,
    value: &GFxValue,
    set_type: GFxMovieSetVarType,
) -> Result<bool, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::set_variable()").map_or(Ok(false), |surface| {
        surface.set_variable_str(path, value, set_type)
    })
}

#[inline(always)]
pub fn set_variable_bool(
    menu_name: &str,
    path: &str,
    value: bool,
    set_type: GFxMovieSetVarType,
) -> Result<bool, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::set_variable_bool()")
        .map_or(Ok(false), |surface| {
            surface.set_variable_bool_str(path, value, set_type)
        })
}

#[inline(always)]
pub fn set_variable_number(
    menu_name: &str,
    path: &str,
    value: f64,
    set_type: GFxMovieSetVarType,
) -> Result<bool, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::set_variable_number()")
        .map_or(Ok(false), |surface| {
            surface.set_variable_number_str(path, value, set_type)
        })
}

#[inline(always)]
pub fn invoke(
    menu_name: &str,
    method_name: &str,
    result: Option<&mut GFxValue>,
    args: &[GFxValue],
) -> Result<bool, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::invoke()").map_or(Ok(false), |surface| {
        surface.invoke_str(method_name, result, args)
    })
}

#[inline(always)]
pub fn invoke_no_return(
    menu_name: &str,
    method_name: &str,
    args: &[GFxValue],
) -> Result<bool, NulError> {
    surface_or_warn(menu_name, "sdk::ui::scaleform::invoke_no_return()")
        .map_or(Ok(false), |surface| {
            surface.invoke_no_return_str(method_name, args)
        })
}

#[inline(always)]
pub fn movie_view(menu_name: &str) -> GPtr<GFxMovieView> {
    surface(menu_name)
        .map(|surface| surface.movie_view())
        .unwrap_or_default()
}

#[inline(always)]
pub fn named_movie_view<M>() -> GPtr<GFxMovieView>
where
    M: NamedMenu,
{
    movie_view(M::MENU_NAME)
}

#[inline(always)]
pub fn top_most_movie_view(depth_limit: u32) -> GPtr<GFxMovieView> {
    top_most_surface(depth_limit)
        .map(|surface| surface.movie_view())
        .unwrap_or_default()
}

#[inline(always)]
pub fn top_most_movie_view_default() -> GPtr<GFxMovieView> {
    top_most_movie_view(menus::DEFAULT_TOP_MOST_MENU_DEPTH_LIMIT)
}

#[inline(always)]
pub fn fx_delegate(menu_name: &str) -> GPtr<FxDelegate> {
    surface(menu_name)
        .map(|surface| surface.fx_delegate())
        .unwrap_or_default()
}

#[inline(always)]
pub fn named_fx_delegate<M>() -> GPtr<FxDelegate>
where
    M: NamedMenu,
{
    fx_delegate(M::MENU_NAME)
}

#[inline(always)]
pub fn top_most_fx_delegate(depth_limit: u32) -> GPtr<FxDelegate> {
    top_most_surface(depth_limit)
        .map(|surface| surface.fx_delegate())
        .unwrap_or_default()
}

#[inline(always)]
pub fn top_most_fx_delegate_default() -> GPtr<FxDelegate> {
    top_most_fx_delegate(menus::DEFAULT_TOP_MOST_MENU_DEPTH_LIMIT)
}
