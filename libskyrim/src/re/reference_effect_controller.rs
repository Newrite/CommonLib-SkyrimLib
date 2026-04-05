use crate::offsets::offsets_rtti::RTTI_ReferenceEffectController;
use crate::offsets::offsets_vtable::VTABLE_ReferenceEffectController;
use crate::re::BGSArtObject;
use crate::re::BGSLoadGameBuffer;
use crate::re::BGSSaveGameBuffer;
use crate::re::BSFixedString;
use crate::re::BSTempEffect;
use crate::re::NiAVObject;
use crate::re::NiNode;
use crate::re::NiObject;
use crate::re::NiPoint3;
use crate::re::NiPointer;
use crate::re::NiRTTI;
use crate::re::ReferenceEffect;
use crate::re::TESEffectShader;
use crate::re::TESObjectREFR;
use crate::re::bs_atomic::{BSSpinLock, BSSpinLockGuard};
use crate::re::bst_array::BSTArray;
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::{relocation_func, relocation_variable, virtual_method};

#[repr(C)]
struct ProcessListsReferenceEffectStopView {
    pad00: [u8; 0x108],                               // 00
    magic_effects: BSTArray<NiPointer<BSTempEffect>>, // 108
    magic_effects_lock: BSSpinLock,                   // 120
}

const _: () = assert!(core::mem::size_of::<ProcessListsReferenceEffectStopView>() == 0x128);
const _: () =
    assert!(core::mem::offset_of!(ProcessListsReferenceEffectStopView, magic_effects) == 0x108);
const _: () = assert!(
    core::mem::offset_of!(ProcessListsReferenceEffectStopView, magic_effects_lock) == 0x120
);

impl ProcessListsReferenceEffectStopView {
    relocation_variable! {
        fn get_singleton() -> *mut ProcessListsReferenceEffectStopView => RelocationID::new(514167, 400315), is_indirect_ptr
    }
}

/// C++ `RE::ReferenceEffectController`
#[repr(C)]
pub struct ReferenceEffectController {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<ReferenceEffectController>() == 0x8);
const _: () = assert!(core::mem::offset_of!(ReferenceEffectController, vtable) == 0x00);

impl RttiType for ReferenceEffectController {
    const RTTI: VariantID = RTTI_ReferenceEffectController;
}

impl ReferenceEffectController {
    pub const RTTI: VariantID = RTTI_ReferenceEffectController;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ReferenceEffectController;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_HANDLE_EVENT: usize = 0x01;
        pub fn handle_event(a_event: *const BSFixedString)
    }

    virtual_method! {
        pub const VFUNC_GET_ELAPSED_TIME: usize = 0x02;
        pub fn get_elapsed_time() -> f32
    }

    virtual_method! {
        pub const VFUNC_GET_SCALE: usize = 0x03;
        pub fn get_scale() -> f32
    }

    virtual_method! {
        pub const VFUNC_SWITCH_ATTACHED_ROOT: usize = 0x04;
        pub fn switch_attached_root(a_root: *mut NiNode, a_attach_root: *mut NiNode)
    }

    virtual_method! {
        pub const VFUNC_GET_SOURCE_POSITION: usize = 0x05;
        pub fn get_source_position() -> &'static NiPoint3
    }

    virtual_method! {
        pub const VFUNC_GET_USE_SOURCE_POSITION: usize = 0x06;
        pub fn get_use_source_position() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_NO_INITIAL_FLARE: usize = 0x07;
        pub fn get_no_initial_flare() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_EFFECT_PERSISTS: usize = 0x08;
        pub fn get_effect_persists() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_GORY_VISUALS: usize = 0x09;
        pub fn get_gory_visuals() -> bool
    }

    virtual_method! {
        pub const VFUNC_REMOVE_HIT_EFFECT: usize = 0x0A;
        pub fn remove_hit_effect(a_ref_effect: *mut ReferenceEffect)
    }

    virtual_method! {
        pub const VFUNC_GET_TARGET_REFERENCE: usize = 0x0B;
        pub fn get_target_reference() -> *mut TESObjectREFR
    }

    virtual_method! {
        pub const VFUNC_GET_HIT_EFFECT_ART: usize = 0x0C;
        pub fn get_hit_effect_art() -> *mut BGSArtObject
    }

    virtual_method! {
        pub const VFUNC_GET_HIT_EFFECT_SHADER: usize = 0x0D;
        pub fn get_hit_effect_shader() -> *mut TESEffectShader
    }

    virtual_method! {
        pub const VFUNC_GET_MANAGER_HANDLES_SAVE_LOAD: usize = 0x0E;
        pub fn get_manager_handles_save_load() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_ATTACH_ROOT: usize = 0x0F;
        pub fn get_attach_root() -> *mut NiAVObject
    }

    virtual_method! {
        pub const VFUNC_GET_PARTICLE_ATTACH_EXTENT: usize = 0x10;
        pub fn get_particle_attach_extent() -> f32
    }

    virtual_method! {
        pub const VFUNC_GET_USE_PARTICLE_ATTACH_EXTENT: usize = 0x11;
        pub fn get_use_particle_attach_extent() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_DO_PARTICLES: usize = 0x12;
        pub fn get_do_particles() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_PARTICLES_USE_LOCAL_SPACE: usize = 0x13;
        pub fn get_particles_use_local_space() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_USE_ROOT_WORLD_ROTATE: usize = 0x14;
        pub fn get_use_root_world_rotate() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_IS_ROOT_ACTOR: usize = 0x15;
        pub fn get_is_root_actor() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_CLEAR_WHEN_CELL_IS_UNLOADED: usize = 0x16;
        pub fn get_clear_when_cell_is_unloaded() -> bool
    }

    virtual_method! {
        pub const VFUNC_EFFECT_SHOULD_FACE_TARGET: usize = 0x17;
        pub fn effect_should_face_target() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_FACING_TARGET: usize = 0x18;
        pub fn get_facing_target() -> *mut TESObjectREFR
    }

    virtual_method! {
        pub const VFUNC_GET_SHADER_USE_PARENT_CELL: usize = 0x19;
        pub fn get_shader_use_parent_cell() -> bool
    }

    virtual_method! {
        pub const VFUNC_EFFECT_ATTACHES_TO_CAMERA: usize = 0x1A;
        pub fn effect_attaches_to_camera() -> bool
    }

    virtual_method! {
        pub const VFUNC_EFFECT_ROTATES_WITH_CAMERA: usize = 0x1B;
        pub fn effect_rotates_with_camera() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_ALLOW_TARGET_ROOT: usize = 0x1C;
        pub fn get_allow_target_root() -> bool
    }

    virtual_method! {
        pub const VFUNC_IS_READY_FOR_ATTACH: usize = 0x1D;
        pub fn is_ready_for_attach() -> bool
    }

    virtual_method! {
        pub const VFUNC_SET_WIND_POINT: usize = 0x1E;
        pub fn set_wind_point(a_point: *const NiPoint3)
    }

    virtual_method! {
        pub const VFUNC_GET_WIND_POINT: usize = 0x1F;
        pub fn get_wind_point() -> &'static NiPoint3
    }

    virtual_method! {
        pub const VFUNC_GET_ALLOW_NO_3D: usize = 0x20;
        pub fn get_allow_no_3d() -> bool
    }

    virtual_method! {
        pub const VFUNC_SAVE_GAME: usize = 0x21;
        pub fn save_game(a_buf: *mut BGSSaveGameBuffer)
    }

    virtual_method! {
        pub const VFUNC_LOAD_GAME: usize = 0x22;
        pub fn load_game(a_buf: *mut BGSLoadGameBuffer)
    }

    relocation_func! {
        pub fn start(&mut self, a_effect_out: *mut *mut ReferenceEffect) => RelocationID::new(33961, 34761)
    }

    #[inline]
    pub fn stop(&mut self) {
        let process_lists = ProcessListsReferenceEffectStopView::get_singleton();
        if process_lists.is_null() {
            return;
        }

        let reference_effect_rtti = ReferenceEffect::NI_RTTI.address() as *const NiRTTI;
        if reference_effect_rtti.is_null() {
            return;
        }

        unsafe {
            let process_lists = &mut *process_lists;
            let _locker = BSSpinLockGuard::from_ptr(core::ptr::addr_of_mut!(
                process_lists.magic_effects_lock
            ));

            for effect in process_lists.magic_effects.as_mut_slice() {
                let temp_effect = effect.get();
                if temp_effect.is_null() {
                    continue;
                }

                let ni_object = temp_effect.cast::<NiObject>();
                let temp_rtti = (*ni_object).get_rtti();
                if temp_rtti.is_null() || !(*temp_rtti).is_kind_of(reference_effect_rtti) {
                    continue;
                }

                let reference_effect = temp_effect.cast::<ReferenceEffect>();
                if (*reference_effect).controller == self as *mut Self {
                    effect.reset();
                }
            }
        }
    }
}

impl AsRef<ReferenceEffectController> for ReferenceEffectController {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<ReferenceEffectController> for ReferenceEffectController {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub trait ReferenceEffectControllerExt {
    fn dtor(&self);
    fn handle_event(&self, a_event: *const BSFixedString);
    fn get_elapsed_time(&self) -> f32;
    fn get_scale(&self) -> f32;
    fn switch_attached_root(&self, a_root: *mut NiNode, a_attach_root: *mut NiNode);
    fn get_source_position(&self) -> &'static NiPoint3;
    fn get_use_source_position(&self) -> bool;
    fn get_no_initial_flare(&self) -> bool;
    fn get_effect_persists(&self) -> bool;
    fn get_gory_visuals(&self) -> bool;
    fn remove_hit_effect(&self, a_ref_effect: *mut ReferenceEffect);
    fn get_target_reference(&self) -> *mut TESObjectREFR;
    fn get_hit_effect_art(&self) -> *mut BGSArtObject;
    fn get_hit_effect_shader(&self) -> *mut TESEffectShader;
    fn get_manager_handles_save_load(&self) -> bool;
    fn get_attach_root(&self) -> *mut NiAVObject;
    fn get_particle_attach_extent(&self) -> f32;
    fn get_use_particle_attach_extent(&self) -> bool;
    fn get_do_particles(&self) -> bool;
    fn get_particles_use_local_space(&self) -> bool;
    fn get_use_root_world_rotate(&self) -> bool;
    fn get_is_root_actor(&self) -> bool;
    fn get_clear_when_cell_is_unloaded(&self) -> bool;
    fn effect_should_face_target(&self) -> bool;
    fn get_facing_target(&self) -> *mut TESObjectREFR;
    fn get_shader_use_parent_cell(&self) -> bool;
    fn effect_attaches_to_camera(&self) -> bool;
    fn effect_rotates_with_camera(&self) -> bool;
    fn get_allow_target_root(&self) -> bool;
    fn is_ready_for_attach(&self) -> bool;
    fn set_wind_point(&self, a_point: *const NiPoint3);
    fn get_wind_point(&self) -> &'static NiPoint3;
    fn get_allow_no_3d(&self) -> bool;
    fn save_game(&self, a_buf: *mut BGSSaveGameBuffer);
    fn load_game(&self, a_buf: *mut BGSLoadGameBuffer);
    fn start(&mut self, a_effect_out: *mut *mut ReferenceEffect);
    fn stop(&mut self);
}

impl<T: AsRef<ReferenceEffectController> + AsMut<ReferenceEffectController>>
    ReferenceEffectControllerExt for T
{
    fn dtor(&self) {
        self.as_ref().dtor()
    }

    fn handle_event(&self, a_event: *const BSFixedString) {
        self.as_ref().handle_event(a_event)
    }

    fn get_elapsed_time(&self) -> f32 {
        self.as_ref().get_elapsed_time()
    }

    fn get_scale(&self) -> f32 {
        self.as_ref().get_scale()
    }

    fn switch_attached_root(&self, a_root: *mut NiNode, a_attach_root: *mut NiNode) {
        self.as_ref().switch_attached_root(a_root, a_attach_root)
    }

    fn get_source_position(&self) -> &'static NiPoint3 {
        self.as_ref().get_source_position()
    }

    fn get_use_source_position(&self) -> bool {
        self.as_ref().get_use_source_position()
    }

    fn get_no_initial_flare(&self) -> bool {
        self.as_ref().get_no_initial_flare()
    }

    fn get_effect_persists(&self) -> bool {
        self.as_ref().get_effect_persists()
    }

    fn get_gory_visuals(&self) -> bool {
        self.as_ref().get_gory_visuals()
    }

    fn remove_hit_effect(&self, a_ref_effect: *mut ReferenceEffect) {
        self.as_ref().remove_hit_effect(a_ref_effect)
    }

    fn get_target_reference(&self) -> *mut TESObjectREFR {
        self.as_ref().get_target_reference()
    }

    fn get_hit_effect_art(&self) -> *mut BGSArtObject {
        self.as_ref().get_hit_effect_art()
    }

    fn get_hit_effect_shader(&self) -> *mut TESEffectShader {
        self.as_ref().get_hit_effect_shader()
    }

    fn get_manager_handles_save_load(&self) -> bool {
        self.as_ref().get_manager_handles_save_load()
    }

    fn get_attach_root(&self) -> *mut NiAVObject {
        self.as_ref().get_attach_root()
    }

    fn get_particle_attach_extent(&self) -> f32 {
        self.as_ref().get_particle_attach_extent()
    }

    fn get_use_particle_attach_extent(&self) -> bool {
        self.as_ref().get_use_particle_attach_extent()
    }

    fn get_do_particles(&self) -> bool {
        self.as_ref().get_do_particles()
    }

    fn get_particles_use_local_space(&self) -> bool {
        self.as_ref().get_particles_use_local_space()
    }

    fn get_use_root_world_rotate(&self) -> bool {
        self.as_ref().get_use_root_world_rotate()
    }

    fn get_is_root_actor(&self) -> bool {
        self.as_ref().get_is_root_actor()
    }

    fn get_clear_when_cell_is_unloaded(&self) -> bool {
        self.as_ref().get_clear_when_cell_is_unloaded()
    }

    fn effect_should_face_target(&self) -> bool {
        self.as_ref().effect_should_face_target()
    }

    fn get_facing_target(&self) -> *mut TESObjectREFR {
        self.as_ref().get_facing_target()
    }

    fn get_shader_use_parent_cell(&self) -> bool {
        self.as_ref().get_shader_use_parent_cell()
    }

    fn effect_attaches_to_camera(&self) -> bool {
        self.as_ref().effect_attaches_to_camera()
    }

    fn effect_rotates_with_camera(&self) -> bool {
        self.as_ref().effect_rotates_with_camera()
    }

    fn get_allow_target_root(&self) -> bool {
        self.as_ref().get_allow_target_root()
    }

    fn is_ready_for_attach(&self) -> bool {
        self.as_ref().is_ready_for_attach()
    }

    fn set_wind_point(&self, a_point: *const NiPoint3) {
        self.as_ref().set_wind_point(a_point)
    }

    fn get_wind_point(&self) -> &'static NiPoint3 {
        self.as_ref().get_wind_point()
    }

    fn get_allow_no_3d(&self) -> bool {
        self.as_ref().get_allow_no_3d()
    }

    fn save_game(&self, a_buf: *mut BGSSaveGameBuffer) {
        self.as_ref().save_game(a_buf)
    }

    fn load_game(&self, a_buf: *mut BGSLoadGameBuffer) {
        self.as_ref().load_game(a_buf)
    }

    fn start(&mut self, a_effect_out: *mut *mut ReferenceEffect) {
        self.as_mut().start(a_effect_out)
    }

    fn stop(&mut self) {
        self.as_mut().stop()
    }
}
