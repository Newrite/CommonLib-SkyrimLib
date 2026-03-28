use crate::offsets::offsets_rtti::RTTI_BGSDestructibleObjectForm;
use crate::offsets::offsets_vtable::VTABLE_BGSDestructibleObjectForm;
use crate::re::base_form_component::BaseFormComponent;
use crate::re::bgs_debris::BGSDebris;
use crate::re::bgs_explosion::BGSExplosion;
use crate::re::queued_file::QueuedFile;
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::{EnumSet, inherit};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DestructibleObjectStageFlag {
    None = 0,
    CapDamage = 1 << 0,
    DisableObject = 1 << 1,
    DestroyObject = 1 << 2,
    IgnoreExternalDamage = 1 << 3,
    BecomesDynamic = 1 << 4,
}

core_util::impl_enumset_type!(DestructibleObjectStageFlag => u8);

#[repr(C)]
pub struct DestructibleObjectStage {
    pub model_damage_stage: i8,                          // 00
    pub health_percentage: i8,                           // 01
    pub flags: EnumSet<DestructibleObjectStageFlag, u8>, // 02
    pub pad03: u8,                                       // 03
    pub self_damage_per_second: u32,                     // 04
    pub explosion: *mut BGSExplosion,                    // 08
    pub debris: *mut BGSDebris,                          // 10
    pub debris_count: u32,                               // 18
    pub pad1c: u32,                                      // 1C
    pub replacement_model: *mut TESModelTextureSwap,     // 20
}

const _: () = assert!(core::mem::size_of::<DestructibleObjectStage>() == 0x28);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectStage, model_damage_stage) == 0x00);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectStage, health_percentage) == 0x01);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectStage, flags) == 0x02);
const _: () =
    assert!(core::mem::offset_of!(DestructibleObjectStage, self_damage_per_second) == 0x04);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectStage, explosion) == 0x08);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectStage, debris) == 0x10);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectStage, debris_count) == 0x18);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectStage, pad1c) == 0x1C);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectStage, replacement_model) == 0x20);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DestructibleObjectDataFlag {
    None = 0,
    VatsTargetable = 1 << 0,
}

core_util::impl_enumset_type!(DestructibleObjectDataFlag => u8);

#[repr(C)]
pub struct DestructibleObjectData {
    pub health: u32,                                    // 00
    pub num_stages: i8,                                 // 04
    pub flags: EnumSet<DestructibleObjectDataFlag, u8>, // 05
    pub pad06: u16,                                     // 06
    pub stages: *mut *mut DestructibleObjectStage,      // 08
    pub replacement_model_ref_count: i32,               // 10
    pub pad14: u32,                                     // 14
    // TODO: Keep this as a raw `*mut QueuedFile` until vendored source provides
    // a source-backed `QueuedFile` NiRef ownership layer; then restore the
    // honest end state `NiPointer<QueuedFile>`.
    pub preloaded_replacement_models: *mut QueuedFile, // 18
}

const _: () = assert!(core::mem::size_of::<DestructibleObjectData>() == 0x20);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectData, health) == 0x00);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectData, num_stages) == 0x04);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectData, flags) == 0x05);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectData, pad06) == 0x06);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectData, stages) == 0x08);
const _: () =
    assert!(core::mem::offset_of!(DestructibleObjectData, replacement_model_ref_count) == 0x10);
const _: () = assert!(core::mem::offset_of!(DestructibleObjectData, pad14) == 0x14);
const _: () =
    assert!(core::mem::offset_of!(DestructibleObjectData, preloaded_replacement_models) == 0x18);

#[repr(C)]
pub struct BGSDestructibleObjectForm {
    pub base: BaseFormComponent,           // 00
    pub data: *mut DestructibleObjectData, // 08
}

const _: () = assert!(core::mem::size_of::<BGSDestructibleObjectForm>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSDestructibleObjectForm, data) == 0x08);

impl RttiType for BGSDestructibleObjectForm {
    const RTTI: VariantID = RTTI_BGSDestructibleObjectForm;
}

inherit!(BGSDestructibleObjectForm : BaseFormComponent);

impl BGSDestructibleObjectForm {
    pub const RTTI: VariantID = RTTI_BGSDestructibleObjectForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSDestructibleObjectForm;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    virtual_method! {
        pub const VFUNC_INITIALIZE_DATA_COMPONENT: usize = 0x01;
        pub fn initialize_data_component(&mut self)
    }

    virtual_method! {
        pub const VFUNC_CLEAR_DATA_COMPONENT: usize = 0x02;
        pub fn clear_data_component(&mut self)
    }

    virtual_method! {
        pub const VFUNC_COPY_COMPONENT: usize = 0x03;
        pub fn copy_component(&mut self, rhs: *mut BaseFormComponent)
    }

    #[inline(always)]
    pub const fn get_destructible_data_ptr(&self) -> *mut DestructibleObjectData {
        self.data
    }

    #[inline(always)]
    pub fn get_destructible_data_ref(&self) -> Option<&DestructibleObjectData> {
        unsafe { self.get_destructible_data_ptr().as_ref() }
    }

    #[inline(always)]
    pub const fn has_destructible_data(&self) -> bool {
        !self.data.is_null()
    }
}

pub trait BGSDestructibleObjectFormExt {
    fn dtor(&mut self);
    fn initialize_data_component(&mut self);
    fn clear_data_component(&mut self);
    fn copy_component(&mut self, rhs: *mut BaseFormComponent);
    fn get_destructible_data_ptr(&self) -> *mut DestructibleObjectData;
    fn get_destructible_data_ref(&self) -> Option<&DestructibleObjectData>;
    fn has_destructible_data(&self) -> bool;
}

impl<T: AsRef<BGSDestructibleObjectForm> + AsMut<BGSDestructibleObjectForm>>
    BGSDestructibleObjectFormExt for T
{
    fn dtor(&mut self) {
        BGSDestructibleObjectForm::dtor(self.as_mut())
    }

    fn initialize_data_component(&mut self) {
        BGSDestructibleObjectForm::initialize_data_component(self.as_mut())
    }

    fn clear_data_component(&mut self) {
        BGSDestructibleObjectForm::clear_data_component(self.as_mut())
    }

    fn copy_component(&mut self, rhs: *mut BaseFormComponent) {
        BGSDestructibleObjectForm::copy_component(self.as_mut(), rhs)
    }

    fn get_destructible_data_ptr(&self) -> *mut DestructibleObjectData {
        self.as_ref().get_destructible_data_ptr()
    }

    fn get_destructible_data_ref(&self) -> Option<&DestructibleObjectData> {
        self.as_ref().get_destructible_data_ref()
    }

    fn has_destructible_data(&self) -> bool {
        self.as_ref().has_destructible_data()
    }
}
