use crate::offsets::offsets_rtti::RTTI_IMovementControllerRegisterInterface;
use crate::offsets::offsets_vtable::VTABLE_IMovementControllerRegisterInterface;
use crate::re::{BSFixedString, IMovementInterface};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::IMovementControllerRegisterInterface`
#[repr(C)]
pub struct IMovementControllerRegisterInterface {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<IMovementControllerRegisterInterface>() == 0x8);
const _: () = assert!(core::mem::offset_of!(IMovementControllerRegisterInterface, vtable) == 0x00);

impl RttiType for IMovementControllerRegisterInterface {
    const RTTI: VariantID = RTTI_IMovementControllerRegisterInterface;
}

impl AsRef<IMovementControllerRegisterInterface> for IMovementControllerRegisterInterface {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<IMovementControllerRegisterInterface> for IMovementControllerRegisterInterface {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl IMovementControllerRegisterInterface {
    pub const RTTI: VariantID = RTTI_IMovementControllerRegisterInterface;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMovementControllerRegisterInterface;

    virtual_method! {
        pub const VFUNC_DESTRUCTOR: usize = 0x00;
        pub fn destructor(&mut self)
    }

    virtual_method! {
        pub const VFUNC_REGISTER_INTERFACE: usize = 0x01;
        pub fn register_interface(&mut self, name: *mut BSFixedString, interface: *mut IMovementInterface) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_INTERFACE_1: usize = 0x02;
        pub fn get_interface_1(&mut self, name: &BSFixedString) -> *mut IMovementInterface
    }

    virtual_method! {
        pub const VFUNC_GET_INTERFACE_2: usize = 0x03;
        pub fn get_interface_2(&mut self, name: &BSFixedString) -> *mut IMovementInterface
    }

    virtual_method! {
        pub const VFUNC_UNK_04: usize = 0x04;
        pub fn unk_04(&mut self)
    }
}

pub trait IMovementControllerRegisterInterfaceExt {
    fn destructor(&mut self);
    fn register_interface(
        &mut self,
        name: *mut BSFixedString,
        interface: *mut IMovementInterface,
    ) -> bool;
    fn get_interface_1(&mut self, name: &BSFixedString) -> *mut IMovementInterface;
    fn get_interface_2(&mut self, name: &BSFixedString) -> *mut IMovementInterface;
    fn unk_04(&mut self);
}

impl<T: AsMut<IMovementControllerRegisterInterface>> IMovementControllerRegisterInterfaceExt for T {
    #[inline(always)]
    fn destructor(&mut self) {
        IMovementControllerRegisterInterface::destructor(self.as_mut())
    }

    #[inline(always)]
    fn register_interface(
        &mut self,
        name: *mut BSFixedString,
        interface: *mut IMovementInterface,
    ) -> bool {
        IMovementControllerRegisterInterface::register_interface(self.as_mut(), name, interface)
    }

    #[inline(always)]
    fn get_interface_1(&mut self, name: &BSFixedString) -> *mut IMovementInterface {
        IMovementControllerRegisterInterface::get_interface_1(self.as_mut(), name)
    }

    #[inline(always)]
    fn get_interface_2(&mut self, name: &BSFixedString) -> *mut IMovementInterface {
        IMovementControllerRegisterInterface::get_interface_2(self.as_mut(), name)
    }

    #[inline(always)]
    fn unk_04(&mut self) {
        IMovementControllerRegisterInterface::unk_04(self.as_mut())
    }
}
