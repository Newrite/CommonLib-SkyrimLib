use core_util::EnumSet;

use crate::offsets::offsets_rtti::RTTI_Explosion;
use crate::offsets::offsets_vtable::VTABLE_Explosion;
use crate::re::{
    ActorCause, ActorHandle, BSSoundHandle, FormCastable, FormType, ModelDBHandle, NiPoint3,
    NiPointLight, NiPointer, NonActorMagicCaster, TESObjectREFR, TESObjectWEAP,
    bhkSimpleShapePhantom,
};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExplosionFlags {
    None = 0,
    DecalsPlaced = 1 << 0,
    TargetsFound = 1 << 1,
    TargetsProcessed = 1 << 2,
    ForcesApplied = 1 << 3,
    IgnoreImageSpaceSwap = 1 << 4,
    Underwater = 1 << 5,
    Initialized = 1 << 6,
    NonHostile = 1 << 7,
    WaterTestDone = 1 << 8,
    SoundTestDone = 1 << 9,
}

core_util::impl_enumset_type!(ExplosionFlags => u32);

/// C++ `RE::Explosion::EXPLOSION_RUNTIME_DATA`
#[repr(C)]
pub struct ExplosionRuntimeData {
    pub explosion_db_handle: ModelDBHandle,     // 00
    pub age: f32,                               // 08
    pub lifetime: f32,                          // 0C
    pub hit_time: f32,                          // 10
    pub radius: f32,                            // 14
    pub imod_radius: f32,                       // 18
    pub unk_b4: f32,                            // 1C
    pub unk_b8: *mut bhkSimpleShapePhantom,     // 20
    pub unk_c0: u64,                            // 28
    pub unk_c8: u64,                            // 30
    pub sound01: BSSoundHandle,                 // 38
    pub sound02: BSSoundHandle,                 // 44
    pub light: NiPointer<NiPointLight>,         // 50
    pub actor_owner: ActorHandle,               // 58
    pub unk_f4: ActorHandle,                    // 5C
    pub unk_f8: u32,                            // 60
    pub pad_fc: u32,                            // 64
    pub actor_cause: NiPointer<ActorCause>,     // 68
    pub magic_caster: *mut NonActorMagicCaster, // 70
    pub weapon_source: *mut TESObjectWEAP,      // 78
    pub frame_count: u32,                       // 80
    pub unk_11c: NiPoint3,                      // 84
    pub negative_velocity: NiPoint3,            // 90
    pub damage: f32,                            // 9C
    pub unk_138: f32,                           // A0
    pub flags: EnumSet<ExplosionFlags, u32>,    // A4
}

const _: () = assert!(core::mem::size_of::<ExplosionRuntimeData>() == 0xA8);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, explosion_db_handle) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, age) == 0x08);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, lifetime) == 0x0C);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, hit_time) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, radius) == 0x14);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, imod_radius) == 0x18);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, unk_b4) == 0x1C);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, unk_b8) == 0x20);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, unk_c0) == 0x28);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, unk_c8) == 0x30);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, sound01) == 0x38);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, sound02) == 0x44);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, light) == 0x50);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, actor_owner) == 0x58);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, unk_f4) == 0x5C);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, unk_f8) == 0x60);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, pad_fc) == 0x64);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, actor_cause) == 0x68);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, magic_caster) == 0x70);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, weapon_source) == 0x78);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, frame_count) == 0x80);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, unk_11c) == 0x84);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, negative_velocity) == 0x90);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, damage) == 0x9C);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, unk_138) == 0xA0);
const _: () = assert!(core::mem::offset_of!(ExplosionRuntimeData, flags) == 0xA4);

/// C++ `RE::Explosion`
#[repr(C)]
pub struct Explosion {
    pub base: TESObjectREFR, // 00
}

const _: () = assert!(core::mem::size_of::<Explosion>() == 0x80);
const _: () = assert!(core::mem::offset_of!(Explosion, base) == 0x00);

core_util::inherit!(Explosion : TESObjectREFR, base);

impl RttiType for Explosion {
    const RTTI: VariantID = RTTI_Explosion;
}

impl FormCastable for Explosion {
    const TARGET_FORM_TYPE: FormType = FormType::Explosion;
}

impl Explosion {
    pub const RTTI: VariantID = RTTI_Explosion;
    pub const VTABLE: &'static [VariantID] = &VTABLE_Explosion;
    pub const FORMTYPE: FormType = FormType::Explosion;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x98, 0xA0, 0x98);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x140, 0x148, 0x140);

    crate::runtime_data_accessor! {
        pub fn get_explosion_runtime_data() -> ExplosionRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_explosion_runtime_data_mut() -> ExplosionRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    // override (TESObjectREFR)
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    // void SaveGame(BGSSaveFormBuffer* a_buf) override;                   // 0E
    // void Revert(BGSLoadFormBuffer* a_buf) override;                     // 12
    // void SetActorCause(ActorCause* a_cause) override;                   // 50
    // ActorCause* GetActorCause() const override;                         // 51
    // MagicCaster* GetMagicCaster(MagicSystem::CastingSource a_source) override; // 5C
    // void InitHavok() override;                                          // 66
    // void Release3DRelatedData() override;                               // 6B
    // Explosion* AsExplosion() override;                                  // 8E (flat only)
    // bool OnAddCellPerformQueueReference(TESObjectCELL& a_cell) const override; // 90 (flat only)

    crate::relocated_virtual_method! {
        pub const VFUNC_INITIALIZE: VariantOffset = VariantOffset::new_se_ae(0xA2, 0xA3);
        pub fn initialize(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE: VariantOffset = VariantOffset::new_se_ae(0xA3, 0xA4);
        pub fn update(&mut self, delta: f32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_FIND_TARGETS: VariantOffset = VariantOffset::new_se_ae(0xA4, 0xA5);
        pub fn find_targets(&mut self)
    }
}
