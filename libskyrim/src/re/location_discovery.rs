#![allow(non_snake_case)]

use crate::re::{BSTEventSource, MapMarkerData};
use crate::relocation::RelocationID;

pub mod LocationDiscovery {
    use super::*;

    /// C++ `RE::LocationDiscovery::Event`
    #[repr(C)]
    pub struct Event {
        pub map_marker_data: *mut MapMarkerData,     // 00
        pub worldspace_id: *const core::ffi::c_char, // 08
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x10);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(40056, 41067)
    }
}
