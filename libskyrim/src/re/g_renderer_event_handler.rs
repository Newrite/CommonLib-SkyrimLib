#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{GListNode, GRenderer};

/// C++ `RE::GRendererEventHandler::EventType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererEventHandlerEventType {
    kEndFrame = 0,
    kRendererReleased = 1,
}

/// C++ `RE::GRendererEventHandler`
#[repr(C)]
pub struct GRendererEventHandler {
    pub base: GListNode<GRendererEventHandler>, // 00
    pub vtable: *const usize,                   // 10
    pub renderer: *mut GRenderer,               // 18
}

const _: () = assert!(core::mem::size_of::<GRendererEventHandler>() == 0x20);
const _: () = assert!(core::mem::offset_of!(GRendererEventHandler, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GRendererEventHandler, vtable) == 0x10);
const _: () = assert!(core::mem::offset_of!(GRendererEventHandler, renderer) == 0x18);

inherit!(GRendererEventHandler : GListNode<GRendererEventHandler>, base);

impl GRendererEventHandler {
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x0;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_ON_EVENT: usize = 0x1;
        pub fn on_event(renderer: *mut GRenderer, change_type: GRendererEventHandlerEventType)
    }
}
