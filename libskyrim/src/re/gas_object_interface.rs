#![allow(non_camel_case_types)]

use crate::re::{GASObject, GASStringContext};

/// C++ `RE::GASObjectInterface`
#[repr(C)]
pub struct GASObjectInterface {
    pub vtable: *const usize,  // 00
    pub unk08: u64,            // 08
    pub proto: *mut GASObject, // 10
}

const _: () = assert!(core::mem::size_of::<GASObjectInterface>() == 0x18);
const _: () = assert!(core::mem::offset_of!(GASObjectInterface, vtable) == 0x0);
const _: () = assert!(core::mem::offset_of!(GASObjectInterface, unk08) == 0x8);
const _: () = assert!(core::mem::offset_of!(GASObjectInterface, proto) == 0x10);

impl GASObjectInterface {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_01: usize = 0x01; pub fn gas_object_interface_01() }
    crate::virtual_method! { pub const VFUNC_02: usize = 0x02; pub fn gas_object_interface_02() }
    crate::virtual_method! { pub const VFUNC_03: usize = 0x03; pub fn gas_object_interface_03() }
    crate::virtual_method! { pub const VFUNC_04: usize = 0x04; pub fn gas_object_interface_04() }
    crate::virtual_method! { pub const VFUNC_05: usize = 0x05; pub fn gas_object_interface_05() }
    crate::virtual_method! { pub const VFUNC_06: usize = 0x06; pub fn gas_object_interface_06() }
    crate::virtual_method! { pub const VFUNC_07: usize = 0x07; pub fn gas_object_interface_07() }
    crate::virtual_method! { pub const VFUNC_08: usize = 0x08; pub fn gas_object_interface_08() }
    crate::virtual_method! { pub const VFUNC_09: usize = 0x09; pub fn gas_object_interface_09() }
    crate::virtual_method! { pub const VFUNC_0A: usize = 0x0A; pub fn gas_object_interface_0a() }
    crate::virtual_method! { pub const VFUNC_0B: usize = 0x0B; pub fn gas_object_interface_0b() }
    crate::virtual_method! { pub const VFUNC_0C: usize = 0x0C; pub fn gas_object_interface_0c() }

    crate::virtual_method! {
        pub const VFUNC_SET_PROTO: usize = 0x0D;
        pub fn set_proto(string_context: *mut GASStringContext, object: *mut GASObject)
    }

    crate::virtual_method! { pub const VFUNC_0E: usize = 0x0E; pub fn gas_object_interface_0e() }
    crate::virtual_method! { pub const VFUNC_0F: usize = 0x0F; pub fn gas_object_interface_0f() }
    crate::virtual_method! { pub const VFUNC_10: usize = 0x10; pub fn gas_object_interface_10() }
    crate::virtual_method! { pub const VFUNC_11: usize = 0x11; pub fn gas_object_interface_11() }
    crate::virtual_method! { pub const VFUNC_12: usize = 0x12; pub fn gas_object_interface_12() }
    crate::virtual_method! { pub const VFUNC_13: usize = 0x13; pub fn gas_object_interface_13() }
    crate::virtual_method! { pub const VFUNC_14: usize = 0x14; pub fn gas_object_interface_14() }
}

impl AsRef<GASObjectInterface> for GASObjectInterface {
    #[inline(always)]
    fn as_ref(&self) -> &GASObjectInterface {
        self
    }
}

impl AsMut<GASObjectInterface> for GASObjectInterface {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut GASObjectInterface {
        self
    }
}

pub trait GASObjectInterfaceExt: AsRef<GASObjectInterface> + AsMut<GASObjectInterface> {
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn gas_object_interface_01(&mut self) {
        self.as_mut().gas_object_interface_01()
    }

    #[inline(always)]
    fn gas_object_interface_02(&mut self) {
        self.as_mut().gas_object_interface_02()
    }

    #[inline(always)]
    fn gas_object_interface_03(&mut self) {
        self.as_mut().gas_object_interface_03()
    }

    #[inline(always)]
    fn gas_object_interface_04(&mut self) {
        self.as_mut().gas_object_interface_04()
    }

    #[inline(always)]
    fn gas_object_interface_05(&mut self) {
        self.as_mut().gas_object_interface_05()
    }

    #[inline(always)]
    fn gas_object_interface_06(&mut self) {
        self.as_mut().gas_object_interface_06()
    }

    #[inline(always)]
    fn gas_object_interface_07(&mut self) {
        self.as_mut().gas_object_interface_07()
    }

    #[inline(always)]
    fn gas_object_interface_08(&mut self) {
        self.as_mut().gas_object_interface_08()
    }

    #[inline(always)]
    fn gas_object_interface_09(&mut self) {
        self.as_mut().gas_object_interface_09()
    }

    #[inline(always)]
    fn gas_object_interface_0a(&mut self) {
        self.as_mut().gas_object_interface_0a()
    }

    #[inline(always)]
    fn gas_object_interface_0b(&mut self) {
        self.as_mut().gas_object_interface_0b()
    }

    #[inline(always)]
    fn gas_object_interface_0c(&mut self) {
        self.as_mut().gas_object_interface_0c()
    }

    #[inline(always)]
    fn set_proto(&mut self, string_context: *mut GASStringContext, object: *mut GASObject) {
        self.as_mut().set_proto(string_context, object)
    }

    #[inline(always)]
    fn gas_object_interface_0e(&mut self) {
        self.as_mut().gas_object_interface_0e()
    }

    #[inline(always)]
    fn gas_object_interface_0f(&mut self) {
        self.as_mut().gas_object_interface_0f()
    }

    #[inline(always)]
    fn gas_object_interface_10(&mut self) {
        self.as_mut().gas_object_interface_10()
    }

    #[inline(always)]
    fn gas_object_interface_11(&mut self) {
        self.as_mut().gas_object_interface_11()
    }

    #[inline(always)]
    fn gas_object_interface_12(&mut self) {
        self.as_mut().gas_object_interface_12()
    }

    #[inline(always)]
    fn gas_object_interface_13(&mut self) {
        self.as_mut().gas_object_interface_13()
    }

    #[inline(always)]
    fn gas_object_interface_14(&mut self) {
        self.as_mut().gas_object_interface_14()
    }
}

impl<T> GASObjectInterfaceExt for T where T: AsRef<GASObjectInterface> + AsMut<GASObjectInterface> {}
