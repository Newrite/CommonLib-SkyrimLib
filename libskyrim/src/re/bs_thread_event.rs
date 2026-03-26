use crate::relocation::RelocationID;

/// C++ `RE::BSThreadEvent`
#[repr(C)]
pub struct BSThreadEvent {
    pub _data: u8,
}

const _: () = assert!(core::mem::size_of::<BSThreadEvent>() == 0x1);

impl BSThreadEvent {
    crate::relocation_func! {
        pub fn init_sdm() => RelocationID::new(67151, 68449)
    }
}
