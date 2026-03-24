use crate::core_util::{EnumSet, inherit};
use crate::offsets::offsets_nirtti::NiRTTI_NiTimeController;
use crate::offsets::offsets_rtti::RTTI_NiTimeController;
use crate::offsets::offsets_vtable::VTABLE_NiTimeController;
use crate::re::NiCloningProcess;
use crate::re::NiObject;
use crate::re::NiObjectNET;
use crate::re::NiPointer;
use crate::re::NiRef;
use crate::re::NiStream;
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::{relocation_func, virtual_method};

/// C++ `RE::NiTimeController::CycleType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NiTimeControllerCycleType {
    Loop = 0,
    Reverse = 1,
    Clamp = 2,

    Total = 3,
}

core_util::impl_enumset_type!(NiTimeControllerCycleType => u32);

/// C++ `RE::NiTimeController::Flag`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NiTimeControllerFlag {
    AnimTypeAppInit = 1 << 0,
    CycleTypeReverse = 1 << 1,
    CycleTypeClamp = 2 << 1,
    Active = 1 << 3,
    PlayBackwards = 1 << 4,
    ManagerControlled = 1 << 5,
    ComputeScaledTime = 1 << 6,
    ForceUpdate = 1 << 7,
}

core_util::impl_enumset_type!(NiTimeControllerFlag => u16);

impl NiTimeControllerFlag {
    pub const ANIM_TYPE_APP_TIME: u16 = 0 << 0;
    pub const ANIM_TYPE_MASK: u16 = 1;
    pub const CYCLE_TYPE_LOOP: u16 = 0 << 1;
    pub const CYCLE_TYPE_MASK: u16 = 6;
}

/// C++ `RE::NiTimeController`
#[repr(C)]
pub struct NiTimeController {
    pub base: NiObject,                            // 00
    pub flags: EnumSet<NiTimeControllerFlag, u16>, // 10
    pub pad12: u16,                                // 12
    pub frequency: f32,                            // 14
    pub phase: f32,                                // 18
    pub lo_key_time: f32,                          // 1C
    pub hi_key_time: f32,                          // 20
    pub start_time: f32,                           // 24
    pub last_time: f32,                            // 28
    pub weighted_last_time: f32,                   // 2C
    pub scaled_time: f32,                          // 30
    pub pad34: u32,                                // 34
    pub target: *mut NiObjectNET,                  // 38
    pub next: NiPointer<NiTimeController>,         // 40
}

const _: () = assert!(core::mem::size_of::<NiTimeController>() == 0x48);
const _: () = assert!(core::mem::offset_of!(NiTimeController, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiTimeController, flags) == 0x10);
const _: () = assert!(core::mem::offset_of!(NiTimeController, frequency) == 0x14);
const _: () = assert!(core::mem::offset_of!(NiTimeController, phase) == 0x18);
const _: () = assert!(core::mem::offset_of!(NiTimeController, lo_key_time) == 0x1C);
const _: () = assert!(core::mem::offset_of!(NiTimeController, hi_key_time) == 0x20);
const _: () = assert!(core::mem::offset_of!(NiTimeController, start_time) == 0x24);
const _: () = assert!(core::mem::offset_of!(NiTimeController, last_time) == 0x28);
const _: () = assert!(core::mem::offset_of!(NiTimeController, weighted_last_time) == 0x2C);
const _: () = assert!(core::mem::offset_of!(NiTimeController, scaled_time) == 0x30);
const _: () = assert!(core::mem::offset_of!(NiTimeController, target) == 0x38);
const _: () = assert!(core::mem::offset_of!(NiTimeController, next) == 0x40);

impl RttiType for NiTimeController {
    const RTTI: VariantID = RTTI_NiTimeController;
}

impl NiRef for NiTimeController {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

impl AsRef<NiTimeController> for NiTimeController {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<NiTimeController> for NiTimeController {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(NiTimeController : NiObject, base);

impl NiTimeController {
    pub const RTTI: VariantID = RTTI_NiTimeController;
    pub const NI_RTTI: VariantID = NiRTTI_NiTimeController;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiTimeController;

    // override (NiObject)
    // const NiRTTI* GetRTTI() const override;                            // 02

    // RELOCATION_ID SE: 69433, AE: 70810
    relocation_func! {
        pub fn load_binary(&mut self, a_stream: *mut NiStream) => RelocationID::new(69433, 70810)
    }

    // RELOCATION_ID SE: 69434, AE: 70811
    relocation_func! {
        pub fn link_object(&mut self, a_stream: *mut NiStream) => RelocationID::new(69434, 70811)
    }

    // RELOCATION_ID SE: 69435, AE: 70812
    relocation_func! {
        pub fn register_streamables(&mut self, a_stream: *mut NiStream) -> bool => RelocationID::new(69435, 70812)
    }

    // RELOCATION_ID SE: 69436, AE: 70813
    relocation_func! {
        pub fn save_binary(&mut self, a_stream: *mut NiStream) => RelocationID::new(69436, 70813)
    }

    // RELOCATION_ID SE: 69437, AE: 70814
    relocation_func! {
        pub fn is_equal(&mut self, a_object: *mut NiObject) -> bool => RelocationID::new(69437, 70814)
    }

    // RELOCATION_ID SE: 69449, AE: 70826
    relocation_func! {
        pub fn process_clone(&mut self, a_cloning: *mut NiCloningProcess) => RelocationID::new(69449, 70826)
    }

    // add
    // RELOCATION_ID SE: 69440, AE: 70817
    relocation_func! {
        pub fn start(&mut self, a_time: f32) => RelocationID::new(69440, 70817)
    }

    // RELOCATION_ID SE: 69441, AE: 70818
    relocation_func! {
        pub fn stop(&mut self) => RelocationID::new(69441, 70818)
    }

    virtual_method! {
        pub const VFUNC_UPDATE: usize = 0x27;
        pub fn update(a_time: f32)
    }

    // RELOCATION_ID SE: 69442, AE: 70819
    relocation_func! {
        pub fn set_target(&mut self, a_target: *mut NiObjectNET) => RelocationID::new(69442, 70819)
    }

    virtual_method! {
        pub const VFUNC_IS_TRANSFORM_CONTROLLER: usize = 0x29;
        pub fn is_transform_controller() -> bool
    }

    virtual_method! {
        pub const VFUNC_IS_VERTEX_CONTROLLER: usize = 0x2A;
        pub fn is_vertex_controller() -> bool
    }

    // RELOCATION_ID SE: 69447, AE: 70824
    relocation_func! {
        pub fn compute_scaled_time(&mut self, a_time: f32) -> f32 => RelocationID::new(69447, 70824)
    }

    virtual_method! {
        pub const VFUNC_ON_PRE_DISPLAY: usize = 0x2C;
        pub fn on_pre_display()
    }

    virtual_method! {
        pub const VFUNC_IS_STREAMABLE: usize = 0x2D;
        pub fn is_streamable() -> bool
    }

    virtual_method! {
        pub const VFUNC_TARGET_IS_REQUIRED_TYPE: usize = 0x2E;
        pub fn target_is_required_type() -> bool
    }

    // RELOCATION_ID SE: 69444, AE: 70821
    relocation_func! {
        pub fn start_animations(a_target: *mut NiObjectNET) => RelocationID::new(69444, 70821)
    }

    // RELOCATION_ID SE: 69438, AE: 70815
    relocation_func! {
        pub fn ctor(&mut self) -> *mut NiTimeController => RelocationID::new(69438, 70815)
    }

    // RELOCATION_ID SE: 69439, AE: 70816
    relocation_func! {
        pub fn dtor(&mut self) => RelocationID::new(69439, 70816)
    }

    #[inline(always)]
    pub fn get_next(&self) -> *mut NiTimeController {
        self.next.get()
    }

    #[inline(always)]
    pub fn set_next(&mut self, a_next: *mut NiTimeController) {
        unsafe { self.next.reset_to(a_next) };
    }
}

pub trait NiTimeControllerExt {
    fn load_binary(&mut self, a_stream: *mut NiStream);
    fn link_object(&mut self, a_stream: *mut NiStream);
    fn register_streamables(&mut self, a_stream: *mut NiStream) -> bool;
    fn save_binary(&mut self, a_stream: *mut NiStream);
    fn is_equal(&mut self, a_object: *mut NiObject) -> bool;
    fn process_clone(&mut self, a_cloning: *mut NiCloningProcess);
    fn start(&mut self, a_time: f32);
    fn stop(&mut self);
    fn update(&mut self, a_time: f32);
    fn set_target(&mut self, a_target: *mut NiObjectNET);
    fn is_transform_controller(&self) -> bool;
    fn is_vertex_controller(&self) -> bool;
    fn compute_scaled_time(&mut self, a_time: f32) -> f32;
    fn on_pre_display(&mut self);
    fn is_streamable(&self) -> bool;
    fn target_is_required_type(&self) -> bool;
    fn get_next(&self) -> *mut NiTimeController;
    fn set_next(&mut self, a_next: *mut NiTimeController);
}

impl<T: AsRef<NiTimeController> + AsMut<NiTimeController>> NiTimeControllerExt for T {
    #[inline(always)]
    fn load_binary(&mut self, a_stream: *mut NiStream) {
        NiTimeController::load_binary(self.as_mut(), a_stream)
    }

    #[inline(always)]
    fn link_object(&mut self, a_stream: *mut NiStream) {
        NiTimeController::link_object(self.as_mut(), a_stream)
    }

    #[inline(always)]
    fn register_streamables(&mut self, a_stream: *mut NiStream) -> bool {
        NiTimeController::register_streamables(self.as_mut(), a_stream)
    }

    #[inline(always)]
    fn save_binary(&mut self, a_stream: *mut NiStream) {
        NiTimeController::save_binary(self.as_mut(), a_stream)
    }

    #[inline(always)]
    fn is_equal(&mut self, a_object: *mut NiObject) -> bool {
        NiTimeController::is_equal(self.as_mut(), a_object)
    }

    #[inline(always)]
    fn process_clone(&mut self, a_cloning: *mut NiCloningProcess) {
        NiTimeController::process_clone(self.as_mut(), a_cloning)
    }

    #[inline(always)]
    fn start(&mut self, a_time: f32) {
        NiTimeController::start(self.as_mut(), a_time)
    }

    #[inline(always)]
    fn stop(&mut self) {
        NiTimeController::stop(self.as_mut())
    }

    #[inline(always)]
    fn update(&mut self, a_time: f32) {
        NiTimeController::update(self.as_mut(), a_time)
    }

    #[inline(always)]
    fn set_target(&mut self, a_target: *mut NiObjectNET) {
        NiTimeController::set_target(self.as_mut(), a_target)
    }

    #[inline(always)]
    fn is_transform_controller(&self) -> bool {
        NiTimeController::is_transform_controller(self.as_ref())
    }

    #[inline(always)]
    fn is_vertex_controller(&self) -> bool {
        NiTimeController::is_vertex_controller(self.as_ref())
    }

    #[inline(always)]
    fn compute_scaled_time(&mut self, a_time: f32) -> f32 {
        NiTimeController::compute_scaled_time(self.as_mut(), a_time)
    }

    #[inline(always)]
    fn on_pre_display(&mut self) {
        NiTimeController::on_pre_display(self.as_mut())
    }

    #[inline(always)]
    fn is_streamable(&self) -> bool {
        NiTimeController::is_streamable(self.as_ref())
    }

    #[inline(always)]
    fn target_is_required_type(&self) -> bool {
        NiTimeController::target_is_required_type(self.as_ref())
    }

    #[inline(always)]
    fn get_next(&self) -> *mut NiTimeController {
        NiTimeController::get_next(self.as_ref())
    }

    #[inline(always)]
    fn set_next(&mut self, a_next: *mut NiTimeController) {
        NiTimeController::set_next(self.as_mut(), a_next)
    }
}
