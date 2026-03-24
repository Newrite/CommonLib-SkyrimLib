use crate::core_util::{EnumSet, inherit};
use crate::offsets::offsets_nirtti::NiRTTI_NiControllerSequence;
use crate::offsets::offsets_rtti::RTTI_NiControllerSequence;
use crate::offsets::offsets_vtable::VTABLE_NiControllerSequence;
use crate::re::BSAnimNote;
use crate::re::NiAVObject;
use crate::re::NiBlendInterpolator;
use crate::re::NiControllerManager;
use crate::re::NiDefaultAVObjectPalette;
use crate::re::NiInterpController;
use crate::re::NiInterpolator;
use crate::re::NiObject;
use crate::re::NiPointer;
use crate::re::NiRef;
use crate::re::NiStringPalette;
use crate::re::NiTextKeyExtraData;
use crate::re::NiTimeControllerCycleType;
use crate::re::SimpleArray;
use crate::re::bs_fixed_string::BSFixedString;
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::NiControllerSequence::AnimState`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NiControllerSequenceAnimState {
    Inactive = 0,
    Animating = 1,
    EaseIn = 2,
    EaseOut = 3,
    TransSource = 4,
    TransDest = 5,
    MorphSource = 6,
}

core_util::impl_enumset_type!(NiControllerSequenceAnimState => u32);

/// C++ `RE::NiControllerSequence::InterpArrayItem`
#[repr(C)]
pub struct NiControllerSequenceInterpArrayItem {
    pub interpolator: NiPointer<NiInterpolator>,    // 00
    pub interp_ctlr: NiPointer<NiInterpController>, // 08
    pub blend_interp: *mut NiBlendInterpolator,     // 10
    pub blend_idx: u8,                              // 18
    pub pad19: u8,                                  // 19
    pub pad1a: u16,                                 // 1A
    pub pad1c: u32,                                 // 1C
}

const _: () = assert!(core::mem::size_of::<NiControllerSequenceInterpArrayItem>() == 0x20);
const _: () =
    assert!(core::mem::offset_of!(NiControllerSequenceInterpArrayItem, interpolator) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(NiControllerSequenceInterpArrayItem, interp_ctlr) == 0x08);
const _: () =
    assert!(core::mem::offset_of!(NiControllerSequenceInterpArrayItem, blend_interp) == 0x10);
const _: () =
    assert!(core::mem::offset_of!(NiControllerSequenceInterpArrayItem, blend_idx) == 0x18);

/// C++ `RE::NiControllerSequence::IDTag`
#[repr(C)]
pub struct NiControllerSequenceIDTag {
    pub av_object_name: BSFixedString,  // 00
    pub property_type: BSFixedString,   // 08
    pub ctlr_type: BSFixedString,       // 10
    pub ctlr_id: BSFixedString,         // 18
    pub interpolator_id: BSFixedString, // 20
}

const _: () = assert!(core::mem::size_of::<NiControllerSequenceIDTag>() == 0x28);
const _: () = assert!(core::mem::offset_of!(NiControllerSequenceIDTag, av_object_name) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiControllerSequenceIDTag, property_type) == 0x08);
const _: () = assert!(core::mem::offset_of!(NiControllerSequenceIDTag, ctlr_type) == 0x10);
const _: () = assert!(core::mem::offset_of!(NiControllerSequenceIDTag, ctlr_id) == 0x18);
const _: () = assert!(core::mem::offset_of!(NiControllerSequenceIDTag, interpolator_id) == 0x20);

/// C++ `RE::NiControllerSequence`
#[repr(C)]
pub struct NiControllerSequence {
    pub base: NiObject,                                                 // 00
    pub name: BSFixedString,                                            // 10
    pub array_size: u32,                                                // 18
    pub array_grow_by: u32,                                             // 1C
    pub interp_array: SimpleArray<NiControllerSequenceInterpArrayItem>, // 20
    pub id_tag_array: SimpleArray<NiControllerSequenceIDTag>,           // 28
    pub seq_weight: f32,                                                // 30
    pub pad34: u32,                                                     // 34
    pub text_keys: NiPointer<NiTextKeyExtraData>,                       // 38
    pub cycle_type: EnumSet<NiTimeControllerCycleType, u32>,            // 40
    pub frequency: f32,                                                 // 44
    pub begin_key_time: f32,                                            // 48
    pub end_key_time: f32,                                              // 4C
    pub last_time: f32,                                                 // 50
    pub weighted_last_time: f32,                                        // 54
    pub last_scaled_time: f32,                                          // 58
    pub pad5c: u32,                                                     // 5C
    pub owner: *mut NiControllerManager,                                // 60
    pub state: EnumSet<NiControllerSequenceAnimState, u32>,             // 68
    pub offset: f32,                                                    // 6C
    pub start_time: f32,                                                // 70
    pub end_time: f32,                                                  // 74
    pub dest_frame: f32,                                                // 78
    pub pad7c: u32,                                                     // 7C
    pub partner_sequence: *mut NiControllerSequence,                    // 80
    pub accum_root_name: BSFixedString,                                 // 88
    pub accum_root: *mut NiAVObject,                                    // 90
    pub deprecated_string_palette: NiPointer<NiStringPalette>,          // 98
    pub cur_anim_n_idx: i16,                                            // A0
    pub unk_a2: u16,                                                    // A2
    pub unk_a4: u32,                                                    // A4
    pub anim_notes: SimpleArray<NiPointer<BSAnimNote>>,                 // A8
    pub num_notes: u16,                                                 // B0
    pub removable_objects: bool,                                        // B2
    pub unk_b3: u8,                                                     // B3
    pub unk_b4: u32,                                                    // B4
}

const _: () = assert!(core::mem::size_of::<NiControllerSequence>() == 0xB8);
const _: () = assert!(core::mem::offset_of!(NiControllerSequence, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiControllerSequence, name) == 0x10);
const _: () = assert!(core::mem::offset_of!(NiControllerSequence, interp_array) == 0x20);
const _: () = assert!(core::mem::offset_of!(NiControllerSequence, id_tag_array) == 0x28);
const _: () = assert!(core::mem::offset_of!(NiControllerSequence, text_keys) == 0x38);
const _: () = assert!(core::mem::offset_of!(NiControllerSequence, cycle_type) == 0x40);
const _: () = assert!(core::mem::offset_of!(NiControllerSequence, owner) == 0x60);
const _: () = assert!(core::mem::offset_of!(NiControllerSequence, state) == 0x68);
const _: () = assert!(core::mem::offset_of!(NiControllerSequence, partner_sequence) == 0x80);
const _: () =
    assert!(core::mem::offset_of!(NiControllerSequence, deprecated_string_palette) == 0x98);
const _: () = assert!(core::mem::offset_of!(NiControllerSequence, anim_notes) == 0xA8);
const _: () = assert!(core::mem::offset_of!(NiControllerSequence, num_notes) == 0xB0);
const _: () = assert!(core::mem::offset_of!(NiControllerSequence, removable_objects) == 0xB2);

impl RttiType for NiControllerSequence {
    const RTTI: VariantID = RTTI_NiControllerSequence;
}

impl NiRef for NiControllerSequence {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(NiControllerSequence : NiObject);

impl NiControllerSequence {
    pub const RTTI: VariantID = RTTI_NiControllerSequence;
    pub const NI_RTTI: VariantID = NiRTTI_NiControllerSequence;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiControllerSequence;

    // override (NiObject)
    // const NiRTTI* GetRTTI() const override;                            // 02
    // NiObject*     CreateClone(NiCloningProcess& a_cloning) override;   // 17
    // void          LoadBinary(NiStream& a_stream) override;             // 18
    // void          LinkObject(NiStream& a_stream) override;             // 19
    // bool          RegisterStreamables(NiStream& a_stream) override;    // 1A
    // void          SaveBinary(NiStream& a_stream) override;             // 1B
    // bool          IsEqual(NiObject* a_object) override;                // 1C
    // void          ProcessClone(NiCloningProcess& a_cloning) override;  // 1D
    // void          PostLinkObject(NiStream& a_stream) override;         // 1E

    virtual_method! {
        pub const VFUNC_DEACTIVATE: usize = 0x25;
        pub fn deactivate(ease_out_time: f32, transition: bool) -> bool
    }

    crate::relocation_func! {
        pub fn activate(
            &mut self,
            interp_index: u8,
            max_offset: bool,
            seq_weight: f32,
            ease_in_time: f32,
            partner_sequence: *mut NiControllerSequence,
            transition: bool
        ) -> bool => RelocationID::new(70882, 72463)
    }

    #[inline(always)]
    pub const fn animating(&self) -> bool {
        self.state.underlying() == NiControllerSequenceAnimState::Animating as u32
    }

    #[inline(always)]
    pub const fn inactive(&self) -> bool {
        self.state.underlying() == NiControllerSequenceAnimState::Inactive as u32
    }

    crate::relocation_func! {
        pub fn set_phase(&mut self, phase: f32, arg2: bool) => RelocationID::new(70860, 72439)
    }

    crate::relocation_func! {
        pub fn resolve_transform_interpolators(
            &mut self,
            root: *mut NiAVObject,
            object_palette: *mut NiDefaultAVObjectPalette,
            formal: u32
        ) -> bool => RelocationID::new(70909, 72497)
    }
}
