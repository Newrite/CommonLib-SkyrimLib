use bitflags::bitflags;

use crate::offsets::offsets_rtti::RTTI_Character;
use crate::offsets::offsets_vtable::VTABLE_Character;
use crate::re::{
    Actor, BGSLoadFormBuffer, BGSSaveFormBuffer, BSFaceGenAnimationData, BSFaceGenNiNode,
    BSTSmartPointer, BipedAnim, FormCastable, FormType,
};
use crate::relocation::{RttiType, VariantID};

bitflags! {
    /// C++ `RE::Character::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CharacterRecordFlags: u32 {
        const DELETED = 1 << 5;
        const STARTS_DEAD = 1 << 9;
        const PERSISTENT = 1 << 10;
        const INITIALLY_DISABLED = 1 << 11;
        const IGNORED = 1 << 12;
        const NO_AI_ACQUIRE = 1 << 25;
        const DONT_HAVOK_SETTLE = 1 << 29;
    }
}

/// C++ `RE::Character`
#[repr(C)]
pub struct Character {
    pub base: Actor, // 00
}

const _: () = assert!(core::mem::size_of::<Character>() == 0x80);
const _: () = assert!(core::mem::offset_of!(Character, base) == 0x00);

impl RttiType for Character {
    const RTTI: VariantID = RTTI_Character;
}

impl FormCastable for Character {
    const TARGET_FORM_TYPE: FormType = FormType::ActorCharacter;
}

core_util::inherit!(Character : Actor, base);

impl Character {
    pub const RTTI: VariantID = RTTI_Character;
    pub const VTABLE: &'static [VariantID] = &VTABLE_Character;
    pub const FORMTYPE: FormType = FormType::ActorCharacter;

    // override (Actor)
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_SAVE_GAME: usize = 0x0E;
        pub fn save_game(buf: *mut BGSSaveFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD_GAME: usize = 0x0F;
        pub fn load_game(buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_LOAD_GAME: usize = 0x10;
        pub fn init_load_game(buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_FINISH_LOAD_GAME: usize = 0x11;
        pub fn finish_load_game(buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_REVERT: usize = 0x12;
        pub fn revert(buf: *mut BGSLoadFormBuffer)
    }

    crate::virtual_method! {
        pub const VFUNC_PREDESTROY: usize = 0x3B;
        pub fn predestroy()
    }

    crate::virtual_method! {
        pub const VFUNC_IS_CHILD: usize = 0x5E;
        pub fn is_child(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_FACE_NODE_SKINNED: usize = 0x61;
        pub fn get_face_node_skinned(&mut self) -> *mut BSFaceGenNiNode
    }

    crate::virtual_method! {
        pub const VFUNC_GET_FACE_GEN_ANIMATION_DATA: usize = 0x63;
        pub fn get_face_gen_animation_data(&mut self) -> *mut BSFaceGenAnimationData
    }

    crate::virtual_method! {
        pub const VFUNC_SET_BIPED: usize = 0x81;
        pub fn set_biped(&mut self, biped: &BSTSmartPointer<BipedAnim>)
    }

    // TODO: `Character.h` has no matching `.cpp`, and the vendored source only proves the
    // flat-only override block (`0x0C0..0x120`) plus fresh slots `Unk_128` / `Unk_129` through
    // header comments. Add those methods once a source-backed verify pass confirms the correct
    // cross-runtime vtable indices instead of guessing the VR numbering.
}

pub trait CharacterExt {
    fn dtor(&mut self);
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer);
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer);
    fn init_load_game(&mut self, buf: *mut BGSLoadFormBuffer);
    fn finish_load_game(&mut self, buf: *mut BGSLoadFormBuffer);
    fn revert(&mut self, buf: *mut BGSLoadFormBuffer);
    fn predestroy(&mut self);
    fn is_child(&self) -> bool;
    fn get_face_node_skinned(&mut self) -> *mut BSFaceGenNiNode;
    fn get_face_gen_animation_data(&mut self) -> *mut BSFaceGenAnimationData;
    fn set_biped(&mut self, biped: &BSTSmartPointer<BipedAnim>);
}

impl<T> CharacterExt for T
where
    T: AsRef<Character> + AsMut<Character>,
{
    #[inline(always)]
    fn dtor(&mut self) {
        Character::dtor(self.as_mut())
    }

    #[inline(always)]
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer) {
        Character::save_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        Character::load_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn init_load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        Character::init_load_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn finish_load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        Character::finish_load_game(self.as_mut(), buf)
    }

    #[inline(always)]
    fn revert(&mut self, buf: *mut BGSLoadFormBuffer) {
        Character::revert(self.as_mut(), buf)
    }

    #[inline(always)]
    fn predestroy(&mut self) {
        Character::predestroy(self.as_mut())
    }

    #[inline(always)]
    fn is_child(&self) -> bool {
        Character::is_child(self.as_ref())
    }

    #[inline(always)]
    fn get_face_node_skinned(&mut self) -> *mut BSFaceGenNiNode {
        Character::get_face_node_skinned(self.as_mut())
    }

    #[inline(always)]
    fn get_face_gen_animation_data(&mut self) -> *mut BSFaceGenAnimationData {
        Character::get_face_gen_animation_data(self.as_mut())
    }

    #[inline(always)]
    fn set_biped(&mut self, biped: &BSTSmartPointer<BipedAnim>) {
        Character::set_biped(self.as_mut(), biped)
    }
}
