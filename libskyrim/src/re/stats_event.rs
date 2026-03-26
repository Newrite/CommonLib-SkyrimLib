/// C++ `RE::BSScript::StatsEvent`
#[repr(C)]
pub struct StatsEvent {
    pub running_stacks_count: u32,   // 00
    pub suspended_stacks_count: u32, // 04
    pub function_msg_count: u32,     // 08
    pub detached_object_count: u32,  // 0C
}

const _: () = assert!(core::mem::size_of::<StatsEvent>() == 0x10);
const _: () = assert!(core::mem::offset_of!(StatsEvent, running_stacks_count) == 0x00);
const _: () = assert!(core::mem::offset_of!(StatsEvent, suspended_stacks_count) == 0x04);
const _: () = assert!(core::mem::offset_of!(StatsEvent, function_msg_count) == 0x08);
const _: () = assert!(core::mem::offset_of!(StatsEvent, detached_object_count) == 0x0C);
