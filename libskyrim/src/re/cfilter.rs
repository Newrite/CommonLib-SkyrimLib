use crate::re::collision_layers::ColLayer;

/// Source-backed subset of `RE::CFilter`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CFilter {
    pub filter: u32, // 00
}

const _: () = assert!(core::mem::size_of::<CFilter>() == 0x4);
const _: () = assert!(core::mem::offset_of!(CFilter, filter) == 0x0);

impl CFilter {
    #[inline(always)]
    pub fn get_collision_layer(self) -> ColLayer {
        unsafe { core::mem::transmute((self.filter & 0x7F) as i32) }
    }
}
