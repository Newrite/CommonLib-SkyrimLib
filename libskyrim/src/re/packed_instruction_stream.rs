#[repr(C)]
pub struct PackedInstructionStream {
    pub num_instruction_bits: u32,            // 00
    pub jump_target_bit_count: u16,           // 04
    pub local_variable_bit_count: i8,         // 06
    pub member_variable_bit_count: i8,        // 07
    pub instructions: *mut core::ffi::c_void, // 08
}

const _: () = assert!(core::mem::size_of::<PackedInstructionStream>() == 0x10);

impl Default for PackedInstructionStream {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl PackedInstructionStream {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            num_instruction_bits: 0,
            jump_target_bit_count: 0,
            local_variable_bit_count: 0,
            member_variable_bit_count: 0,
            instructions: core::ptr::null_mut(),
        }
    }
}
