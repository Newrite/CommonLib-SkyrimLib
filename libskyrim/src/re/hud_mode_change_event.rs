/// VR-only C++ `RE::HudModeChangeEvent`
#[repr(C)]
pub struct HudModeChangeEvent {
    pub _data: u8,
}

const _: () = assert!(core::mem::size_of::<HudModeChangeEvent>() == 0x1);
