#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{GFxResource, GRefCountBase, GStatGroups};

/// C++ `RE::GFxResourceLibBase`
#[repr(C)]
pub struct GFxResourceLibBase {
    pub base: GRefCountBase<GFxResourceLibBase, { GStatGroups::kGStat_Default_Mem as u32 }>, // 00
}

const _: () = assert!(core::mem::size_of::<GFxResourceLibBase>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GFxResourceLibBase, base) == 0x0);

inherit!(GFxResourceLibBase : GRefCountBase<GFxResourceLibBase, { GStatGroups::kGStat_Default_Mem as u32 }>, base);

impl GFxResourceLibBase {
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_REMOVE_RESOURCE_ON_RELEASE: usize = 0x01;
        pub fn remove_resource_on_release(resource: *mut GFxResource)
    }

    crate::virtual_method! {
        pub const VFUNC_PIN_RESOURCE: usize = 0x02;
        pub fn pin_resource(resource: *mut GFxResource)
    }

    crate::virtual_method! {
        pub const VFUNC_UNPIN_RESOURCE: usize = 0x03;
        pub fn unpin_resource(resource: *mut GFxResource)
    }
}

impl AsRef<GFxResourceLibBase> for GFxResourceLibBase {
    #[inline(always)]
    fn as_ref(&self) -> &GFxResourceLibBase {
        self
    }
}

impl AsMut<GFxResourceLibBase> for GFxResourceLibBase {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut GFxResourceLibBase {
        self
    }
}

pub trait GFxResourceLibBaseExt: AsRef<GFxResourceLibBase> + AsMut<GFxResourceLibBase> {
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn remove_resource_on_release(&mut self, resource: *mut GFxResource) {
        self.as_mut().remove_resource_on_release(resource)
    }

    #[inline(always)]
    fn pin_resource(&mut self, resource: *mut GFxResource) {
        self.as_mut().pin_resource(resource)
    }

    #[inline(always)]
    fn unpin_resource(&mut self, resource: *mut GFxResource) {
        self.as_mut().unpin_resource(resource)
    }
}

impl<T> GFxResourceLibBaseExt for T where T: AsRef<GFxResourceLibBase> + AsMut<GFxResourceLibBase> {}
