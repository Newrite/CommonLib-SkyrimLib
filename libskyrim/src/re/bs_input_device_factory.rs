use crate::re::{BSIInputDevice, INPUT_DEVICE};
use crate::relocation::RelocationID;

/// C++ `RE::BSInputDeviceFactory`
pub struct BSInputDeviceFactory;

impl BSInputDeviceFactory {
    crate::relocation_func! {
        pub fn create_input_device(device_type: INPUT_DEVICE) -> *mut BSIInputDevice => RelocationID::new(67431, 68738)
    }

    #[inline(always)]
    pub unsafe fn destroy_input_device(device: *mut BSIInputDevice) {
        unsafe { crate::ffi::commonlib_destroy_bsi_input_device(device.cast()) }
    }
}
