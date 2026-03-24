use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiNode;
use crate::offsets::offsets_rtti::RTTI_NiNode;
use crate::offsets::offsets_vtable::VTABLE_NiNode;
use crate::re::NiAVObject;
use crate::re::NiPointer;
use crate::re::NiTObjectArray;
use crate::re::NiUpdateData;
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset};

/// Cross-runtime honest prefix of C++ `RE::NiNode`.
///
/// The `children` tail starts at `0x110` on SE/AE and `0x138` on VR, so the
/// main struct keeps only the stable `NiAVObject` prefix.
#[repr(C)]
pub struct NiNode {
    pub base: NiAVObject, // 00
}

const _: () = assert!(core::mem::size_of::<NiNode>() == 0x110);
const _: () = assert!(core::mem::offset_of!(NiNode, base) == 0x00);

impl RttiType for NiNode {
    const RTTI: VariantID = RTTI_NiNode;
}

impl crate::re::ni_ref_object::NiRef for NiNode {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(NiNode : NiAVObject);

impl NiNode {
    pub const RTTI: VariantID = RTTI_NiNode;
    pub const NI_RTTI: VariantID = NiRTTI_NiNode;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiNode;
    pub const FULL_SIZE: VariantOffset = VariantOffset::new_se_ae(0x128, 0x150);
    pub const CHILDREN_OFFSET: VariantOffset = VariantOffset::new_se_ae(0x110, 0x138);

    // override (NiAVObject)
    // const NiRTTI* GetRTTI() const override;                            // 02
    // NiNode*       AsNode() override;                                   // 03
    // NiObject*     CreateClone(NiCloningProcess& a_cloning) override;   // 17
    // void          LoadBinary(NiStream& a_stream) override;             // 18
    // void          LinkObject(NiStream& a_stream) override;             // 19
    // bool          RegisterStreamables(NiStream& a_stream) override;    // 1A
    // void          SaveBinary(NiStream& a_stream) override;             // 1B
    // bool          IsEqual(NiObject* a_object) override;                // 1C
    // void          ProcessClone(NiCloningProcess& a_cloning) override;  // 1D
    // void          UpdateControllers(NiUpdateData& a_data) override;    // 25
    // void          PerformOp(PerformOpFunc& a_func) override;           // 26
    // void          AttachProperty(NiAlphaProperty* a_property) override;  // 27
    // NiAVObject*   GetObjectByName(const BSFixedString& a_name) override;  // 2A
    // void          SetSelectiveUpdateFlags(...) override;               // 2B
    // void          UpdateDownwardPass(NiUpdateData& a_data, std::uint32_t a_arg2) override;  // 2C
    // void          UpdateSelectedDownwardPass(NiUpdateData& a_data, std::uint32_t a_arg2) override;  // 2D
    // void          UpdateRigidDownwardPass(NiUpdateData& a_data, std::uint32_t a_arg2) override;  // 2E
    // void          UpdateWorldBound() override;                         // 2F
    // void          UpdateTransformAndBounds(NiUpdateData& a_data) override;  // 31
    // void          OnVisible(NiCullingProcess& a_process, std::int32_t a_alphaGroupIndex) override;  // 34

    crate::relocation_func! {
        pub fn ctor(&mut self, a_arr_buf_len: u16) -> *mut NiNode => RelocationID::new(68936, 70287)
    }

    crate::runtime_data_accessor! {
        fn children_raw() -> NiTObjectArray<NiPointer<NiAVObject>> {
            se_ae: 0x110,
            vr: 0x138
        }
    }

    #[inline(always)]
    pub fn get_children(&self) -> &NiTObjectArray<NiPointer<NiAVObject>> {
        crate::runtime_assert_size!(NiTObjectArray<NiPointer<NiAVObject>>, se_ae: 0x18, vr: 0x18);
        self.children_raw()
    }

    #[inline]
    pub fn create(a_arr_buf_len: u16) -> *mut NiNode {
        let size = Self::FULL_SIZE.offset();
        unsafe {
            let memory = crate::ffi::commonlib_malloc(size).cast::<u8>();
            assert!(!memory.is_null(), "NiNode::Create allocation failed");
            core::ptr::write_bytes(memory, 0, size);
            let node = memory.cast::<NiNode>();
            (*node).ctor(a_arr_buf_len);
            node
        }
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_ATTACH_CHILD: VariantOffset = VariantOffset::new_se_ae(0x35, 0x36);
        pub fn attach_child(&mut self, a_child: *mut NiAVObject, a_first_avail: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_INSERT_CHILD_AT: VariantOffset = VariantOffset::new_se_ae(0x36, 0x37);
        pub fn insert_child_at(&mut self, a_idx: u32, a_child: *mut NiAVObject)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_DETACH_CHILD1: VariantOffset = VariantOffset::new_se_ae(0x37, 0x38);
        pub fn detach_child1(&mut self, a_child: *mut NiAVObject, a_child_out: &mut NiPointer<NiAVObject>)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_DETACH_CHILD2: VariantOffset = VariantOffset::new_se_ae(0x38, 0x39);
        pub fn detach_child2(&mut self, a_child: *mut NiAVObject)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_DETACH_CHILD_AT1: VariantOffset = VariantOffset::new_se_ae(0x39, 0x3A);
        pub fn detach_child_at1(&mut self, a_idx: u32, a_child_out: &mut NiPointer<NiAVObject>)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_DETACH_CHILD_AT2: VariantOffset = VariantOffset::new_se_ae(0x3A, 0x3B);
        pub fn detach_child_at2(&mut self, a_idx: u32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_AT1: VariantOffset = VariantOffset::new_se_ae(0x3B, 0x3C);
        pub fn set_at1(&mut self, a_idx: u32, a_child: *mut NiAVObject, a_child_out: &mut NiPointer<NiAVObject>)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_AT2: VariantOffset = VariantOffset::new_se_ae(0x3C, 0x3D);
        pub fn set_at2(&mut self, a_idx: u32, a_child: *mut NiAVObject)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_UPWARD_PASS: VariantOffset = VariantOffset::new_se_ae(0x3D, 0x3E);
        pub fn update_upward_pass(&mut self, a_data: *mut NiUpdateData)
    }

    #[inline(always)]
    pub fn detach_child(&mut self, a_child: *mut NiAVObject) {
        self.detach_child2(a_child);
    }

    #[inline(always)]
    pub fn detach_child_with_out(
        &mut self,
        a_child: *mut NiAVObject,
        a_child_out: &mut NiPointer<NiAVObject>,
    ) {
        self.detach_child1(a_child, a_child_out);
    }

    #[inline(always)]
    pub fn detach_child_at(&mut self, a_idx: u32) {
        self.detach_child_at2(a_idx);
    }

    #[inline(always)]
    pub fn detach_child_at_with_out(
        &mut self,
        a_idx: u32,
        a_child_out: &mut NiPointer<NiAVObject>,
    ) {
        self.detach_child_at1(a_idx, a_child_out);
    }

    #[inline(always)]
    pub fn set_at(&mut self, a_idx: u32, a_child: *mut NiAVObject) {
        self.set_at2(a_idx, a_child);
    }

    #[inline(always)]
    pub fn set_at_with_out(
        &mut self,
        a_idx: u32,
        a_child: *mut NiAVObject,
        a_child_out: &mut NiPointer<NiAVObject>,
    ) {
        self.set_at1(a_idx, a_child, a_child_out);
    }
}

pub trait NiNodeExt {
    fn get_children(&self) -> &NiTObjectArray<NiPointer<NiAVObject>>;
    fn attach_child(&mut self, a_child: *mut NiAVObject, a_first_avail: bool);
    fn insert_child_at(&mut self, a_idx: u32, a_child: *mut NiAVObject);
    fn detach_child(&mut self, a_child: *mut NiAVObject);
    fn detach_child_with_out(
        &mut self,
        a_child: *mut NiAVObject,
        a_child_out: &mut NiPointer<NiAVObject>,
    );
    fn detach_child_at(&mut self, a_idx: u32);
    fn detach_child_at_with_out(&mut self, a_idx: u32, a_child_out: &mut NiPointer<NiAVObject>);
    fn set_at(&mut self, a_idx: u32, a_child: *mut NiAVObject);
    fn set_at_with_out(
        &mut self,
        a_idx: u32,
        a_child: *mut NiAVObject,
        a_child_out: &mut NiPointer<NiAVObject>,
    );
    fn update_upward_pass(&mut self, a_data: *mut NiUpdateData);
}

impl<T: AsRef<NiNode> + AsMut<NiNode>> NiNodeExt for T {
    #[inline(always)]
    fn get_children(&self) -> &NiTObjectArray<NiPointer<NiAVObject>> {
        self.as_ref().get_children()
    }

    #[inline(always)]
    fn attach_child(&mut self, a_child: *mut NiAVObject, a_first_avail: bool) {
        NiNode::attach_child(self.as_mut(), a_child, a_first_avail)
    }

    #[inline(always)]
    fn insert_child_at(&mut self, a_idx: u32, a_child: *mut NiAVObject) {
        NiNode::insert_child_at(self.as_mut(), a_idx, a_child)
    }

    #[inline(always)]
    fn detach_child(&mut self, a_child: *mut NiAVObject) {
        NiNode::detach_child(self.as_mut(), a_child)
    }

    #[inline(always)]
    fn detach_child_with_out(
        &mut self,
        a_child: *mut NiAVObject,
        a_child_out: &mut NiPointer<NiAVObject>,
    ) {
        NiNode::detach_child_with_out(self.as_mut(), a_child, a_child_out)
    }

    #[inline(always)]
    fn detach_child_at(&mut self, a_idx: u32) {
        NiNode::detach_child_at(self.as_mut(), a_idx)
    }

    #[inline(always)]
    fn detach_child_at_with_out(&mut self, a_idx: u32, a_child_out: &mut NiPointer<NiAVObject>) {
        NiNode::detach_child_at_with_out(self.as_mut(), a_idx, a_child_out)
    }

    #[inline(always)]
    fn set_at(&mut self, a_idx: u32, a_child: *mut NiAVObject) {
        NiNode::set_at(self.as_mut(), a_idx, a_child)
    }

    #[inline(always)]
    fn set_at_with_out(
        &mut self,
        a_idx: u32,
        a_child: *mut NiAVObject,
        a_child_out: &mut NiPointer<NiAVObject>,
    ) {
        NiNode::set_at_with_out(self.as_mut(), a_idx, a_child, a_child_out)
    }

    #[inline(always)]
    fn update_upward_pass(&mut self, a_data: *mut NiUpdateData) {
        NiNode::update_upward_pass(self.as_mut(), a_data)
    }
}
