use core::sync::atomic::Ordering;

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_MovementControllerNPC;
use crate::offsets::offsets_vtable::VTABLE_MovementControllerNPC;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::{
    Actor, BSSpinLock, BSTArray, IAnimationSetCallbackFunctor, IMovementDirectControl,
    IMovementMessageInterface, IMovementMotionDrivenControl, IMovementPlannerDirectControl,
    IMovementSelectIdle, MovementControllerAI,
};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type MovementMessage;
}

/// C++ `RE::MovementControllerNPC`
#[repr(C)]
pub struct MovementControllerNPC {
    pub base: MovementControllerAI,                            // 000
    pub movement_message_interface: IMovementMessageInterface, // 120
    pub movement_motion_driven_control: IMovementMotionDrivenControl, // 128
    pub movement_select_idle: IMovementSelectIdle,             // 130
    pub movement_direct_control: IMovementDirectControl,       // 138
    pub movement_planner_direct_control: IMovementPlannerDirectControl, // 140
    pub animation_set_callback_functor: IAnimationSetCallbackFunctor, // 148
    pub unk150: BSSpinLock,                                    // 150
    pub movement_messages: BSTArray<*mut MovementMessage>,     // 158
    pub unk170: BSTArray<*mut core::ffi::c_void>,              // 170
    pub unk188: BSTArray<*mut core::ffi::c_void>,              // 188
    pub unk1a0: BSSpinLock,                                    // 1A0
    pub unk1a8: u64,                                           // 1A8
    pub unk1b0: u64,                                           // 1B0
    pub actor: *mut Actor,                                     // 1B8
    pub unk1c0: u32,                                           // 1C0
    pub unk1c4: bool,                                          // 1C4
    pub controls_driven: bool,                                 // 1C5
    pub unk1c6: bool,                                          // 1C6
    pub unk1c7: bool,                                          // 1C7
    pub unk1c8: bool,                                          // 1C8
    pub unk1c9: bool,                                          // 1C9
    pub unk1ca: bool,                                          // 1CA
    pub unk1cb: bool,                                          // 1CB
    pub unk1cc: u32,                                           // 1CC
}

// TODO: CommonLib only proves the flat-runtime `sizeof(MovementControllerNPC) == 0x1D0`.
// Verify VR size and secondary-base offsets before relying on deep field access there.
const _: () = assert!(core::mem::size_of::<MovementControllerNPC>() == 0x1D0);
const _: () = assert!(core::mem::offset_of!(MovementControllerNPC, base) == 0x000);
const _: () =
    assert!(core::mem::offset_of!(MovementControllerNPC, movement_message_interface) == 0x120);
const _: () =
    assert!(core::mem::offset_of!(MovementControllerNPC, movement_motion_driven_control) == 0x128);
const _: () = assert!(core::mem::offset_of!(MovementControllerNPC, movement_select_idle) == 0x130);
const _: () =
    assert!(core::mem::offset_of!(MovementControllerNPC, movement_direct_control) == 0x138);
const _: () =
    assert!(core::mem::offset_of!(MovementControllerNPC, movement_planner_direct_control) == 0x140);
const _: () =
    assert!(core::mem::offset_of!(MovementControllerNPC, animation_set_callback_functor) == 0x148);
const _: () = assert!(core::mem::offset_of!(MovementControllerNPC, actor) == 0x1B8);
const _: () = assert!(core::mem::offset_of!(MovementControllerNPC, controls_driven) == 0x1C5);

impl RttiType for MovementControllerNPC {
    const RTTI: VariantID = RTTI_MovementControllerNPC;
}

inherit!(MovementControllerNPC : MovementControllerAI, base);

impl AsRef<IMovementMessageInterface> for MovementControllerNPC {
    #[inline(always)]
    fn as_ref(&self) -> &IMovementMessageInterface {
        &self.movement_message_interface
    }
}

impl AsMut<IMovementMessageInterface> for MovementControllerNPC {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut IMovementMessageInterface {
        &mut self.movement_message_interface
    }
}

impl AsRef<IMovementMotionDrivenControl> for MovementControllerNPC {
    #[inline(always)]
    fn as_ref(&self) -> &IMovementMotionDrivenControl {
        &self.movement_motion_driven_control
    }
}

impl AsMut<IMovementMotionDrivenControl> for MovementControllerNPC {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut IMovementMotionDrivenControl {
        &mut self.movement_motion_driven_control
    }
}

impl AsRef<IMovementSelectIdle> for MovementControllerNPC {
    #[inline(always)]
    fn as_ref(&self) -> &IMovementSelectIdle {
        &self.movement_select_idle
    }
}

impl AsMut<IMovementSelectIdle> for MovementControllerNPC {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut IMovementSelectIdle {
        &mut self.movement_select_idle
    }
}

impl AsRef<IMovementDirectControl> for MovementControllerNPC {
    #[inline(always)]
    fn as_ref(&self) -> &IMovementDirectControl {
        &self.movement_direct_control
    }
}

impl AsMut<IMovementDirectControl> for MovementControllerNPC {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut IMovementDirectControl {
        &mut self.movement_direct_control
    }
}

impl AsRef<IMovementPlannerDirectControl> for MovementControllerNPC {
    #[inline(always)]
    fn as_ref(&self) -> &IMovementPlannerDirectControl {
        &self.movement_planner_direct_control
    }
}

impl AsMut<IMovementPlannerDirectControl> for MovementControllerNPC {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut IMovementPlannerDirectControl {
        &mut self.movement_planner_direct_control
    }
}

impl AsRef<IAnimationSetCallbackFunctor> for MovementControllerNPC {
    #[inline(always)]
    fn as_ref(&self) -> &IAnimationSetCallbackFunctor {
        &self.animation_set_callback_functor
    }
}

impl AsMut<IAnimationSetCallbackFunctor> for MovementControllerNPC {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut IAnimationSetCallbackFunctor {
        &mut self.animation_set_callback_functor
    }
}

impl AsRef<MovementControllerNPC> for MovementControllerNPC {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<MovementControllerNPC> for MovementControllerNPC {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSTSmartPointerIntrusiveRefCountable for MovementControllerNPC {
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

impl MovementControllerNPC {
    pub const RTTI: VariantID = RTTI_MovementControllerNPC;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MovementControllerNPC;

    crate::virtual_method! { pub const VFUNC_UNK_0A: usize = 0x0A; pub fn unk_0a(&mut self) }
    crate::virtual_method! { pub const VFUNC_UNK_0B: usize = 0x0B; pub fn unk_0b(&mut self) }
    crate::virtual_method! { pub const VFUNC_SET_AI_DRIVEN: usize = 0x0C; pub fn set_ai_driven(&mut self) }
    crate::virtual_method! { pub const VFUNC_SET_CONTROLS_DRIVEN: usize = 0x0D; pub fn set_controls_driven(&mut self) }
    crate::virtual_method! { pub const VFUNC_GET_AI_DRIVEN: usize = 0x0E; pub fn get_ai_driven(&mut self) -> bool }
    crate::virtual_method! { pub const VFUNC_GET_CONTROLS_DRIVEN: usize = 0x0F; pub fn get_controls_driven(&mut self) -> bool }
    crate::virtual_method! { pub const VFUNC_UNK_10: usize = 0x10; pub fn unk_10(&mut self) }
    crate::virtual_method! { pub const VFUNC_UNK_11: usize = 0x11; pub fn unk_11(&mut self) }
    crate::virtual_method! { pub const VFUNC_UNK_12: usize = 0x12; pub fn unk_12(&mut self) }
    crate::virtual_method! { pub const VFUNC_UNK_13: usize = 0x13; pub fn unk_13(&mut self) }
    crate::virtual_method! { pub const VFUNC_UNK_14: usize = 0x14; pub fn unk_14(&mut self) }
}
