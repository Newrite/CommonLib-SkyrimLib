/// C++ `RE::TESFastTravelEndEvent`
#[repr(C)]
pub struct TESFastTravelEndEvent {
    pub fast_travel_end_hours: f32, // 00
    pub pad04: u32,                 // 04
}

const _: () = assert!(core::mem::size_of::<TESFastTravelEndEvent>() == 0x08);
