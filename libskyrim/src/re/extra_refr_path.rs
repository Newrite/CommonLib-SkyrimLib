use crate::core_util::{EnumSet, inherit};
use crate::offsets::offsets_rtti::RTTI_ExtraRefrPath;
use crate::offsets::offsets_vtable::VTABLE_ExtraRefrPath;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped, NiPoint3};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraRefrPath::PATH_TYPE`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtraRefrPathPathType {
    Translation = 0,
    Spline = 1,
}

core_util::impl_enumset_type!(ExtraRefrPathPathType => u32);

/// C++ `RE::ExtraRefrPath`
#[repr(C)]
pub struct ExtraRefrPath {
    pub base: BSExtraData,                          // 00
    pub start_pos: NiPoint3,                        // 10
    pub start_tangent: NiPoint3,                    // 1C
    pub start_euler: NiPoint3,                      // 28
    pub goal_pos: NiPoint3,                         // 34
    pub goal_tangent: NiPoint3,                     // 40
    pub goal_euler: NiPoint3,                       // 4C
    pub speed: f32,                                 // 58
    pub max_rot_speed: f32,                         // 5C
    pub current_parameter: f32,                     // 60
    pub type_: EnumSet<ExtraRefrPathPathType, u32>, // 64
}

const _: () = assert!(core::mem::size_of::<ExtraRefrPath>() == 0x68);
const _: () = assert!(core::mem::offset_of!(ExtraRefrPath, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraRefrPath, start_pos) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraRefrPath, start_tangent) == 0x1C);
const _: () = assert!(core::mem::offset_of!(ExtraRefrPath, start_euler) == 0x28);
const _: () = assert!(core::mem::offset_of!(ExtraRefrPath, goal_pos) == 0x34);
const _: () = assert!(core::mem::offset_of!(ExtraRefrPath, goal_tangent) == 0x40);
const _: () = assert!(core::mem::offset_of!(ExtraRefrPath, goal_euler) == 0x4C);
const _: () = assert!(core::mem::offset_of!(ExtraRefrPath, speed) == 0x58);
const _: () = assert!(core::mem::offset_of!(ExtraRefrPath, max_rot_speed) == 0x5C);
const _: () = assert!(core::mem::offset_of!(ExtraRefrPath, current_parameter) == 0x60);
const _: () = assert!(core::mem::offset_of!(ExtraRefrPath, type_) == 0x64);

impl RttiType for ExtraRefrPath {
    const RTTI: VariantID = RTTI_ExtraRefrPath;
}

impl ExtraDataTyped for ExtraRefrPath {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::RefrPath;
}

inherit!(ExtraRefrPath : BSExtraData, base);

impl ExtraRefrPath {
    pub const RTTI: VariantID = RTTI_ExtraRefrPath;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraRefrPath;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::RefrPath;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kRefrPath; }

    #[inline(always)]
    pub fn create() -> *mut Self {
        let extra = BSExtraData::create_typed::<Self>(Self::VTABLE[0].address());
        unsafe {
            (*extra).base.next = core::ptr::null_mut();
            (*extra).current_parameter = 0.0;
            (*extra).type_ = ExtraRefrPathPathType::Translation.into();
        }
        extra
    }

    crate::relocation_func! {
        pub fn setup_translation(
            &mut self,
            start_pos: &NiPoint3,
            start_euler: &NiPoint3,
            goal_pos: &NiPoint3,
            end_euler: &NiPoint3,
            speed: f32,
            max_rot_speed: f32
        ) => crate::relocation::RelocationID::new(12666, 12808)
    }
}
