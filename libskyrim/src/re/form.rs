use crate::re::bs_core_types::FormID;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Form {
    pub record_type: u32,       // 00 
    pub length: u32,            // 04
    pub flags: u32,             // 08
    pub form_id: FormID,        // 0C
    pub version_control: u32,   // 10
    pub form_version: u16,      // 14
    pub vc_version: u16,        // 16
}

// Проверка размера: 0x18
const _: () = assert!(core::mem::size_of::<Form>() == 0x18);

impl Form {
    #[inline]
    pub const fn get_type_bytes(&self) -> [u8; 4] {
        self.record_type.to_le_bytes()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FormGroup {
    pub group_data: Form,       // 00
    pub group_offset: u64,      // 18
}

// Проверка размера: 0x20
const _: () = assert!(core::mem::size_of::<FormGroup>() == 0x20);