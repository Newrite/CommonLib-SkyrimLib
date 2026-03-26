use crate::offsets::offsets_rtti::RTTI_BSMouseDevice;
use crate::offsets::offsets_vtable::VTABLE_BSMouseDevice;
use crate::re::BSInputDevice;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSMouseDevice`
#[repr(C)]
pub struct BSMouseDevice {
    pub base: BSInputDevice,    // 00
    pub background_mouse: bool, // 70
}

const _: () = assert!(core::mem::size_of::<BSMouseDevice>() == 0x78);
const _: () = assert!(core::mem::offset_of!(BSMouseDevice, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSMouseDevice, background_mouse) == 0x70);

impl RttiType for BSMouseDevice {
    const RTTI: VariantID = RTTI_BSMouseDevice;
}

core_util::inherit!(BSMouseDevice : BSInputDevice, base);

impl BSMouseDevice {
    pub const RTTI: VariantID = RTTI_BSMouseDevice;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSMouseDevice;

    crate::virtual_method! {
        pub const VFUNC_REINITIALIZE: usize = 0x09;
        pub fn reinitialize(&mut self)
    }
}

pub trait BSMouseDeviceExt {
    fn reinitialize(&mut self);
}

impl<T> BSMouseDeviceExt for T
where
    T: AsRef<BSMouseDevice> + AsMut<BSMouseDevice>,
{
    #[inline(always)]
    fn reinitialize(&mut self) {
        BSMouseDevice::reinitialize(self.as_mut())
    }
}
