use crate::offsets::offsets_rtti::RTTI_BSAnimationGraphChannel;
use crate::offsets::offsets_vtable::VTABLE_BSAnimationGraphChannel;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::{BSFixedString, BSIntrusiveRefCounted};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSAnimationGraphChannel`
#[repr(C)]
pub struct BSAnimationGraphChannel {
    pub vtable: *const usize,                         // 00
    pub intrusive_ref_counted: BSIntrusiveRefCounted, // 08
    pub pad0c: u32,                                   // 0C
    pub channel_name: BSFixedString,                  // 10
    pub value: u32,                                   // 18
    pub pad1c: u32,                                   // 1C
}

const _: () = assert!(core::mem::size_of::<BSAnimationGraphChannel>() == 0x20);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphChannel, vtable) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(BSAnimationGraphChannel, intrusive_ref_counted) == 0x08);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphChannel, channel_name) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphChannel, value) == 0x18);

impl RttiType for BSAnimationGraphChannel {
    const RTTI: VariantID = RTTI_BSAnimationGraphChannel;
}

impl AsRef<BSAnimationGraphChannel> for BSAnimationGraphChannel {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSAnimationGraphChannel> for BSAnimationGraphChannel {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSTSmartPointerIntrusiveRefCountable for BSAnimationGraphChannel {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.intrusive_ref_counted.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.intrusive_ref_counted.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let func: extern "C" fn(*mut Self) =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0usize) };
        func(self as *const Self as *mut Self);
    }
}

impl BSAnimationGraphChannel {
    pub const RTTI: VariantID = RTTI_BSAnimationGraphChannel;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSAnimationGraphChannel;

    // ~BSAnimationGraphChannel() override;  // 00

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_POLL_CHANNEL_UPDATE_IMPL: usize = 0x01;
        pub fn poll_channel_update_impl(&mut self, arg1: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_RESET_IMPL: usize = 0x02;
        pub fn reset_impl(&mut self)
    }
}

pub trait BSAnimationGraphChannelExt {
    fn poll_channel_update_impl(&mut self, arg1: bool);
    fn reset_impl(&mut self);
}

impl<T: AsRef<BSAnimationGraphChannel> + AsMut<BSAnimationGraphChannel>> BSAnimationGraphChannelExt
    for T
{
    #[inline(always)]
    fn poll_channel_update_impl(&mut self, arg1: bool) {
        BSAnimationGraphChannel::poll_channel_update_impl(self.as_mut(), arg1)
    }

    #[inline(always)]
    fn reset_impl(&mut self) {
        BSAnimationGraphChannel::reset_impl(self.as_mut())
    }
}
