/// C++ `RE::BSPathingSearchParameters`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSPathingSearchParameters {
    pub preferred_cost_factor: f32,    // 00
    pub tangent_smoothing_factor: f32, // 04
    pub retry_count: u16,              // 08
    pub flags: u16,                    // 0A
}

const _: () = assert!(core::mem::size_of::<BSPathingSearchParameters>() == 0x0C);
const _: () =
    assert!(core::mem::offset_of!(BSPathingSearchParameters, preferred_cost_factor) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(BSPathingSearchParameters, tangent_smoothing_factor) == 0x04);
const _: () = assert!(core::mem::offset_of!(BSPathingSearchParameters, retry_count) == 0x08);
const _: () = assert!(core::mem::offset_of!(BSPathingSearchParameters, flags) == 0x0A);
