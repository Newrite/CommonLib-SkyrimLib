use core::marker::PhantomData;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::{
    RTTI_BSPathingStreamRead, RTTI_BSPathingStreamWrite, RTTI_IPipelineStageInterface,
    RTTI_MovementAgent, RTTI_MovementArbiter, RTTI_MovementControllerAI,
};
use crate::offsets::offsets_vtable::{
    VTABLE_BSPathingStreamRead, VTABLE_BSPathingStreamWrite, VTABLE_IPipelineStageInterface,
    VTABLE_MovementAgent, VTABLE_MovementArbiter, VTABLE_MovementControllerAI,
};
use crate::re::bst_smart_pointer::{BSTSmartPointer, BSTSmartPointerIntrusiveRefCountable};
use crate::re::{
    BSFixedString, BSTSmallArray, IMovementControllerRegisterInterface, IMovementState, NiPoint3,
};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

crate::core_util::abstract_type! {
    pub type BSPathingStreamRead;
    pub type BSPathingStreamWrite;
    pub type IMovementDebugRenderingInterface;
    pub type IPipelineStageInterface;
    pub type MovementUpdateDataSmallDelta;
    pub type MovementUpdateDataLargeDelta;
}

impl RttiType for BSPathingStreamRead {
    const RTTI: VariantID = RTTI_BSPathingStreamRead;
}

impl BSPathingStreamRead {
    pub const RTTI: VariantID = RTTI_BSPathingStreamRead;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPathingStreamRead;
}

impl RttiType for BSPathingStreamWrite {
    const RTTI: VariantID = RTTI_BSPathingStreamWrite;
}

impl BSPathingStreamWrite {
    pub const RTTI: VariantID = RTTI_BSPathingStreamWrite;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPathingStreamWrite;
}

impl RttiType for IPipelineStageInterface {
    const RTTI: VariantID = RTTI_IPipelineStageInterface;
}

impl IPipelineStageInterface {
    pub const RTTI: VariantID = RTTI_IPipelineStageInterface;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IPipelineStageInterface;
}

/// C++ `RE::MovementAgent`
#[repr(C)]
pub struct MovementAgent {
    pub vtable: *const usize,                // 00
    pub intrusive_ref_counted: [u32; 2],     // 08
    pub movement_state: *mut IMovementState, // 10
}

const _: () = assert!(core::mem::size_of::<MovementAgent>() == 0x18);
const _: () = assert!(core::mem::offset_of!(MovementAgent, intrusive_ref_counted) == 0x08);
const _: () = assert!(core::mem::offset_of!(MovementAgent, movement_state) == 0x10);

impl RttiType for MovementAgent {
    const RTTI: VariantID = RTTI_MovementAgent;
}

impl AsRef<MovementAgent> for MovementAgent {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<MovementAgent> for MovementAgent {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSTSmartPointerIntrusiveRefCountable for MovementAgent {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        let ref_count =
            unsafe { &*(&self.intrusive_ref_counted[0] as *const u32 as *const AtomicU32) };
        ref_count.fetch_add(1, Ordering::SeqCst);
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        let ref_count =
            unsafe { &*(&self.intrusive_ref_counted[0] as *const u32 as *const AtomicU32) };
        ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let func: extern "C" fn(*mut Self) =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0usize) };
        func(self as *const Self as *mut Self);
    }
}

impl MovementAgent {
    pub const RTTI: VariantID = RTTI_MovementAgent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MovementAgent;

    virtual_method! {
        pub const VFUNC_GET_AGENT_TYPE_PTR: usize = 0x01;
        fn get_agent_type_ptr(&self) -> *const BSFixedString
    }

    #[inline(always)]
    pub fn get_agent_type(&self) -> &BSFixedString {
        unsafe { &*self.get_agent_type_ptr() }
    }

    virtual_method! { pub const VFUNC_GET_AGENT_SAVE_TYPE: usize = 0x02; pub fn get_agent_save_type(&self) -> u8 }
    virtual_method! { pub const VFUNC_GET_PIPELINE_STAGE_INTERFACE: usize = 0x03; pub fn get_pipeline_stage_interface(&mut self, name: &BSFixedString) -> *mut IPipelineStageInterface }
    virtual_method! { pub const VFUNC_REGISTER_WITH_CONTROLLER: usize = 0x04; pub fn register_with_controller(&mut self, controller: &mut IMovementControllerRegisterInterface) }
    virtual_method! { pub const VFUNC_GET_DEBUG_RENDERING_INTERFACE: usize = 0x05; pub fn get_debug_rendering_interface(&mut self) -> *mut IMovementDebugRenderingInterface }
    virtual_method! { pub const VFUNC_ACTIVATE: usize = 0x06; pub fn activate(&mut self) -> bool }
    virtual_method! { pub const VFUNC_INIT: usize = 0x07; pub fn init(&mut self, stream: &mut BSPathingStreamRead) }
    virtual_method! { pub const VFUNC_KILL: usize = 0x08; pub fn kill(&mut self) }
    virtual_method! { pub const VFUNC_DEACTIVATE: usize = 0x09; pub fn deactivate(&mut self) }
    virtual_method! { pub const VFUNC_UNREGISTER_WITH_CONTROLLER: usize = 0x0A; pub fn unregister_with_controller(&mut self) }
    virtual_method! { pub const VFUNC_SAVE_GAME: usize = 0x0B; pub fn save_game(&mut self, stream: &mut BSPathingStreamWrite) }
    virtual_method! { pub const VFUNC_LOAD_GAME: usize = 0x0C; pub fn load_game(&mut self, stream: &mut BSPathingStreamRead) }
    virtual_method! { pub const VFUNC_FINISH_LOAD_GAME: usize = 0x0D; pub fn finish_load_game(&mut self) }
}

pub trait MovementAgentExt {
    fn get_agent_type(&self) -> &BSFixedString;
    fn get_agent_save_type(&self) -> u8;
    fn get_pipeline_stage_interface(
        &mut self,
        name: &BSFixedString,
    ) -> *mut IPipelineStageInterface;
    fn register_with_controller(&mut self, controller: &mut IMovementControllerRegisterInterface);
    fn get_debug_rendering_interface(&mut self) -> *mut IMovementDebugRenderingInterface;
    fn activate(&mut self) -> bool;
    fn init(&mut self, stream: &mut BSPathingStreamRead);
    fn kill(&mut self);
    fn deactivate(&mut self);
    fn unregister_with_controller(&mut self);
    fn save_game(&mut self, stream: &mut BSPathingStreamWrite);
    fn load_game(&mut self, stream: &mut BSPathingStreamRead);
    fn finish_load_game(&mut self);
}

impl<T: AsRef<MovementAgent> + AsMut<MovementAgent>> MovementAgentExt for T {
    #[inline(always)]
    fn get_agent_type(&self) -> &BSFixedString {
        MovementAgent::get_agent_type(self.as_ref())
    }

    #[inline(always)]
    fn get_agent_save_type(&self) -> u8 {
        MovementAgent::get_agent_save_type(self.as_ref())
    }

    #[inline(always)]
    fn get_pipeline_stage_interface(
        &mut self,
        name: &BSFixedString,
    ) -> *mut IPipelineStageInterface {
        MovementAgent::get_pipeline_stage_interface(self.as_mut(), name)
    }

    #[inline(always)]
    fn register_with_controller(&mut self, controller: &mut IMovementControllerRegisterInterface) {
        MovementAgent::register_with_controller(self.as_mut(), controller)
    }

    #[inline(always)]
    fn get_debug_rendering_interface(&mut self) -> *mut IMovementDebugRenderingInterface {
        MovementAgent::get_debug_rendering_interface(self.as_mut())
    }

    #[inline(always)]
    fn activate(&mut self) -> bool {
        MovementAgent::activate(self.as_mut())
    }

    #[inline(always)]
    fn init(&mut self, stream: &mut BSPathingStreamRead) {
        MovementAgent::init(self.as_mut(), stream)
    }

    #[inline(always)]
    fn kill(&mut self) {
        MovementAgent::kill(self.as_mut())
    }

    #[inline(always)]
    fn deactivate(&mut self) {
        MovementAgent::deactivate(self.as_mut())
    }

    #[inline(always)]
    fn unregister_with_controller(&mut self) {
        MovementAgent::unregister_with_controller(self.as_mut())
    }

    #[inline(always)]
    fn save_game(&mut self, stream: &mut BSPathingStreamWrite) {
        MovementAgent::save_game(self.as_mut(), stream)
    }

    #[inline(always)]
    fn load_game(&mut self, stream: &mut BSPathingStreamRead) {
        MovementAgent::load_game(self.as_mut(), stream)
    }

    #[inline(always)]
    fn finish_load_game(&mut self) {
        MovementAgent::finish_load_game(self.as_mut())
    }
}

/// C++ `RE::MovementArbiter`
#[repr(C)]
pub struct MovementArbiter {
    pub vtable: *const usize,                // 00
    pub intrusive_ref_counted: [u32; 2],     // 08
    pub movement_state: *mut IMovementState, // 10
}

const _: () = assert!(core::mem::size_of::<MovementArbiter>() == 0x18);
const _: () = assert!(core::mem::offset_of!(MovementArbiter, intrusive_ref_counted) == 0x08);
const _: () = assert!(core::mem::offset_of!(MovementArbiter, movement_state) == 0x10);

impl RttiType for MovementArbiter {
    const RTTI: VariantID = RTTI_MovementArbiter;
}

impl AsRef<MovementArbiter> for MovementArbiter {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<MovementArbiter> for MovementArbiter {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSTSmartPointerIntrusiveRefCountable for MovementArbiter {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        let ref_count =
            unsafe { &*(&self.intrusive_ref_counted[0] as *const u32 as *const AtomicU32) };
        ref_count.fetch_add(1, Ordering::SeqCst);
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        let ref_count =
            unsafe { &*(&self.intrusive_ref_counted[0] as *const u32 as *const AtomicU32) };
        ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let func: extern "C" fn(*mut Self) =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0usize) };
        func(self as *const Self as *mut Self);
    }
}

impl MovementArbiter {
    pub const RTTI: VariantID = RTTI_MovementArbiter;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MovementArbiter;

    virtual_method! {
        pub const VFUNC_GET_PIPELINE_STAGE_PTR: usize = 0x01;
        fn get_pipeline_stage_ptr(&self) -> *const BSFixedString
    }

    #[inline(always)]
    pub fn get_pipeline_stage(&self) -> &BSFixedString {
        unsafe { &*self.get_pipeline_stage_ptr() }
    }

    virtual_method! {
        pub const VFUNC_GET_ARBITER_TYPE_PTR: usize = 0x02;
        fn get_arbiter_type_ptr(&self) -> *const BSFixedString
    }

    #[inline(always)]
    pub fn get_arbiter_type(&self) -> &BSFixedString {
        unsafe { &*self.get_arbiter_type_ptr() }
    }

    virtual_method! { pub const VFUNC_GET_ARBITER_SAVE_TYPE: usize = 0x03; pub fn get_arbiter_save_type(&self) -> u8 }
    virtual_method! { pub const VFUNC_REGISTER_WITH_CONTROLLER: usize = 0x04; pub fn register_with_controller(&mut self, controller: &mut IMovementControllerRegisterInterface) }
    virtual_method! { pub const VFUNC_GET_DEBUG_RENDERING_INTERFACE: usize = 0x05; pub fn get_debug_rendering_interface(&mut self) -> *mut IMovementDebugRenderingInterface }
    virtual_method! { pub const VFUNC_ACTIVATE: usize = 0x06; pub fn activate(&mut self) }
    virtual_method! { pub const VFUNC_INIT: usize = 0x07; pub fn init(&mut self, stream: &mut BSPathingStreamRead) }
    virtual_method! { pub const VFUNC_UPDATE_SMALL_DELTA: usize = 0x08; pub fn update_small_delta(&mut self, delta: &mut MovementUpdateDataSmallDelta) }
    virtual_method! { pub const VFUNC_UPDATE_LARGE_DELTA: usize = 0x09; pub fn update_large_delta(&mut self, delta: &mut MovementUpdateDataLargeDelta) }
    virtual_method! { pub const VFUNC_KILL: usize = 0x0A; pub fn kill(&mut self) }
    virtual_method! { pub const VFUNC_DEACTIVATE: usize = 0x0B; pub fn deactivate(&mut self) }
    virtual_method! { pub const VFUNC_UNREGISTER_WITH_CONTROLLER: usize = 0x0C; pub fn unregister_with_controller(&mut self) }
    virtual_method! { pub const VFUNC_SAVE_GAME: usize = 0x0D; pub fn save_game(&mut self, stream: &mut BSPathingStreamWrite) }
    virtual_method! { pub const VFUNC_LOAD_GAME: usize = 0x0E; pub fn load_game(&mut self, stream: &mut BSPathingStreamRead) }
    virtual_method! { pub const VFUNC_ADD_AGENT: usize = 0x0F; pub fn add_agent(&mut self, agent: &BSTSmartPointer<MovementAgent>) -> bool }
    virtual_method! { pub const VFUNC_REMOVE_AGENT: usize = 0x10; pub fn remove_agent(&mut self, agent: &BSTSmartPointer<MovementAgent>) -> bool }
    virtual_method! { pub const VFUNC_REMOVE_ALL_AGENTS: usize = 0x11; pub fn remove_all_agents(&mut self) }
}

pub trait MovementArbiterExt {
    fn get_pipeline_stage(&self) -> &BSFixedString;
    fn get_arbiter_type(&self) -> &BSFixedString;
    fn get_arbiter_save_type(&self) -> u8;
    fn register_with_controller(&mut self, controller: &mut IMovementControllerRegisterInterface);
    fn get_debug_rendering_interface(&mut self) -> *mut IMovementDebugRenderingInterface;
    fn activate(&mut self);
    fn init(&mut self, stream: &mut BSPathingStreamRead);
    fn update_small_delta(&mut self, delta: &mut MovementUpdateDataSmallDelta);
    fn update_large_delta(&mut self, delta: &mut MovementUpdateDataLargeDelta);
    fn kill(&mut self);
    fn deactivate(&mut self);
    fn unregister_with_controller(&mut self);
    fn save_game(&mut self, stream: &mut BSPathingStreamWrite);
    fn load_game(&mut self, stream: &mut BSPathingStreamRead);
    fn add_agent(&mut self, agent: &BSTSmartPointer<MovementAgent>) -> bool;
    fn remove_agent(&mut self, agent: &BSTSmartPointer<MovementAgent>) -> bool;
    fn remove_all_agents(&mut self);
}

impl<T: AsRef<MovementArbiter> + AsMut<MovementArbiter>> MovementArbiterExt for T {
    #[inline(always)]
    fn get_pipeline_stage(&self) -> &BSFixedString {
        MovementArbiter::get_pipeline_stage(self.as_ref())
    }

    #[inline(always)]
    fn get_arbiter_type(&self) -> &BSFixedString {
        MovementArbiter::get_arbiter_type(self.as_ref())
    }

    #[inline(always)]
    fn get_arbiter_save_type(&self) -> u8 {
        MovementArbiter::get_arbiter_save_type(self.as_ref())
    }

    #[inline(always)]
    fn register_with_controller(&mut self, controller: &mut IMovementControllerRegisterInterface) {
        MovementArbiter::register_with_controller(self.as_mut(), controller)
    }

    #[inline(always)]
    fn get_debug_rendering_interface(&mut self) -> *mut IMovementDebugRenderingInterface {
        MovementArbiter::get_debug_rendering_interface(self.as_mut())
    }

    #[inline(always)]
    fn activate(&mut self) {
        MovementArbiter::activate(self.as_mut())
    }

    #[inline(always)]
    fn init(&mut self, stream: &mut BSPathingStreamRead) {
        MovementArbiter::init(self.as_mut(), stream)
    }

    #[inline(always)]
    fn update_small_delta(&mut self, delta: &mut MovementUpdateDataSmallDelta) {
        MovementArbiter::update_small_delta(self.as_mut(), delta)
    }

    #[inline(always)]
    fn update_large_delta(&mut self, delta: &mut MovementUpdateDataLargeDelta) {
        MovementArbiter::update_large_delta(self.as_mut(), delta)
    }

    #[inline(always)]
    fn kill(&mut self) {
        MovementArbiter::kill(self.as_mut())
    }

    #[inline(always)]
    fn deactivate(&mut self) {
        MovementArbiter::deactivate(self.as_mut())
    }

    #[inline(always)]
    fn unregister_with_controller(&mut self) {
        MovementArbiter::unregister_with_controller(self.as_mut())
    }

    #[inline(always)]
    fn save_game(&mut self, stream: &mut BSPathingStreamWrite) {
        MovementArbiter::save_game(self.as_mut(), stream)
    }

    #[inline(always)]
    fn load_game(&mut self, stream: &mut BSPathingStreamRead) {
        MovementArbiter::load_game(self.as_mut(), stream)
    }

    #[inline(always)]
    fn add_agent(&mut self, agent: &BSTSmartPointer<MovementAgent>) -> bool {
        MovementArbiter::add_agent(self.as_mut(), agent)
    }

    #[inline(always)]
    fn remove_agent(&mut self, agent: &BSTSmartPointer<MovementAgent>) -> bool {
        MovementArbiter::remove_agent(self.as_mut(), agent)
    }

    #[inline(always)]
    fn remove_all_agents(&mut self) {
        MovementArbiter::remove_all_agents(self.as_mut())
    }
}

// TODO: This helper preserves the exact 8-byte tagged-pointer ABI, but it intentionally
// omits Rust-side owning Clone/Drop semantics. If Rust code ever constructs or owns
// these values directly, add source-backed copy/destruction behavior that mirrors the
// header's low-bit clear plus `BSTSmartPointer<T>` lifetime rules.
#[repr(C)]
pub struct MovementControllerAIActiveSmartPtr<T: BSTSmartPointerIntrusiveRefCountable> {
    pub raw: usize, // 00
    _marker: PhantomData<*mut T>,
}

impl<T: BSTSmartPointerIntrusiveRefCountable> MovementControllerAIActiveSmartPtr<T> {
    #[inline(always)]
    pub fn set_active(&mut self, value: bool) {
        self.clear();
        self.raw |= value as usize;
    }

    #[inline(always)]
    pub const fn q_active(&self) -> bool {
        (self.raw & 1) != 0
    }

    #[inline(always)]
    pub fn q_ptr(&self) -> BSTSmartPointer<T> {
        unsafe { BSTSmartPointer::new((self.raw & !1usize) as *mut T) }
    }

    #[inline(always)]
    fn clear(&mut self) {
        self.raw &= !1usize;
    }
}

const _: () =
    assert!(core::mem::size_of::<MovementControllerAIActiveSmartPtr<MovementArbiter>>() == 0x8);
const _: () =
    assert!(core::mem::size_of::<MovementControllerAIActiveSmartPtr<MovementAgent>>() == 0x8);

pub type MovementControllerAIActiveArbiterSmartPtr =
    MovementControllerAIActiveSmartPtr<MovementArbiter>;
pub type MovementControllerAIActiveAgentSmartPtr =
    MovementControllerAIActiveSmartPtr<MovementAgent>;

/// C++ `RE::MovementControllerAI`
#[repr(C)]
pub struct MovementControllerAI {
    pub base: IMovementControllerRegisterInterface, // 000
    pub ref_count: AtomicU32,                       // 008
    pub pad00c: u32,                                // 00C
    pub arbiters: BSTSmallArray<
        MovementControllerAIActiveArbiterSmartPtr,
        { core::mem::size_of::<MovementControllerAIActiveArbiterSmartPtr>() * 2 },
    >, // 010
    pub agents: BSTSmallArray<
        MovementControllerAIActiveAgentSmartPtr,
        { core::mem::size_of::<MovementControllerAIActiveAgentSmartPtr>() },
    >, // 030
    pub unk048: u64,
    pub unk050: u64,
    pub unk058: u64,
    pub unk060: u64,
    pub unk068: u64,
    pub unk070: u64,
    pub unk078: u64,
    pub unk080: u64,
    pub unk088: u64,
    pub unk090: u64,
    pub unk098: u64,
    pub unk0a0: u64,
    pub unk0a8: u64,
    pub unk0b0: u64,
    pub unk0b8: u64,
    pub unk0c0: u64,
    pub unk0c8: u64,
    pub unk0d0: u64,
    pub unk0d8: u64,
    pub unk0e0: u64,
    pub unk0e8: u64,
    pub unk0f0: u64,
    pub unk0f8: u64,
    pub unk100: u64,
    pub unk108: u64,
    pub unk110: u64,
    pub unk118: u64,
}

// TODO: CommonLib only proves the flat-runtime `sizeof(MovementControllerAI) == 0x120`.
// Verify VR size and secondary-field offsets before relying on non-prefix layout there.
const _: () = assert!(core::mem::size_of::<MovementControllerAI>() == 0x120);
const _: () = assert!(core::mem::offset_of!(MovementControllerAI, base) == 0x000);
const _: () = assert!(core::mem::offset_of!(MovementControllerAI, ref_count) == 0x008);
const _: () = assert!(core::mem::offset_of!(MovementControllerAI, arbiters) == 0x010);
const _: () = assert!(core::mem::offset_of!(MovementControllerAI, agents) == 0x030);

impl RttiType for MovementControllerAI {
    const RTTI: VariantID = RTTI_MovementControllerAI;
}

inherit!(MovementControllerAI : IMovementControllerRegisterInterface, base);

impl AsRef<MovementControllerAI> for MovementControllerAI {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<MovementControllerAI> for MovementControllerAI {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl MovementControllerAI {
    pub const RTTI: VariantID = RTTI_MovementControllerAI;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MovementControllerAI;

    virtual_method! { pub const VFUNC_UNK_05: usize = 0x05; pub fn unk_05(&mut self) }
    virtual_method! { pub const VFUNC_UNK_06: usize = 0x06; pub fn unk_06(&mut self) }
    virtual_method! { pub const VFUNC_GET_MOVE_DIRECTION: usize = 0x07; pub fn get_move_direction(&mut self, delta: f32, out_move_direction: *mut NiPoint3) }
    virtual_method! { pub const VFUNC_UNK_08: usize = 0x08; pub fn unk_08(&mut self) }
    virtual_method! { pub const VFUNC_UNK_09: usize = 0x09; pub fn unk_09(&mut self) }

    #[inline(always)]
    pub fn get_move_direction_value(&mut self, delta: f32) -> NiPoint3 {
        let mut out_move_direction = NiPoint3::default();
        self.get_move_direction(delta, &mut out_move_direction);
        out_move_direction
    }
}

pub trait MovementControllerAIExt {
    fn unk_05(&mut self);
    fn unk_06(&mut self);
    fn get_move_direction(&mut self, delta: f32, out_move_direction: *mut NiPoint3);
    fn get_move_direction_value(&mut self, delta: f32) -> NiPoint3;
    fn unk_08(&mut self);
    fn unk_09(&mut self);
}

impl<T: AsMut<MovementControllerAI>> MovementControllerAIExt for T {
    #[inline(always)]
    fn unk_05(&mut self) {
        MovementControllerAI::unk_05(self.as_mut())
    }

    #[inline(always)]
    fn unk_06(&mut self) {
        MovementControllerAI::unk_06(self.as_mut())
    }

    #[inline(always)]
    fn get_move_direction(&mut self, delta: f32, out_move_direction: *mut NiPoint3) {
        MovementControllerAI::get_move_direction(self.as_mut(), delta, out_move_direction)
    }

    #[inline(always)]
    fn get_move_direction_value(&mut self, delta: f32) -> NiPoint3 {
        MovementControllerAI::get_move_direction_value(self.as_mut(), delta)
    }

    #[inline(always)]
    fn unk_08(&mut self) {
        MovementControllerAI::unk_08(self.as_mut())
    }

    #[inline(always)]
    fn unk_09(&mut self) {
        MovementControllerAI::unk_09(self.as_mut())
    }
}
