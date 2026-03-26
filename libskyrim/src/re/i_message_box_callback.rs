use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_IMessageBoxCallback;
use crate::offsets::offsets_vtable::VTABLE_IMessageBoxCallback;
use crate::re::bs_intrusive_ref_counted::BSIntrusiveRefCounted;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::IMessageBoxCallback::Message`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IMessageBoxCallbackMessage {
    kUnk0 = 0,
    kUnk1 = 1,
    kUnk2 = 2,
}

const _: () = assert!(core::mem::size_of::<IMessageBoxCallbackMessage>() == 0x4);

/// C++ `RE::IMessageBoxCallback`
#[repr(C)]
pub struct IMessageBoxCallback {
    pub vtable: *const usize,        // 00
    pub base: BSIntrusiveRefCounted, // 08
    pub unk0c: u32,                  // 0C
}

const _: () = assert!(core::mem::size_of::<IMessageBoxCallback>() == 0x10);
const _: () = assert!(core::mem::offset_of!(IMessageBoxCallback, vtable) == 0x00);
const _: () = assert!(core::mem::offset_of!(IMessageBoxCallback, base) == 0x08);
const _: () = assert!(core::mem::offset_of!(IMessageBoxCallback, unk0c) == 0x0C);

inherit!(IMessageBoxCallback : BSIntrusiveRefCounted, base);

impl RttiType for IMessageBoxCallback {
    const RTTI: VariantID = RTTI_IMessageBoxCallback;
}

impl BSTSmartPointerIntrusiveRefCountable for IMessageBoxCallback {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        self.dtor();
    }
}

impl IMessageBoxCallback {
    pub const RTTI: VariantID = RTTI_IMessageBoxCallback;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMessageBoxCallback;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_RUN: usize = 0x01;
        pub fn run(message: IMessageBoxCallbackMessage)
    }
}

pub trait IMessageBoxCallbackExt {
    fn dtor(&mut self);
    fn run(&mut self, message: IMessageBoxCallbackMessage);
}

impl<T> IMessageBoxCallbackExt for T
where
    T: AsRef<IMessageBoxCallback> + AsMut<IMessageBoxCallback>,
{
    #[inline(always)]
    fn dtor(&mut self) {
        IMessageBoxCallback::dtor(self.as_mut())
    }

    #[inline(always)]
    fn run(&mut self, message: IMessageBoxCallbackMessage) {
        IMessageBoxCallback::run(self.as_mut(), message)
    }
}
