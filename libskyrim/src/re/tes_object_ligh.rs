use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_TESObjectLIGH;
use crate::offsets::offsets_vtable::VTABLE_TESObjectLIGH;
use crate::re::BGSDestructibleObjectForm;
use crate::re::BGSEquipType;
use crate::re::BGSLensFlare;
use crate::re::BGSMessageIcon;
use crate::re::BGSSoundDescriptorForm;
use crate::re::Color;
use crate::re::NiColor;
use crate::re::NiLight;
use crate::re::NiNode;
use crate::re::TESBoundAnimObject;
use crate::re::TESFullName;
use crate::re::TESIcon;
use crate::re::TESModelTextureSwap;
use crate::re::TESObjectREFR;
use crate::re::TESValueForm;
use crate::re::TESWeightForm;
use crate::relocation::{RelocationID, RttiType, VariantID};

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESLightFlags: u32 {
        const kNone = 0;
        const kDynamic = 1 << 0;
        const kCanCarry = 1 << 1;
        const kNegative = 1 << 2;
        const kFlicker = 1 << 3;
        const kDeepCopy = 1 << 4;
        const kOffByDefault = 1 << 5;
        const kFlickerSlow = 1 << 6;
        const kPulse = 1 << 7;
        const kPulseSlow = 1 << 8;
        const kSpotlight = 1 << 9;
        const kSpotShadow = 1 << 10;
        const kHemiShadow = 1 << 11;
        const kOmniShadow = 1 << 12;
        const kPortalStrict = 1 << 13;
        const kType = Self::kSpotlight.bits() | Self::kSpotShadow.bits() | Self::kHemiShadow.bits() | Self::kOmniShadow.bits();
    }
}

unsafe impl bytemuck::Zeroable for TESLightFlags {}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable)]
pub struct OBJ_LIGH {
    pub time: i32,                        // 0x00
    pub radius: u32,                      // 0x04
    pub color: Color,                     // 0x08
    pub flags: TESLightFlags,             // 0x0C
    pub falloff_exponent: f32,            // 0x10
    pub fov: f32,                         // 0x14
    pub near_distance: f32,               // 0x18
    pub flicker_period_recip: f32,        // 0x1C
    pub flicker_intensity_amplitude: f32, // 0x20
    pub flicker_movement_amplitude: f32,  // 0x24
}

const _: () = assert!(core::mem::size_of::<OBJ_LIGH>() == 0x28);

#[repr(C)]
pub struct TESObjectLIGH {
    pub base: TESBoundAnimObject,                            // 0x000
    pub full_name: TESFullName,                              // 0x030
    pub model_texture_swap: TESModelTextureSwap,             // 0x040
    pub icon: TESIcon,                                       // 0x078
    pub message_icon: BGSMessageIcon,                        // 0x088
    pub weight_form: TESWeightForm,                          // 0x0A0
    pub value_form: TESValueForm,                            // 0x0B0
    pub destructible_object_form: BGSDestructibleObjectForm, // 0x0C0
    pub equip_type: BGSEquipType,                            // 0x0D0
    pub data: OBJ_LIGH,                                      // 0x0E0 - DATA
    pub fade: f32,                                           // 0x108 - FNAM
    pub pad10c: u32,                                         // 0x10C
    pub sound: *mut BGSSoundDescriptorForm,                  // 0x110 - SNAM
    pub emittance_color: NiColor,                            // 0x118
    pub pad124: u32,                                         // 0x124
    pub lens_flare: *mut BGSLensFlare,                       // 0x128
}

const _: () = assert!(core::mem::size_of::<TESObjectLIGH>() == 0x130);
const _: () = assert!(core::mem::offset_of!(TESObjectLIGH, full_name) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESObjectLIGH, model_texture_swap) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESObjectLIGH, icon) == 0x78);
const _: () = assert!(core::mem::offset_of!(TESObjectLIGH, message_icon) == 0x88);
const _: () = assert!(core::mem::offset_of!(TESObjectLIGH, weight_form) == 0xA0);
const _: () = assert!(core::mem::offset_of!(TESObjectLIGH, value_form) == 0xB0);
const _: () = assert!(core::mem::offset_of!(TESObjectLIGH, destructible_object_form) == 0xC0);
const _: () = assert!(core::mem::offset_of!(TESObjectLIGH, equip_type) == 0xD0);

impl RttiType for TESObjectLIGH {
    const RTTI: VariantID = RTTI_TESObjectLIGH;
}

inherit!(TESObjectLIGH : TESBoundAnimObject);
inherit!(TESObjectLIGH => TESFullName, full_name);
inherit!(TESObjectLIGH => TESModelTextureSwap, model_texture_swap);
inherit!(TESObjectLIGH => TESIcon, icon);
inherit!(TESObjectLIGH => BGSMessageIcon, message_icon);
inherit!(TESObjectLIGH => TESWeightForm, weight_form);
inherit!(TESObjectLIGH => TESValueForm, value_form);
inherit!(TESObjectLIGH => BGSDestructibleObjectForm, destructible_object_form);
inherit!(TESObjectLIGH => BGSEquipType, equip_type);

impl crate::re::FormCastable for TESObjectLIGH {
    const TARGET_FORM_TYPE: crate::re::FormType = crate::re::FormType::Light;
}

impl TESObjectLIGH {
    pub const RTTI: VariantID = RTTI_TESObjectLIGH;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObjectLIGH;
    pub const FORMTYPE: crate::re::FormType = crate::re::FormType::Light;

    // override (TESBoundAnimObject)
    // void        InitializeData() override;           // 04
    // bool        Load(TESFile* a_mod) override;       // 06
    // void        SaveGame(BGSSaveFormBuffer* a_buf) override;  // 0E
    // void        LoadGame(BGSLoadFormBuffer* a_buf) override;  // 0F
    // void        InitItemImpl() override;             // 13
    // bool        Activate(...) override;              // 37
    // void        UnClone3D(TESObjectREFR* a_ref) override;     // 41
    // NiAVObject* LoadGraphics(TESObjectREFR* a_ref) override;  // 47
    // NiAVObject* Clone3D(TESObjectREFR* a_ref) override;       // 4A

    // override (BGSEquipType)
    // BGSEquipSlot* GetEquipSlot() const override;     // 04
    // void          SetEquipSlot(BGSEquipSlot* a_slot) override;  // 05

    #[inline(always)]
    pub fn can_be_carried(&self) -> bool {
        self.data.flags.contains(TESLightFlags::kCanCarry)
    }

    #[inline(always)]
    pub fn get_no_flicker(&self) -> bool {
        !self.data.flags.intersects(
            TESLightFlags::kFlicker
                | TESLightFlags::kFlickerSlow
                | TESLightFlags::kPulse
                | TESLightFlags::kPulseSlow,
        )
    }

    crate::relocation_func! {
        pub fn gen_dynamic(
            this: &TESObjectLIGH,
            a_ref: *mut TESObjectREFR,
            a_node: *mut NiNode,
            a_force_dynamic: core::ffi::c_char,
            a_use_light_radius: core::ffi::c_char,
            a_affect_ref_only: core::ffi::c_char
        ) -> *mut NiLight => RelocationID::new(17208, 17610)
    }
}
