use core::sync::atomic::Ordering;

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_BSPathingRequest;
use crate::offsets::offsets_vtable::VTABLE_BSPathingRequest;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::{
    BSIntrusiveRefCounted, BSPathingActorAttributes, BSPathingAvoidNode, BSPathingGoal,
    BSPathingRestrictions, BSPathingSearchParameters, BSPathingStart, BSPathingStreamRead,
    BSPathingStreamWrite, BSTArray, BSTTuple, IDebugText, IMovementParameters,
    MovementActorAvoidanceParameters,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSPathingRequest::ArrayRefCounted`
pub type BSPathingRequestArrayRefCounted = BSTTuple<BSTArray<BSPathingAvoidNode>, u32>;

/// C++ `RE::BSPathingRequest`
#[repr(C)]
pub struct BSPathingRequest {
    pub vtable: *const usize,                         // 00
    pub base: BSIntrusiveRefCounted,                  // 08
    pub pad0c: u32,                                   // 0C
    pub start: BSPathingStart,                        // 10
    pub goal: BSPathingGoal,                          // 48
    pub restrictions: BSPathingRestrictions,          // 98
    pub search_parameters: BSPathingSearchParameters, // B0
    pub padbc: u32,                                   // BC
    pub actor_attributes: BSPathingActorAttributes,   // C0
    pub movement_actor_avoidance_parameters: MovementActorAvoidanceParameters, // D8
    // TODO: SOURCE - replace this raw pointer stand-in with
    // `BSTSmartPointer<IMovementParameters>` after the movement-parameters
    // intrusive smart-pointer contract is translated.
    pub default_parameters: *mut IMovementParameters, // F8
}

const _: () = assert!(core::mem::size_of::<BSPathingRequest>() == 0x100);
const _: () = assert!(core::mem::offset_of!(BSPathingRequest, base) == 0x08);
const _: () = assert!(core::mem::offset_of!(BSPathingRequest, start) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSPathingRequest, goal) == 0x48);
const _: () = assert!(core::mem::offset_of!(BSPathingRequest, restrictions) == 0x98);
const _: () = assert!(core::mem::offset_of!(BSPathingRequest, search_parameters) == 0xB0);
const _: () = assert!(core::mem::offset_of!(BSPathingRequest, actor_attributes) == 0xC0);
const _: () =
    assert!(core::mem::offset_of!(BSPathingRequest, movement_actor_avoidance_parameters) == 0xD8);
const _: () = assert!(core::mem::offset_of!(BSPathingRequest, default_parameters) == 0xF8);

impl RttiType for BSPathingRequest {
    const RTTI: VariantID = RTTI_BSPathingRequest;
}

inherit!(BSPathingRequest : BSIntrusiveRefCounted, base);

impl AsRef<BSPathingRequest> for BSPathingRequest {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSPathingRequest> for BSPathingRequest {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSTSmartPointerIntrusiveRefCountable for BSPathingRequest {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let func: extern "C" fn(*mut Self) =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0usize) };
        func(self as *const Self as *mut Self);
    }
}

impl BSPathingRequest {
    pub const RTTI: VariantID = RTTI_BSPathingRequest;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPathingRequest;

    crate::virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x01;
        pub fn get_type(&mut self) -> u32
    }

    crate::virtual_method! {
        pub const VFUNC_COPY_TO: usize = 0x02;
        pub fn copy_to(&mut self, dest: &mut BSPathingRequest)
    }

    crate::virtual_method! {
        pub const VFUNC_WRITE: usize = 0x03;
        pub fn write(&mut self, stream: *mut BSPathingStreamWrite)
    }

    crate::virtual_method! {
        pub const VFUNC_READ: usize = 0x04;
        pub fn read(&mut self, stream: *mut BSPathingStreamRead)
    }

    crate::virtual_method! {
        pub const VFUNC_CHECK_VALID: usize = 0x05;
        pub fn check_valid(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_PRINT_DEBUG_TEXT: usize = 0x06;
        pub fn print_debug_text(&mut self, debug_text: *mut IDebugText)
    }
}
