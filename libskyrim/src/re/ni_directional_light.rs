use crate::offsets::offsets_nirtti::NiRTTI_NiDirectionalLight;
use crate::offsets::offsets_rtti::RTTI_NiDirectionalLight;
use crate::offsets::offsets_vtable::VTABLE_NiDirectionalLight;
use crate::re::{NiColor, NiLight, NiPoint3, NiRef};
use crate::relocation::{RttiType, VariantID};
use crate::{runtime_data_accessor, runtime_data_mut_accessor};

/// C++ `RE::NiDirectionalLight::DIRECTIONAL_LIGHT_RUNTIME_DATA`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DIRECTIONAL_LIGHT_RUNTIME_DATA {
    pub world_dir: NiPoint3,   // 00
    pub effect_color: NiColor, // 0C
}

const _: () = assert!(core::mem::size_of::<DIRECTIONAL_LIGHT_RUNTIME_DATA>() == 0x18);
const _: () = assert!(core::mem::offset_of!(DIRECTIONAL_LIGHT_RUNTIME_DATA, world_dir) == 0x00);
const _: () = assert!(core::mem::offset_of!(DIRECTIONAL_LIGHT_RUNTIME_DATA, effect_color) == 0x0C);

/// Honest common cross-runtime prefix of C++ `RE::NiDirectionalLight`.
#[repr(C)]
pub struct NiDirectionalLight {
    pub base: NiLight, // 000
}

const _: () = assert!(core::mem::size_of::<NiDirectionalLight>() == 0x110);
const _: () = assert!(core::mem::offset_of!(NiDirectionalLight, base) == 0x00);

impl RttiType for NiDirectionalLight {
    const RTTI: VariantID = RTTI_NiDirectionalLight;
}

crate::core_util::inherit!(NiDirectionalLight : NiLight);

impl NiRef for NiDirectionalLight {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

impl NiDirectionalLight {
    pub const RTTI: VariantID = RTTI_NiDirectionalLight;
    pub const NI_RTTI: VariantID = NiRTTI_NiDirectionalLight;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiDirectionalLight;

    // override (NiLight)
    // const NiRTTI* GetRTTI() const override;                           // 02
    // NiObject*     CreateClone(NiCloningProcess& a_cloning) override;  // 17
    // void          LoadBinary(NiStream& a_stream) override;            // 18
    // void          SaveBinary(NiStream& a_stream) override;            // 1B
    // bool          IsEqual(NiObject* a_object) override;               // 1C

    runtime_data_accessor! {
        pub fn get_directional_light_runtime_data() -> DIRECTIONAL_LIGHT_RUNTIME_DATA {
            se_ae: 0x140,
            vr: 0x168
        }
    }

    runtime_data_mut_accessor! {
        pub fn get_directional_light_runtime_data_mut() -> DIRECTIONAL_LIGHT_RUNTIME_DATA {
            se_ae: 0x140,
            vr: 0x168
        }
    }

    #[inline(always)]
    pub fn get_world_direction(&self) -> &NiPoint3 {
        &self.get_directional_light_runtime_data().world_dir
    }
}
