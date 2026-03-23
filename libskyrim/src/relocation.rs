use crate::ffi;
use crate::runtime;
use core::ffi::c_void;
use core::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelocationError {
    UnresolvedId(usize),
    UnresolvedOffset(usize),
    UnsupportedCallHookSize(usize),
    UnsupportedBranchHookSize(usize),
    MissingRuntimeDynamicCast,
}

pub trait TryIntoAddress {
    fn try_into_address(self) -> Result<usize, RelocationError>;
}

pub trait IntoAddress: TryIntoAddress {
    fn into_address(self) -> usize
    where
        Self: Sized,
    {
        self.try_into_address().unwrap_or(0)
    }
}

impl<T: TryIntoAddress> IntoAddress for T {}

pub trait TryIntoOffset {
    fn try_into_offset(self) -> Result<usize, RelocationError>;
}

pub trait IntoOffset: TryIntoOffset {
    fn into_offset(self) -> usize
    where
        Self: Sized,
    {
        self.try_into_offset().unwrap_or(0)
    }
}

impl<T: TryIntoOffset> IntoOffset for T {}

impl TryIntoAddress for usize {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        Ok(self)
    }
}

impl TryIntoOffset for usize {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        Ok(self)
    }
}

#[inline(always)]
fn module_base() -> usize {
    unsafe { ffi::commonlib_offset_to_address(0) }
}

#[inline(always)]
fn resolve_id_address(id: usize) -> Result<usize, RelocationError> {
    let address = unsafe { ffi::commonlib_id_to_address(id) };
    if address == 0 {
        Err(RelocationError::UnresolvedId(id))
    } else {
        Ok(address)
    }
}

#[inline(always)]
fn resolve_offset_address(offset: usize) -> Result<usize, RelocationError> {
    let address = unsafe { ffi::commonlib_offset_to_address(offset) };
    if address == 0 {
        Err(RelocationError::UnresolvedOffset(offset))
    } else {
        Ok(address)
    }
}

fn fatal_resolution(context: &str, error: RelocationError) -> ! {
    match error {
        RelocationError::UnresolvedId(id) => crate::log::fatal_runtime(format_args!(
            "{}: Address Library failed to resolve ID {}",
            context, id
        )),
        RelocationError::UnresolvedOffset(offset) => crate::log::fatal_runtime(format_args!(
            "{}: failed to resolve offset {:#X}",
            context, offset
        )),
        RelocationError::UnsupportedCallHookSize(size) => crate::log::fatal_runtime(format_args!(
            "{}: unsupported call hook size {}. Only 5 or 6 are supported.",
            context, size
        )),
        RelocationError::UnsupportedBranchHookSize(size) => {
            crate::log::fatal_runtime(format_args!(
                "{}: unsupported branch hook size {}. Only 5 or 6 are supported.",
                context, size
            ))
        }
        RelocationError::MissingRuntimeDynamicCast => crate::log::fatal_runtime(format_args!(
            "{}: failed to resolve RTDynamicCast address",
            context
        )),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ID {
    id: usize,
}

impl ID {
    pub const fn new(id: usize) -> Self {
        Self { id }
    }

    pub const fn id(self) -> usize {
        self.id
    }

    pub fn try_offset(self) -> Result<usize, RelocationError> {
        Ok(resolve_id_address(self.id)? - module_base())
    }

    pub fn offset(self) -> usize {
        self.try_offset().unwrap_or(0)
    }

    pub fn try_address(self) -> Result<usize, RelocationError> {
        resolve_id_address(self.id)
    }

    pub fn address(self) -> usize {
        self.try_address().unwrap_or(0)
    }
}

impl TryIntoAddress for ID {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        self.try_address()
    }
}

impl TryIntoOffset for ID {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        self.try_offset()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Offset {
    offset: usize,
}

impl Offset {
    pub const fn new(offset: usize) -> Self {
        Self { offset }
    }

    pub const fn offset(self) -> usize {
        self.offset
    }

    pub fn try_address(self) -> Result<usize, RelocationError> {
        resolve_offset_address(self.offset)
    }

    pub fn address(self) -> usize {
        self.try_address().unwrap_or(0)
    }
}

impl TryIntoAddress for Offset {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        self.try_address()
    }
}

impl TryIntoOffset for Offset {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        Ok(self.offset)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RelocationID {
    pub se: usize,
    pub ae: usize,
    pub vr: usize,
}

impl RelocationID {
    pub const fn new(se: usize, ae: usize) -> Self {
        Self { se, ae, vr: se }
    }

    pub const fn with_vr(se: usize, ae: usize, vr: usize) -> Self {
        Self { se, ae, vr }
    }

    pub fn id(self) -> usize {
        if runtime::is_vr() {
            self.vr
        } else if runtime::is_ae() {
            self.ae
        } else if runtime::is_se() {
            self.se
        } else {
            0
        }
    }

    pub fn active(self) -> ID {
        ID::new(self.id())
    }

    pub fn try_offset(self) -> Result<usize, RelocationError> {
        self.active().try_offset()
    }

    pub fn offset(self) -> usize {
        self.try_offset().unwrap_or(0)
    }

    pub fn try_address(self) -> Result<usize, RelocationError> {
        self.active().try_address()
    }

    pub fn address(self) -> usize {
        self.try_address().unwrap_or(0)
    }
}

impl TryIntoAddress for RelocationID {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        self.try_address()
    }
}

impl TryIntoOffset for RelocationID {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        self.try_offset()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VariantID {
    pub se: usize,
    pub ae: usize,
    pub vr: usize,
}

impl VariantID {
    pub const fn new(se: usize, ae: usize, vr: usize) -> Self {
        Self { se, ae, vr }
    }

    pub fn try_offset(self) -> Result<usize, RelocationError> {
        if runtime::is_vr() {
            Ok(self.vr)
        } else if runtime::is_ae() {
            ID::new(self.ae).try_offset()
        } else if runtime::is_se() {
            ID::new(self.se).try_offset()
        } else {
            Err(RelocationError::UnresolvedOffset(0))
        }
    }

    pub fn offset(self) -> usize {
        self.try_offset().unwrap_or(0)
    }

    pub fn try_address(self) -> Result<usize, RelocationError> {
        if runtime::is_vr() {
            Offset::new(self.vr).try_address()
        } else if runtime::is_ae() {
            ID::new(self.ae).try_address()
        } else if runtime::is_se() {
            ID::new(self.se).try_address()
        } else {
            Err(RelocationError::UnresolvedOffset(0))
        }
    }

    pub fn address(self) -> usize {
        self.try_address().unwrap_or(0)
    }
}

impl TryIntoAddress for VariantID {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        self.try_address()
    }
}

impl TryIntoOffset for VariantID {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        self.try_offset()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VariantOffset {
    pub se: usize,
    pub ae: usize,
    pub vr: usize,
}

impl VariantOffset {
    pub const fn new(se: usize, ae: usize, vr: usize) -> Self {
        Self { se, ae, vr }
    }

    pub const fn new_se_ae(se_ae: usize, vr: usize) -> Self {
        Self {
            se: se_ae,
            ae: se_ae,
            vr,
        }
    }

    #[inline(always)]
    pub fn offset(self) -> usize {
        if runtime::is_vr() {
            self.vr
        } else if runtime::is_ae() {
            self.ae
        } else {
            self.se
        }
    }

    pub fn try_address(self) -> Result<usize, RelocationError> {
        Offset::new(self.offset()).try_address()
    }

    pub fn address(self) -> usize {
        self.try_address().unwrap_or(0)
    }
}

impl TryIntoAddress for VariantOffset {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        self.try_address()
    }
}

impl TryIntoOffset for VariantOffset {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        Ok(self.offset())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Relocation<T = usize> {
    address: usize,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Relocation<T> {
    pub const fn from_address(address: usize) -> Self {
        Self {
            address,
            _marker: PhantomData,
        }
    }

    pub fn new<A: TryIntoAddress>(source: A) -> Self {
        match Self::try_new(source) {
            Ok(relocation) => relocation,
            Err(error) => fatal_resolution("Failed to resolve relocation", error),
        }
    }

    pub fn try_new<A: TryIntoAddress>(source: A) -> Result<Self, RelocationError> {
        Ok(Self::from_address(source.try_into_address()?))
    }

    pub const fn address(self) -> usize {
        self.address
    }

    pub fn offset(self) -> usize {
        self.address.saturating_sub(module_base())
    }

    pub const fn is_null(self) -> bool {
        self.address == 0
    }

    pub const fn cast<U>(self) -> Relocation<U> {
        Relocation::from_address(self.address)
    }

    pub const fn as_ptr(self) -> *const T {
        self.address as *const T
    }

    pub const fn as_mut_ptr(self) -> *mut T {
        self.address as *mut T
    }

    pub fn write_bytes(self, bytes: &[u8]) {
        safe_write(self.address, bytes);
    }

    pub fn fill_bytes(self, value: u8, count: usize) {
        safe_fill(self.address, value, count);
    }

    pub fn write_call(self, dst: usize, size: usize) -> usize {
        write_call(self.address, dst, size)
    }

    pub fn write_branch(self, dst: usize, size: usize) -> usize {
        write_branch(self.address, dst, size)
    }

    pub fn write_vfunc<I: IntoOffset>(self, index: I, new_func: usize) -> usize {
        write_vfunc(self.address, index, new_func)
    }
}

impl<T: Copy> Relocation<T> {
    pub unsafe fn get(self) -> T {
        debug_assert_eq!(core::mem::size_of::<T>(), core::mem::size_of::<usize>());
        unsafe { core::mem::transmute_copy(&self.address) }
    }
}

impl<T> TryIntoAddress for Relocation<T> {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        Ok(self.address)
    }
}

impl<T> TryIntoOffset for Relocation<T> {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        Ok(self.offset())
    }
}

pub fn safe_write<A: IntoAddress>(address: A, bytes: &[u8]) {
    unsafe {
        ffi::commonlib_safe_write(address.into_address(), bytes.as_ptr(), bytes.len());
    }
}

pub fn safe_fill<A: IntoAddress>(address: A, value: u8, count: usize) {
    unsafe {
        ffi::commonlib_safe_fill(address.into_address(), value, count);
    }
}

pub fn write_vfunc<A: IntoAddress, I: IntoOffset>(
    vtable_addr: A,
    index: I,
    new_func: usize,
) -> usize {
    unsafe { ffi::commonlib_write_vfunc(vtable_addr.into_address(), index.into_offset(), new_func) }
}

pub fn write_call<A: IntoAddress>(src: A, dst: usize, size: usize) -> usize {
    unsafe {
        match size {
            5 => ffi::commonlib_write_call5(src.into_address(), dst),
            6 => ffi::commonlib_write_call6(src.into_address(), dst),
            _ => fatal_resolution(
                "Failed to install call hook",
                RelocationError::UnsupportedCallHookSize(size),
            ),
        }
    }
}

pub fn write_branch<A: IntoAddress>(src: A, dst: usize, size: usize) -> usize {
    unsafe {
        match size {
            5 => ffi::commonlib_write_branch5(src.into_address(), dst),
            6 => ffi::commonlib_write_branch6(src.into_address(), dst),
            _ => fatal_resolution(
                "Failed to install branch hook",
                RelocationError::UnsupportedBranchHookSize(size),
            ),
        }
    }
}

pub trait RttiType {
    const RTTI: VariantID;
}

pub unsafe fn skyrim_cast<T: RttiType, U: RttiType>(from: *mut T) -> *mut U {
    if from.is_null() {
        return core::ptr::null_mut();
    }

    type RTDynamicCastFn = extern "C" fn(
        inptr: *mut c_void,
        vf_delta: i32,
        src_type: *const c_void,
        target_type: *const c_void,
        is_reference: i32,
    ) -> *mut c_void;

    let rtdc = match Relocation::<RTDynamicCastFn>::try_new(RelocationID::new(102238, 109689)) {
        Ok(relocation) => relocation,
        Err(_) => fatal_resolution("skyrim_cast", RelocationError::MissingRuntimeDynamicCast),
    };

    let from_rtti = T::RTTI.address() as *const c_void;
    let to_rtti = U::RTTI.address() as *const c_void;
    if from_rtti.is_null() || to_rtti.is_null() {
        return core::ptr::null_mut();
    }

    let rtdc = unsafe { rtdc.get() };
    let result = rtdc(from as *mut c_void, 0, from_rtti, to_rtti, 0);
    result as *mut U
}

#[inline(always)]
pub fn relocate<T>(se_and_vr: T, ae: T) -> T {
    if runtime::is_ae() { ae } else { se_and_vr }
}

#[inline(always)]
pub fn relocate_vr<T>(se: T, ae: T, vr: T) -> T {
    if runtime::is_vr() {
        vr
    } else if runtime::is_ae() {
        ae
    } else {
        se
    }
}

// ── МАКРОСЫ ДЛЯ ХУКОВ ────────────────────────────────────────────────

#[macro_export]
macro_rules! define_vtable_hook {
    (
        $vis:vis $hook_name:ident {
            vtable: $vtable:expr,
            offset: $offset:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),*) $(-> $ret:ty)? $body:block
        }
    ) => {
        #[allow(non_snake_case)]
        $vis mod $hook_name {
            use super::*;
            type Signature = extern "C" fn($($arg_name: $arg_type),*) $(-> $ret)?;
            type Original = $crate::relocation::Relocation<Signature>;
            static ORIGINAL: $crate::core_util::Later<Original> = $crate::core_util::Later::new();

            #[inline(always)]
            pub fn original_relocation() -> Original { *ORIGINAL }

            #[inline(always)]
            pub fn original($($arg_name: $arg_type),*) $(-> $ret)? {
                let original = unsafe { original_relocation().get() };
                original($($arg_name),*)
            }

            extern "C" fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? { $body }

            pub fn install() {
                use $crate::relocation::{IntoAddress, IntoOffset};

                let orig_addr = $crate::relocation::write_vfunc(
                    $vtable.into_address(),
                    $offset.into_offset(),
                    $hook_func as usize,
                );
                ORIGINAL.init($crate::relocation::Relocation::from_address(orig_addr));
            }
        }
    };
}

#[macro_export]
macro_rules! define_call_hook {
    (
        $vis:vis $hook_name:ident {
            address: $address:expr,
            offset: $offset:expr,
            size: $size:literal,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),*) $(-> $ret:ty)? $body:block
        }
    ) => {
        #[allow(non_snake_case)]
        $vis mod $hook_name {
            use super::*;
            type Signature = extern "C" fn($($arg_name: $arg_type),*) $(-> $ret)?;
            type Original = $crate::relocation::Relocation<Signature>;
            static ORIGINAL: $crate::core_util::Later<Original> = $crate::core_util::Later::new();

            #[inline(always)]
            pub fn original_relocation() -> Original { *ORIGINAL }

            #[inline(always)]
            pub fn original($($arg_name: $arg_type),*) $(-> $ret)? {
                let original = unsafe { original_relocation().get() };
                original($($arg_name),*)
            }

            extern "C" fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? { $body }

            pub fn install() {
                use $crate::relocation::{IntoAddress, IntoOffset};

                let target_addr = $address.into_address() + $offset.into_offset();
                let orig_addr = $crate::relocation::write_call(target_addr, $hook_func as usize, $size);
                ORIGINAL.init($crate::relocation::Relocation::from_address(orig_addr));
            }
        }
    };
}

#[macro_export]
macro_rules! virtual_method {
    (
        $const_vis:vis const $const_name:ident: $const_ty:ty = $const_expr:expr;
        $fn_vis:vis fn $func_name:ident($($arg_name:ident: $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        $const_vis const $const_name: $const_ty = $const_expr;

        #[inline(always)]
        $fn_vis fn $func_name(&self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            use $crate::relocation::IntoOffset;

            unsafe {
                let vtable = *(self as *const _ as *const *const usize);
                let func_ptr = vtable.add(Self::$const_name.into_offset());
                let func: extern "C" fn(*const Self $(, $arg_ty)*) $(-> $ret)? =
                    core::mem::transmute(*func_ptr);
                func(self $(, $arg_name)*)
            }
        }
    };
}

#[macro_export]
macro_rules! relocation_variable {
    (@no_inline $vis:vis fn $name:ident() -> &'static $ty:ty => $id:expr ) => {
        $vis fn $name() -> &'static $ty {
            $crate::relocation_variable!(@body $ty, $id)
        }
    };

    ( $vis:vis fn $name:ident() -> &'static $ty:ty => $id:expr ) => {
        #[inline]
        $vis fn $name() -> &'static $ty {
            $crate::relocation_variable!(@body $ty, $id)
        }
    };

    ( @no_inline $vis:vis fn $name:ident() -> *mut $ty:ty => $id:expr, is_ptr ) => {
        $vis fn $name() -> *mut $ty {
            $crate::relocation_variable!(@body_ptr $ty, $id)
        }
    };

    ( $vis:vis fn $name:ident() -> *mut $ty:ty => $id:expr, is_ptr ) => {
        #[inline]
        $vis fn $name() -> *mut $ty {
            $crate::relocation_variable!(@body_ptr $ty, $id)
        }
    };

    (@body $ty:ty, $id:expr) => {{
        let relocation = $crate::relocation::Relocation::<$ty>::new($id);
        unsafe { &*relocation.as_ptr() }
    }};

    (@body_ptr $ty:ty, $id:expr) => {{
        unsafe { $crate::relocation::Relocation::<*mut $ty>::new($id).get() }
    }};
}

#[macro_export]
macro_rules! relocation_func {
    ( @no_inline $vis:vis fn $name:ident($($arg_name:ident: $arg_ty:ty),* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        $vis fn $name($($arg_name: $arg_ty),*) $(-> $ret)? {
            $crate::relocation_func!(@body ($($arg_ty),*), ($($arg_name),*), $(-> $ret)?, $id)
        }
    };

    ( $vis:vis fn $name:ident($($arg_name:ident: $arg_ty:ty),* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        #[inline]
        $vis fn $name($($arg_name: $arg_ty),*) $(-> $ret)? {
            $crate::relocation_func!(@body ($($arg_ty),*), ($($arg_name),*), $(-> $ret)?, $id)
        }
    };

    ( @no_inline $vis:vis fn $name:ident(&self $(, $arg_name:ident: $arg_ty:ty)* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        $vis fn $name(&self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::relocation_func!(@body (*const Self $(, $arg_ty)*), (self as *const Self $(, $arg_name)*), $(-> $ret)?, $id)
        }
    };

    ( $vis:vis fn $name:ident(&self $(, $arg_name:ident: $arg_ty:ty)* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        #[inline]
        $vis fn $name(&self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::relocation_func!(@body (*const Self $(, $arg_ty)*), (self as *const Self $(, $arg_name)*), $(-> $ret)?, $id)
        }
    };

    ( @no_inline $vis:vis fn $name:ident(&mut self $(, $arg_name:ident: $arg_ty:ty)* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        $vis fn $name(&mut self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::relocation_func!(@body (*mut Self $(, $arg_ty)*), (self as *mut Self $(, $arg_name)*), $(-> $ret)?, $id)
        }
    };

    ( $vis:vis fn $name:ident(&mut self $(, $arg_name:ident: $arg_ty:ty)* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        #[inline]
        $vis fn $name(&mut self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::relocation_func!(@body (*mut Self $(, $arg_ty)*), (self as *mut Self $(, $arg_name)*), $(-> $ret)?, $id)
        }
    };

    (@body ($($arg_types:ty),*), ($($arg_names:expr),*), $(-> $ret:ty)?, $id:expr) => {{
        let func = unsafe {
            $crate::relocation::Relocation::<unsafe extern "C-unwind" fn($($arg_types),*) $(-> $ret)?>
                ::new($id)
                .get()
        };
        unsafe { func($($arg_names),*) }
    }};
}
