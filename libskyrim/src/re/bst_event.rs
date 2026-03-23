use core::marker::PhantomData;

/// C++ `RE::BSEventNotifyControl`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BSEventNotifyControl {
    Continue = 0,
    Stop = 1,
}

/// C++ `RE::BSTEventSink<T>`
#[repr(C)]
pub struct BSTEventSink<T> {
    pub vtable: *const usize, // 00
    pub _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<BSTEventSink<()>>() == 0x8);

/// C++ `RE::BSTEventSource<T>`
///
/// Opaque layout placeholder used for inheritance/ABI slots.
#[repr(C)]
pub struct BSTEventSource<T> {
    pub raw: [u8; 0x58], // 00
    pub _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<BSTEventSource<()>>() == 0x58);
