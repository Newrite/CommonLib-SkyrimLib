use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::offsets::offsets_rtti::{
    RTTI_MagicTarget, RTTI_MagicTarget__ForEachActiveEffectVisitor,
    RTTI_MagicTarget__IPostCreationModification,
};
use crate::offsets::offsets_vtable::{
    VTABLE_MagicTarget, VTABLE_MagicTarget__ForEachActiveEffectVisitor,
    VTABLE_MagicTarget__IPostCreationModification,
};
use crate::re::ActiveEffect;
use crate::re::Actor;
use crate::re::ActorHandle;
use crate::re::BGSKeyword;
use crate::re::Effect;
use crate::re::EffectArchetypeId;
use crate::re::EffectSetting;
use crate::re::MagicItem;
use crate::re::NiPoint3;
use crate::re::TESBoundObject;
use crate::re::TESObjectREFR;
use crate::re::bs_container::BSContainerForEachResult;
use crate::re::bssimple_list::BSSimpleList;
use crate::re::magic_system::CastingSource;
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::runtime;
use crate::virtual_method;
use windows_sys::Win32::System::Threading::{
    FLS_OUT_OF_INDEXES, FlsAlloc, FlsGetValue, FlsSetValue,
};

type VisitorCallback =
    unsafe extern "C" fn(*mut c_void, *mut ActiveEffect) -> BSContainerForEachResult;

/// C++ `RE::MagicTarget::ForEachActiveEffectVisitor`
#[repr(C)]
pub struct MagicTargetForEachActiveEffectVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<MagicTargetForEachActiveEffectVisitor>() == 0x8);

impl RttiType for MagicTargetForEachActiveEffectVisitor {
    const RTTI: VariantID = RTTI_MagicTarget__ForEachActiveEffectVisitor;
}

impl MagicTargetForEachActiveEffectVisitor {
    pub const RTTI: VariantID = RTTI_MagicTarget__ForEachActiveEffectVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicTarget__ForEachActiveEffectVisitor;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_ACCEPT: usize = 0x01;
        pub fn accept(effect: *mut ActiveEffect) -> BSContainerForEachResult
    }
}

/// C++ `RE::MagicTarget::IPostCreationModification`
#[repr(C)]
pub struct MagicTargetPostCreationModification {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<MagicTargetPostCreationModification>() == 0x8);

impl RttiType for MagicTargetPostCreationModification {
    const RTTI: VariantID = RTTI_MagicTarget__IPostCreationModification;
}

impl MagicTargetPostCreationModification {
    pub const RTTI: VariantID = RTTI_MagicTarget__IPostCreationModification;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicTarget__IPostCreationModification;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_MODIFY_ACTIVE_EFFECT: usize = 0x01;
        pub fn modify_active_effect(effect: *mut ActiveEffect)
    }
}

/// C++ `RE::MagicTarget::ResultsCollector`
#[repr(C)]
pub struct MagicTargetResultsCollector {
    pub target: *mut MagicTarget,   // 00
    pub caster: *mut Actor,         // 08
    pub magic_item: *mut MagicItem, // 10
    pub immunities: u16,            // 18
    pub non_trivials: u16,          // 1A
    pub pad1c: u32,                 // 1C
}

const _: () = assert!(core::mem::size_of::<MagicTargetResultsCollector>() == 0x20);

/// C++ `RE::MagicTarget::SpellDispelData`
#[repr(C)]
pub struct MagicTargetSpellDispelData {
    pub spell: *mut MagicItem,                 // 00
    pub caster: ActorHandle,                   // 08
    pub pad0c: u32,                            // 0C
    pub active_effect: *mut ActiveEffect,      // 10 - BSTSmartPointer<ActiveEffect>
    pub next: *mut MagicTargetSpellDispelData, // 18
}

const _: () = assert!(core::mem::size_of::<MagicTargetSpellDispelData>() == 0x20);

/// C++ `RE::MagicTarget::AddTargetData`
#[repr(C)]
pub struct MagicTargetAddTargetData {
    pub caster: *mut TESObjectREFR,  // 00
    pub magic_item: *mut MagicItem,  // 08
    pub effect: *mut Effect,         // 10
    pub source: *mut TESBoundObject, // 18
    pub post_creation_callback: *mut MagicTargetPostCreationModification, // 20
    pub results_collector: *mut MagicTargetResultsCollector, // 28
    pub explosion_point: NiPoint3,   // 30
    pub magnitude: f32,              // 3C
    pub power: f32,                  // 40
    pub casting_source: CastingSource, // 44
    pub area_target: bool,           // 48
    pub dual_casted: bool,           // 49
    pub pad4a: u16,                  // 4A
    pub pad4c: u32,                  // 4C
}

const _: () = assert!(core::mem::size_of::<MagicTargetAddTargetData>() == 0x50);

#[repr(C)]
struct MagicTargetVisitEffectsVisitor {
    base: MagicTargetForEachActiveEffectVisitor, // 00
    ctx: *mut c_void,                            // 08
    callback: VisitorCallback,                   // 10
}

unsafe extern "C" fn visit_effects_visitor_dtor(_this: *mut MagicTargetVisitEffectsVisitor) {}

unsafe extern "C" fn visit_effects_visitor_accept(
    this: *mut MagicTargetVisitEffectsVisitor,
    effect: *mut ActiveEffect,
) -> BSContainerForEachResult {
    unsafe { ((*this).callback)((*this).ctx, effect) }
}

struct MagicTargetVisitEffectsVisitorVTable([*const (); 2]);

unsafe impl Sync for MagicTargetVisitEffectsVisitorVTable {}

static MAGIC_TARGET_VISIT_EFFECTS_VISITOR_VTABLE: MagicTargetVisitEffectsVisitorVTable =
    MagicTargetVisitEffectsVisitorVTable([
        visit_effects_visitor_dtor as *const (),
        visit_effects_visitor_accept as *const (),
    ]);

unsafe extern "C" fn visit_effects_closure<F>(
    ctx: *mut c_void,
    effect: *mut ActiveEffect,
) -> BSContainerForEachResult
where
    F: FnMut(*mut ActiveEffect) -> BSContainerForEachResult,
{
    unsafe { (&mut *(ctx as *mut F))(effect) }
}

const MAGIC_TARGET_ACTIVE_EFFECT_FLS_INITIALIZING: u32 = FLS_OUT_OF_INDEXES - 1;

static MAGIC_TARGET_ACTIVE_EFFECT_FLS_INDEX: AtomicU32 = AtomicU32::new(FLS_OUT_OF_INDEXES);

unsafe extern "system" fn active_effect_snapshot_fls_dtor(lpflsdata: *const c_void) {
    if !lpflsdata.is_null() {
        unsafe {
            drop(Box::from_raw(
                lpflsdata as *mut BSSimpleList<*mut ActiveEffect>,
            ));
        }
    }
}

/// C++ `RE::MagicTarget`
#[repr(C)]
pub struct MagicTarget {
    pub vtable: *const usize,                                     // 00
    pub post_update_dispel_list: *mut MagicTargetSpellDispelData, // 08
    pub pad12: u16,                                               // 12
    pub pad14: u32,                                               // 14
}

const _: () = assert!(core::mem::size_of::<MagicTarget>() == 0x18);
const _: () = assert!(core::mem::offset_of!(MagicTarget, post_update_dispel_list) == 0x08);

impl RttiType for MagicTarget {
    const RTTI: VariantID = RTTI_MagicTarget;
}

impl AsRef<MagicTarget> for MagicTarget {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<MagicTarget> for MagicTarget {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl MagicTarget {
    pub const RTTI: VariantID = RTTI_MagicTarget;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicTarget;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_ADD_TARGET: usize = 0x01;
        pub fn add_target(target_data: *mut MagicTargetAddTargetData) -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_TARGET_STATS_OBJECT: usize = 0x02;
        pub fn get_target_stats_object() -> *mut TESObjectREFR
    }

    virtual_method! {
        pub const VFUNC_MAGIC_TARGET_IS_ACTOR: usize = 0x03;
        pub fn magic_target_is_actor() -> bool
    }

    virtual_method! {
        pub const VFUNC_IS_INVULNERABLE: usize = 0x04;
        pub fn is_invulnerable() -> bool
    }

    virtual_method! {
        pub const VFUNC_INVALIDATE_COMMANDED_ACTOR_EFFECT: usize = 0x05;
        pub fn invalidate_commanded_actor_effect(effect: *mut ActiveEffect)
    }

    virtual_method! {
        pub const VFUNC_CAN_ADD_ACTIVE_EFFECT: usize = 0x06;
        pub fn can_add_active_effect() -> bool
    }

    virtual_method! {
        pub const VFUNC_GET_ACTIVE_EFFECT_LIST: usize = 0x07;
        pub fn get_active_effect_list_impl() -> *mut BSSimpleList<*mut ActiveEffect>
    }

    virtual_method! {
        pub const VFUNC_EFFECT_ADDED: usize = 0x08;
        pub fn effect_added(effect: *mut ActiveEffect)
    }

    virtual_method! {
        pub const VFUNC_EFFECT_REMOVED: usize = 0x09;
        pub fn effect_removed(effect: *mut ActiveEffect)
    }

    virtual_method! {
        pub const VFUNC_CHECK_RESISTANCE: usize = 0x0A;
        pub fn check_resistance(magic_item: *mut MagicItem, effect: *mut Effect, object: *mut TESBoundObject) -> f32
    }

    virtual_method! {
        pub const VFUNC_CHECK_ABSORB: usize = 0x0B;
        pub fn check_absorb(actor: *mut Actor, magic_item: *mut MagicItem, effect: *const Effect) -> bool
    }

    crate::relocation_func! {
        pub fn dispel_effect(&mut self, spell: *mut MagicItem, caster: &mut ActorHandle, effect: *mut ActiveEffect) -> bool => RelocationID::new(33721, 34505)
    }

    #[inline]
    pub fn get_active_effect_list(&self) -> *mut BSSimpleList<*mut ActiveEffect> {
        if !runtime::is_vr() {
            return unsafe { (*(self as *const _ as *mut Self)).get_active_effect_list_impl() };
        }

        let snapshot = unsafe { &mut *Self::active_effect_snapshot_fls() };

        snapshot.clear();

        let mut effects = Vec::new();
        self.visit_active_effects(|effect| {
            if !effect.is_null() {
                effects.push(effect);
            }
            BSContainerForEachResult::Continue
        });

        for effect in effects.into_iter().rev() {
            snapshot.push_front(effect);
        }

        snapshot as *mut _
    }

    #[inline(always)]
    pub fn get_target_as_actor(&self) -> *mut Actor {
        if self.magic_target_is_actor() {
            self as *const Self as *mut Self as *mut Actor
        } else {
            core::ptr::null_mut()
        }
    }

    pub fn has_effect_with_archetype(&self, archetype: EffectArchetypeId) -> bool {
        let mut found = false;
        self.visit_active_effects(|effect| {
            let setting = unsafe { effect.as_ref() }
                .map(|effect| effect.get_base_object())
                .unwrap_or(core::ptr::null_mut());
            if !setting.is_null() && unsafe { (*setting).has_archetype(archetype) } {
                found = true;
                BSContainerForEachResult::Stop
            } else {
                BSContainerForEachResult::Continue
            }
        });
        found
    }

    crate::relocation_func! {
        pub fn has_magic_effect(&self, effect: *mut EffectSetting) -> bool => RelocationID::new(33733, 34517)
    }

    crate::relocation_func! {
        pub fn has_magic_effect_with_keyword(&self, keyword: *mut BGSKeyword, spell_out: *mut *mut MagicItem) -> bool => RelocationID::new(33734, 34518)
    }

    crate::relocation_func! {
        pub fn visit_effects(&self, visitor: &mut MagicTargetForEachActiveEffectVisitor) => RelocationID::new(33756, 34540)
    }

    pub fn visit_active_effects<F>(&self, mut callback: F)
    where
        F: FnMut(*mut ActiveEffect) -> BSContainerForEachResult,
    {
        let mut visitor = MagicTargetVisitEffectsVisitor {
            base: MagicTargetForEachActiveEffectVisitor {
                vtable: MAGIC_TARGET_VISIT_EFFECTS_VISITOR_VTABLE.0.as_ptr().cast(),
            },
            ctx: (&mut callback as *mut F).cast(),
            callback: visit_effects_closure::<F>,
        };

        self.visit_effects(&mut visitor.base);
    }

    #[inline]
    pub fn dispel_effects_with_archetype(&mut self, archetype: EffectArchetypeId, force: bool) {
        let mut queued = Vec::new();

        self.visit_active_effects(|effect| {
            let setting = unsafe { effect.as_ref() }
                .map(|effect| effect.get_base_object())
                .unwrap_or(core::ptr::null_mut());
            if !setting.is_null() && unsafe { (*setting).has_archetype(archetype) } {
                queued.push(effect);
            }
            BSContainerForEachResult::Continue
        });

        for effect in queued {
            if let Some(effect) = unsafe { effect.as_mut() } {
                effect.dispel(force);
            }
        }
    }

    #[inline]
    unsafe fn active_effect_snapshot_fls() -> *mut BSSimpleList<*mut ActiveEffect> {
        let fls_index = Self::active_effect_snapshot_fls_index();

        let snapshot = unsafe { FlsGetValue(fls_index) as *mut BSSimpleList<*mut ActiveEffect> };
        if !snapshot.is_null() {
            return snapshot;
        }

        let snapshot = Box::into_raw(Box::new(BSSimpleList::<*mut ActiveEffect>::default()));
        let ok = unsafe { FlsSetValue(fls_index, snapshot.cast()) };
        if ok == 0 {
            unsafe {
                drop(Box::from_raw(snapshot));
            }
            assert!(
                ok != 0,
                "MagicTarget::GetActiveEffectList FLS initialization failed"
            );
        }
        snapshot
    }

    #[inline]
    fn active_effect_snapshot_fls_index() -> u32 {
        loop {
            let fls_index = MAGIC_TARGET_ACTIVE_EFFECT_FLS_INDEX.load(Ordering::Acquire);
            if fls_index == MAGIC_TARGET_ACTIVE_EFFECT_FLS_INITIALIZING {
                core::hint::spin_loop();
                continue;
            }
            if fls_index != FLS_OUT_OF_INDEXES {
                return fls_index;
            }

            if MAGIC_TARGET_ACTIVE_EFFECT_FLS_INDEX
                .compare_exchange(
                    FLS_OUT_OF_INDEXES,
                    MAGIC_TARGET_ACTIVE_EFFECT_FLS_INITIALIZING,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
                .is_ok()
            {
                let new_index = unsafe { FlsAlloc(Some(active_effect_snapshot_fls_dtor)) };
                if new_index == FLS_OUT_OF_INDEXES {
                    MAGIC_TARGET_ACTIVE_EFFECT_FLS_INDEX
                        .store(FLS_OUT_OF_INDEXES, Ordering::Release);
                    assert!(
                        new_index != FLS_OUT_OF_INDEXES,
                        "MagicTarget::GetActiveEffectList FLS allocation failed"
                    );
                }

                MAGIC_TARGET_ACTIVE_EFFECT_FLS_INDEX.store(new_index, Ordering::Release);
                return new_index;
            }
        }
    }
}

pub trait MagicTargetExt {
    fn dtor(&mut self);
    fn add_target(&mut self, target_data: *mut MagicTargetAddTargetData) -> bool;
    fn get_target_stats_object(&self) -> *mut TESObjectREFR;
    fn magic_target_is_actor(&self) -> bool;
    fn is_invulnerable(&mut self) -> bool;
    fn invalidate_commanded_actor_effect(&mut self, effect: *mut ActiveEffect);
    fn can_add_active_effect(&mut self) -> bool;
    fn get_active_effect_list_impl(&mut self) -> *mut BSSimpleList<*mut ActiveEffect>;
    fn effect_added(&mut self, effect: *mut ActiveEffect);
    fn effect_removed(&mut self, effect: *mut ActiveEffect);
    fn check_resistance(
        &mut self,
        magic_item: *mut MagicItem,
        effect: *mut Effect,
        object: *mut TESBoundObject,
    ) -> f32;
    fn check_absorb(
        &mut self,
        actor: *mut Actor,
        magic_item: *mut MagicItem,
        effect: *const Effect,
    ) -> bool;
    fn dispel_effect(
        &mut self,
        spell: *mut MagicItem,
        caster: &mut ActorHandle,
        effect: *mut ActiveEffect,
    ) -> bool;
    fn get_active_effect_list(&self) -> *mut BSSimpleList<*mut ActiveEffect>;
    fn get_target_as_actor(&self) -> *mut Actor;
    fn has_effect_with_archetype(&self, archetype: EffectArchetypeId) -> bool;
    fn has_magic_effect(&self, effect: *mut EffectSetting) -> bool;
    fn has_magic_effect_with_keyword(
        &self,
        keyword: *mut BGSKeyword,
        spell_out: *mut *mut MagicItem,
    ) -> bool;
    fn visit_effects(&self, visitor: &mut MagicTargetForEachActiveEffectVisitor);
    fn visit_active_effects<F>(&self, callback: F)
    where
        F: FnMut(*mut ActiveEffect) -> BSContainerForEachResult;
    fn dispel_effects_with_archetype(&mut self, archetype: EffectArchetypeId, force: bool);
}

impl<T: AsRef<MagicTarget> + AsMut<MagicTarget>> MagicTargetExt for T {
    fn dtor(&mut self) {
        MagicTarget::dtor(self.as_mut())
    }

    fn add_target(&mut self, target_data: *mut MagicTargetAddTargetData) -> bool {
        MagicTarget::add_target(self.as_mut(), target_data)
    }

    fn get_target_stats_object(&self) -> *mut TESObjectREFR {
        self.as_ref().get_target_stats_object()
    }

    fn magic_target_is_actor(&self) -> bool {
        self.as_ref().magic_target_is_actor()
    }

    fn is_invulnerable(&mut self) -> bool {
        MagicTarget::is_invulnerable(self.as_mut())
    }

    fn invalidate_commanded_actor_effect(&mut self, effect: *mut ActiveEffect) {
        MagicTarget::invalidate_commanded_actor_effect(self.as_mut(), effect)
    }

    fn can_add_active_effect(&mut self) -> bool {
        MagicTarget::can_add_active_effect(self.as_mut())
    }

    fn get_active_effect_list_impl(&mut self) -> *mut BSSimpleList<*mut ActiveEffect> {
        MagicTarget::get_active_effect_list_impl(self.as_mut())
    }

    fn effect_added(&mut self, effect: *mut ActiveEffect) {
        MagicTarget::effect_added(self.as_mut(), effect)
    }

    fn effect_removed(&mut self, effect: *mut ActiveEffect) {
        MagicTarget::effect_removed(self.as_mut(), effect)
    }

    fn check_resistance(
        &mut self,
        magic_item: *mut MagicItem,
        effect: *mut Effect,
        object: *mut TESBoundObject,
    ) -> f32 {
        MagicTarget::check_resistance(self.as_mut(), magic_item, effect, object)
    }

    fn check_absorb(
        &mut self,
        actor: *mut Actor,
        magic_item: *mut MagicItem,
        effect: *const Effect,
    ) -> bool {
        MagicTarget::check_absorb(self.as_mut(), actor, magic_item, effect)
    }

    fn dispel_effect(
        &mut self,
        spell: *mut MagicItem,
        caster: &mut ActorHandle,
        effect: *mut ActiveEffect,
    ) -> bool {
        MagicTarget::dispel_effect(self.as_mut(), spell, caster, effect)
    }

    fn get_active_effect_list(&self) -> *mut BSSimpleList<*mut ActiveEffect> {
        self.as_ref().get_active_effect_list()
    }

    fn get_target_as_actor(&self) -> *mut Actor {
        self.as_ref().get_target_as_actor()
    }

    fn has_effect_with_archetype(&self, archetype: EffectArchetypeId) -> bool {
        self.as_ref().has_effect_with_archetype(archetype)
    }

    fn has_magic_effect(&self, effect: *mut EffectSetting) -> bool {
        self.as_ref().has_magic_effect(effect)
    }

    fn has_magic_effect_with_keyword(
        &self,
        keyword: *mut BGSKeyword,
        spell_out: *mut *mut MagicItem,
    ) -> bool {
        self.as_ref()
            .has_magic_effect_with_keyword(keyword, spell_out)
    }

    fn visit_effects(&self, visitor: &mut MagicTargetForEachActiveEffectVisitor) {
        self.as_ref().visit_effects(visitor)
    }

    fn visit_active_effects<F>(&self, callback: F)
    where
        F: FnMut(*mut ActiveEffect) -> BSContainerForEachResult,
    {
        self.as_ref().visit_active_effects(callback)
    }

    fn dispel_effects_with_archetype(&mut self, archetype: EffectArchetypeId, force: bool) {
        self.as_mut()
            .dispel_effects_with_archetype(archetype, force)
    }
}
