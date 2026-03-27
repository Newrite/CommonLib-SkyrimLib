use core::ffi::c_void;

use crate::offsets::offsets_rtti::RTTI_Hazard;
use crate::offsets::offsets_vtable::VTABLE_Hazard;
use crate::re::{
    ActorHandle, BGSHazard, BSSoundHandle, FormCastable, FormType, NiLight, NiPointer,
    TESObjectREFR,
};
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::Hazard::Flags`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HazardFlags(pub u32);

/// C++ `RE::Hazard::RecordFlags`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HazardRecordFlags(pub u32);

/// C++ `RE::Hazard::HAZARD_RUNTIME_DATA`
#[repr(C)]
pub struct HazardRuntimeData {
    pub hazard_db_handle: *mut c_void, // 00
    pub owner_actor: ActorHandle,      // 08
    pub age: f32,                      // 0C
    pub lifetime: f32,                 // 10
    pub target_timer: f32,             // 14
    pub radius: f32,                   // 18
    pub magnitude: f32,                // 1C
    pub hazard: *mut BGSHazard,        // 20
    pub light: NiPointer<NiLight>,     // 28
    pub sound: BSSoundHandle,          // 30
    pub flags: HazardFlags,            // 3C
}

const _: () = assert!(core::mem::size_of::<HazardRuntimeData>() == 0x40);
const _: () = assert!(core::mem::offset_of!(HazardRuntimeData, hazard_db_handle) == 0x00);
const _: () = assert!(core::mem::offset_of!(HazardRuntimeData, owner_actor) == 0x08);
const _: () = assert!(core::mem::offset_of!(HazardRuntimeData, age) == 0x0C);
const _: () = assert!(core::mem::offset_of!(HazardRuntimeData, lifetime) == 0x10);
const _: () = assert!(core::mem::offset_of!(HazardRuntimeData, target_timer) == 0x14);
const _: () = assert!(core::mem::offset_of!(HazardRuntimeData, radius) == 0x18);
const _: () = assert!(core::mem::offset_of!(HazardRuntimeData, magnitude) == 0x1C);
const _: () = assert!(core::mem::offset_of!(HazardRuntimeData, hazard) == 0x20);
const _: () = assert!(core::mem::offset_of!(HazardRuntimeData, light) == 0x28);
const _: () = assert!(core::mem::offset_of!(HazardRuntimeData, sound) == 0x30);
const _: () = assert!(core::mem::offset_of!(HazardRuntimeData, flags) == 0x3C);

/// C++ `RE::Hazard`
#[repr(C)]
pub struct Hazard {
    pub base: TESObjectREFR, // 00
}

const _: () = assert!(core::mem::size_of::<Hazard>() == 0x80);
const _: () = assert!(core::mem::offset_of!(Hazard, base) == 0x00);

core_util::inherit!(Hazard : TESObjectREFR, base);

impl RttiType for Hazard {
    const RTTI: VariantID = RTTI_Hazard;
}

impl FormCastable for Hazard {
    const TARGET_FORM_TYPE: FormType = FormType::PlacedHazard;
}

impl Hazard {
    pub const RTTI: VariantID = RTTI_Hazard;
    pub const VTABLE: &'static [VariantID] = &VTABLE_Hazard;
    pub const FORMTYPE: FormType = FormType::PlacedHazard;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x98, 0xA0, 0x98);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0xD8, 0xE0, 0xD8);

    crate::runtime_data_accessor! {
        pub fn get_hazard_runtime_data() -> HazardRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_hazard_runtime_data_mut() -> HazardRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    // override (TESObjectREFR)
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    // void SaveGame(BGSSaveFormBuffer* a_buf) override;        // 0E
    // void LoadGame(BGSLoadFormBuffer* a_buf) override;        // 0F
    // void FinishLoadGame(BGSLoadFormBuffer* a_buf) override;  // 11
    // void Revert(BGSLoadFormBuffer* a_buf) override;          // 12
    // void InitItemImpl() override;                            // 13
    // void SetActorCause(ActorCause* a_cause) override;        // 50
    // void Release3DRelatedData() override;                    // 6B
    // bool OnAddCellPerformQueueReference(TESObjectCELL& a_cell) const override; // 90 (flat only)

    crate::relocated_virtual_method! {
        pub const VFUNC_INITIALIZE: VariantOffset = VariantOffset::new_se_ae(0xA2, 0xA3);
        pub fn initialize(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_A3: VariantOffset = VariantOffset::new_se_ae(0xA3, 0xA4);
        pub fn unk_a3(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_IS_PERMANENT: VariantOffset = VariantOffset::new_se_ae(0xA4, 0xA5);
        pub fn is_permanent(&self) -> bool
    }
}
