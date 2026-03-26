use crate::re::{BSInputDevice, INPUT_DEVICE};
use crate::relocation::{RttiType, VariantID};
use crate::rex::openvr::vr;

/// C++ `RE::ControllerDeviceHand`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerDeviceHand {
    kLeft = 0,
    kRight = 1,
    kNeither = 3,
}

const _: () = assert!(core::mem::size_of::<ControllerDeviceHand>() == 0x4);

/// C++ `RE::BSTrackedControllerDevice`
#[repr(C)]
pub struct BSTrackedControllerDevice {
    pub base: BSInputDevice,                            // 00
    pub unk70: u64,                                     // 70
    pub hand: ControllerDeviceHand,                     // 78
    pub tracked_device_index: vr::TrackedDeviceIndex_t, // 7C
}

const _: () = assert!(core::mem::size_of::<BSTrackedControllerDevice>() == 0x80);
const _: () = assert!(core::mem::offset_of!(BSTrackedControllerDevice, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSTrackedControllerDevice, unk70) == 0x70);
const _: () = assert!(core::mem::offset_of!(BSTrackedControllerDevice, hand) == 0x78);
const _: () =
    assert!(core::mem::offset_of!(BSTrackedControllerDevice, tracked_device_index) == 0x7C);

impl RttiType for BSTrackedControllerDevice {
    const RTTI: VariantID = VariantID::new(0, 0, 0x1F5FF58);
}

core_util::inherit!(BSTrackedControllerDevice : BSInputDevice, base);

impl BSTrackedControllerDevice {
    pub const RTTI: VariantID = VariantID::new(0, 0, 0x1F5FF58);
    pub const VTABLE: &'static [VariantID] = &[VariantID::new(0, 0, 0x17E8F78)];

    #[inline(always)]
    pub fn hand(&self) -> ControllerDeviceHand {
        self.hand
    }

    #[inline(always)]
    pub fn tracked_device_index(&self) -> vr::TrackedDeviceIndex_t {
        self.tracked_device_index
    }

    #[inline(always)]
    pub fn is_left_hand(&self) -> bool {
        self.hand == ControllerDeviceHand::kLeft
    }

    #[inline(always)]
    pub fn is_right_hand(&self) -> bool {
        self.hand == ControllerDeviceHand::kRight
    }

    #[inline(always)]
    pub fn is_neither_hand(&self) -> bool {
        self.hand == ControllerDeviceHand::kNeither
    }

    #[inline(always)]
    pub fn is_primary_controller_device(device: INPUT_DEVICE) -> bool {
        device == INPUT_DEVICE::kVivePrimary
            || device == INPUT_DEVICE::kOculusPrimary
            || device == INPUT_DEVICE::kWMRPrimary
    }

    #[inline(always)]
    pub fn is_secondary_controller_device(device: INPUT_DEVICE) -> bool {
        device == INPUT_DEVICE::kViveSecondary
            || device == INPUT_DEVICE::kOculusSecondary
            || device == INPUT_DEVICE::kWMRSecondary
    }
}

pub trait BSTrackedControllerDeviceExt {
    fn hand(&self) -> ControllerDeviceHand;
    fn tracked_device_index(&self) -> vr::TrackedDeviceIndex_t;
    fn is_left_hand(&self) -> bool;
    fn is_right_hand(&self) -> bool;
    fn is_neither_hand(&self) -> bool;
    fn is_primary_controller_device(device: INPUT_DEVICE) -> bool
    where
        Self: Sized;
    fn is_secondary_controller_device(device: INPUT_DEVICE) -> bool
    where
        Self: Sized;
}

impl<T> BSTrackedControllerDeviceExt for T
where
    T: AsRef<BSTrackedControllerDevice>,
{
    #[inline(always)]
    fn hand(&self) -> ControllerDeviceHand {
        BSTrackedControllerDevice::hand(self.as_ref())
    }

    #[inline(always)]
    fn tracked_device_index(&self) -> vr::TrackedDeviceIndex_t {
        BSTrackedControllerDevice::tracked_device_index(self.as_ref())
    }

    #[inline(always)]
    fn is_left_hand(&self) -> bool {
        BSTrackedControllerDevice::is_left_hand(self.as_ref())
    }

    #[inline(always)]
    fn is_right_hand(&self) -> bool {
        BSTrackedControllerDevice::is_right_hand(self.as_ref())
    }

    #[inline(always)]
    fn is_neither_hand(&self) -> bool {
        BSTrackedControllerDevice::is_neither_hand(self.as_ref())
    }

    #[inline(always)]
    fn is_primary_controller_device(device: INPUT_DEVICE) -> bool
    where
        Self: Sized,
    {
        BSTrackedControllerDevice::is_primary_controller_device(device)
    }

    #[inline(always)]
    fn is_secondary_controller_device(device: INPUT_DEVICE) -> bool
    where
        Self: Sized,
    {
        BSTrackedControllerDevice::is_secondary_controller_device(device)
    }
}
