use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_BSFaceGenNiNode;
use crate::offsets::offsets_rtti::RTTI_BSFaceGenNiNode;
use crate::offsets::offsets_vtable::VTABLE_BSFaceGenNiNode;
use crate::re::ActorHandle;
use crate::re::BSFaceGenAnimationData;
use crate::re::NiMatrix3;
use crate::re::NiNode;
use crate::re::NiPointer;
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::BSFaceGenNiNode::RUNTIME_DATA`
#[repr(C)]
pub struct BSFaceGenNiNodeRuntimeData {
    pub base_rotation: NiMatrix3,                          // 00
    pub pad14c: u32,                                       // 24
    pub animation_data: NiPointer<BSFaceGenAnimationData>, // 28
    pub last_time: f32,                                    // 30
    pub unk15c: ActorHandle,                               // 34
    pub flags: u16,                                        // 38
    pub pad162: u16,                                       // 3A
    pub pad164: u32,                                       // 3C
}

const _: () = assert!(core::mem::size_of::<BSFaceGenNiNodeRuntimeData>() == 0x40);
const _: () = assert!(core::mem::offset_of!(BSFaceGenNiNodeRuntimeData, base_rotation) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSFaceGenNiNodeRuntimeData, pad14c) == 0x24);
const _: () = assert!(core::mem::offset_of!(BSFaceGenNiNodeRuntimeData, animation_data) == 0x28);
const _: () = assert!(core::mem::offset_of!(BSFaceGenNiNodeRuntimeData, last_time) == 0x30);
const _: () = assert!(core::mem::offset_of!(BSFaceGenNiNodeRuntimeData, unk15c) == 0x34);
const _: () = assert!(core::mem::offset_of!(BSFaceGenNiNodeRuntimeData, flags) == 0x38);

/// Cross-runtime honest prefix of C++ `RE::BSFaceGenNiNode`.
///
/// `NiNode` already becomes runtime-divergent before this child's own tail, so
/// the shared prefix stops at the same `0x110` boundary used by CommonLib's
/// cross-runtime layout.
#[repr(C)]
pub struct BSFaceGenNiNode {
    pub base: NiNode, // 00
}

const _: () = assert!(core::mem::size_of::<BSFaceGenNiNode>() == 0x110);
const _: () = assert!(core::mem::offset_of!(BSFaceGenNiNode, base) == 0x00);

impl RttiType for BSFaceGenNiNode {
    const RTTI: VariantID = RTTI_BSFaceGenNiNode;
}

impl crate::re::ni_ref_object::NiRef for BSFaceGenNiNode {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(BSFaceGenNiNode : NiNode);

impl BSFaceGenNiNode {
    pub const RTTI: VariantID = RTTI_BSFaceGenNiNode;
    pub const NI_RTTI: VariantID = NiRTTI_BSFaceGenNiNode;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSFaceGenNiNode;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new_se_ae(0x128, 0x150);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new_se_ae(0x168, 0x190);

    // override (NiNode)
    // const NiRTTI* GetRTTI() const override;                           // 02
    // NiObject*     CreateClone(NiCloningProcess& a_cloning) override;  // 17
    // void          UpdateDownwardPass(NiUpdateData& a_data, std::uint32_t a_arg2) override;  // 2C (flat only)

    crate::runtime_data_accessor! {
        fn runtime_data() -> BSFaceGenNiNodeRuntimeData {
            se_ae: 0x128,
            vr: 0x150
        }
    }

    crate::runtime_data_mut_accessor! {
        fn runtime_data_mut() -> BSFaceGenNiNodeRuntimeData {
            se_ae: 0x128,
            vr: 0x150
        }
    }

    #[inline(always)]
    pub fn get_runtime_data(&self) -> &BSFaceGenNiNodeRuntimeData {
        crate::runtime_assert_size!(BSFaceGenNiNodeRuntimeData, se_ae: 0x40, vr: 0x40);
        self.runtime_data()
    }

    #[inline(always)]
    pub fn get_runtime_data_mut(&mut self) -> &mut BSFaceGenNiNodeRuntimeData {
        crate::runtime_assert_size!(BSFaceGenNiNodeRuntimeData, se_ae: 0x40, vr: 0x40);
        self.runtime_data_mut()
    }

    #[inline(always)]
    pub fn get_animation_data(&self) -> *mut BSFaceGenAnimationData {
        self.get_runtime_data().animation_data.get()
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_FIX_SKIN_INSTANCES: VariantOffset = VariantOffset::new_se_ae(0x3E, 0x3F);
        pub fn fix_skin_instances(&mut self, a_skeleton: *mut NiNode, a_arg2: bool)
    }
}
