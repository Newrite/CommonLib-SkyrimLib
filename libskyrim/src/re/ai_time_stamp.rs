/// C++ `RE::AITimeStamp`
#[repr(C)]
pub struct AITimeStamp {
    pub time_stamp: f32, // 00
}

const _: () = assert!(core::mem::size_of::<AITimeStamp>() == 0x4);
const _: () = assert!(core::mem::offset_of!(AITimeStamp, time_stamp) == 0x00);
