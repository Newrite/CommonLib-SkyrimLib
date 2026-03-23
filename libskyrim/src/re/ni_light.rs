use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiLight;
use crate::offsets::offsets_rtti::RTTI_NiLight;
use crate::offsets::offsets_vtable::VTABLE_NiLight;
use crate::re::NiAVObject;
use crate::re::NiColor;
use crate::re::NiPoint3;
use crate::relocation::{RttiType, VariantID};
use crate::{runtime_data_accessor, runtime_data_mut_accessor};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LIGHT_RUNTIME_DATA {
    pub ambient: NiColor, // 0x00
    pub diffuse: NiColor, // 0x0C
    pub radius: NiPoint3, // 0x18
    pub fade: f32,        // 0x24
    pub unk138: u32,      // 0x28
}

const _: () = assert!(core::mem::size_of::<LIGHT_RUNTIME_DATA>() == 0x2C);

/// Common cross-runtime prefix for `RE::NiLight`.
#[repr(C)]
pub struct NiLight {
    pub base: NiAVObject, // 0x000
}

const _: () = assert!(core::mem::size_of::<NiLight>() == 0x110);

impl RttiType for NiLight {
    const RTTI: VariantID = RTTI_NiLight;
}

inherit!(NiLight : NiAVObject);

impl NiLight {
    pub const RTTI: VariantID = RTTI_NiLight;
    pub const NI_RTTI: VariantID = NiRTTI_NiLight;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiLight;

    // override (NiAVObject)
    // const NiRTTI* GetRTTI() const override;                 // 02
    // void          LoadBinary(NiStream& a_stream) override;  // 18
    // void          SaveBinary(NiStream& a_stream) override;  // 1B
    // bool          IsEqual(NiObject* a_object) override;     // 1C

    runtime_data_accessor! {
        pub fn get_light_runtime_data() -> LIGHT_RUNTIME_DATA {
            se_ae: 0x110,
            vr: 0x138
        }
    }

    runtime_data_mut_accessor! {
        pub fn get_light_runtime_data_mut() -> LIGHT_RUNTIME_DATA {
            se_ae: 0x110,
            vr: 0x138
        }
    }
}
