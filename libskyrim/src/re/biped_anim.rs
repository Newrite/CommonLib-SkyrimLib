//! Translation of `RE::BipedAnim.h`.

use crate::relocation::RelocationID;
use crate::relocation_func;
use core_util::inherit;

use crate::re::biped_objects::BIPED_OBJECTS_TOTAL;
use crate::re::bs_intrusive_ref_counted::BSIntrusiveRefCounted;
use crate::re::bs_pointer_handle::ObjectRefHandle;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::ni_smart_pointer::NiPointer;
use crate::re::weapon_animation_graph_manager_holder::WeaponAnimationGraphManagerHolder;

// Forward-declared pointer-only types
use crate::re::bgs_texture_set::BGSTextureSet;
use crate::re::ni_av_object::NiAVObject;
use crate::re::ni_node::NiNode;
use crate::re::tes_form::TESForm;
use crate::re::tes_model::TESModel;
use crate::re::tes_object_arma::TESObjectARMA;

/// C++ `RE::BIPOBJECT`
///
/// Represents a single biped object slot (armor piece, weapon, etc.)
/// attached to an actor's skeleton.
#[repr(C)]
pub struct BIPOBJECT {
    pub item: *mut TESForm,                                     // 00
    pub addon: *mut TESObjectARMA,                              // 08
    pub part: *mut TESModel,                                    // 10
    pub skin_texture: *mut BGSTextureSet,                       // 18
    pub part_clone: NiPointer<NiAVObject>,                      // 20
    pub unk28: u64,                                             // 28 - same as AIProcess::Data0B8
    pub unk30: u64,                                             // 30
    pub unk38: u64,                                             // 38
    pub unk40: u64,                                             // 40
    pub unk48: u64,                                             // 48
    pub unk50: u64,                                             // 50
    pub unk58: u64,                                             // 58
    pub weapon_manager: *mut WeaponAnimationGraphManagerHolder, // 60 - BSTSmartPointer
    pub unk68: u64,                                             // 68
    pub unk70: *mut core::ffi::c_void,                          // 70
}

impl BSTSmartPointerIntrusiveRefCountable for BipedAnim {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        crate::relocation_func! {
            fn dtor_impl(this: *mut BipedAnim) => RelocationID::new(15491, 15656)
        }
        dtor_impl(self as *const Self as *mut Self);
    }
}

const _: () = assert!(core::mem::size_of::<BIPOBJECT>() == 0x78);

/// C++ `RE::BipedAnim`
///
/// Manages the biped animation objects for an actor,
/// including both active and buffered (pending) equipment slots.
#[repr(C)]
pub struct BipedAnim {
    pub base: BSIntrusiveRefCounted,                        // 0000
    pub pad0004: u32,                                       // 0004
    pub root: *mut NiNode,                                  // 0008
    pub objects: [BIPOBJECT; BIPED_OBJECTS_TOTAL],          // 0010
    pub buffered_objects: [BIPOBJECT; BIPED_OBJECTS_TOTAL], // 13C0
    pub actor_ref: ObjectRefHandle,                         // 2770
    pub pad2774: u32,                                       // 2774
}

const _: () = assert!(core::mem::size_of::<BipedAnim>() == 0x2778);

inherit!(BipedAnim : BSIntrusiveRefCounted);

impl BipedAnim {
    // No RTTI / VTABLE Р В Р вЂ Р В РІР‚С™Р Р†Р вЂљРЎСљ BipedAnim is not a virtual class.

    // RELOCATION_ID SE: 15494, AE: 15659
    relocation_func! {
        pub fn remove_all_parts(this: &mut BipedAnim) => RelocationID::new(15494, 15659)
    }

    // RELOCATION_ID SE: 15518, AE: 15695
    relocation_func! {
        pub fn get_shield_object(this: &mut BipedAnim) -> *mut BIPOBJECT => RelocationID::new(15518, 15695)
    }

    // RELOCATION_ID SE: 15491, AE: 15656
    #[allow(dead_code)]
    fn dtor(this: &mut BipedAnim) {
        // Private method Р В Р вЂ Р В РІР‚С™Р Р†Р вЂљРЎСљ called by ~BipedAnim() destructor.
        // The actual destructor also zeroes memory after Dtor().
        crate::relocation_func! {
            fn dtor_impl(this: &mut BipedAnim) => RelocationID::new(15491, 15656)
        }
        dtor_impl(this);
    }
}
