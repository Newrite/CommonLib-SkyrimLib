/// C++ `RE::TESWaitStopEvent`
#[repr(C)]
pub struct TESWaitStopEvent {
    pub interrupted: bool, // 00
}

const _: () = assert!(core::mem::size_of::<TESWaitStopEvent>() == 0x01);
