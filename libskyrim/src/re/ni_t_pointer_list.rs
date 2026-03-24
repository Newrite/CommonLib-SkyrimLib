use crate::re::ni_t_list::NiTList;

/// C++ `RE::NiTPointerList<T>`
pub type NiTPointerList<T> = NiTList<*mut T>;

const _: () = assert!(core::mem::size_of::<NiTPointerList<u8>>() == 0x18);
