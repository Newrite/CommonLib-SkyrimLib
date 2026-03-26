/// C++ `RE::TESSleepStopEvent`
#[repr(C)]
pub struct TESSleepStopEvent {
    pub interrupted: bool, // 00
}

const _: () = assert!(core::mem::size_of::<TESSleepStopEvent>() == 0x01);
