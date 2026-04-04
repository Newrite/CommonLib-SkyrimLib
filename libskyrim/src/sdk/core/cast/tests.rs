use super::{ConstRttiCastSource, DynamicCastExt, DynamicCastMutExt, MutRttiCastSource};
use crate::re::{Actor, TESObjectREFR};
use crate::sdk::core::{GamePtr, GameRef, Resolved};

#[test]
fn cast_traits_expose_expected_signatures_for_core_wrappers() {
    let _ = <Actor as ConstRttiCastSource>::raw_const_source_ptr as fn(&Actor) -> *const Actor;
    let _ = <Actor as MutRttiCastSource>::raw_mut_source_ptr as fn(&mut Actor) -> *mut Actor;
    let _ = <Actor as DynamicCastExt>::cast_const::<TESObjectREFR>
        as fn(&Actor) -> *const TESObjectREFR;
    let _ = <Actor as DynamicCastExt>::can_cast::<TESObjectREFR> as fn(&Actor) -> bool;
    let _ = <Actor as DynamicCastExt>::try_cast_ref::<TESObjectREFR>
        as fn(&Actor) -> Option<&TESObjectREFR>;
    let _ = <Actor as DynamicCastMutExt>::cast_raw::<TESObjectREFR>
        as fn(&mut Actor) -> *mut TESObjectREFR;
    let _ = <Actor as DynamicCastMutExt>::try_cast_mut::<TESObjectREFR>
        as fn(&mut Actor) -> Option<&mut TESObjectREFR>;

    let _ = <GameRef<Actor> as ConstRttiCastSource>::raw_const_source_ptr
        as fn(&GameRef<Actor>) -> *const Actor;
    let _ = <GamePtr<Actor> as ConstRttiCastSource>::raw_const_source_ptr
        as fn(&GamePtr<Actor>) -> *const Actor;
    let _ = <Resolved<Actor> as ConstRttiCastSource>::raw_const_source_ptr
        as fn(&Resolved<Actor>) -> *const Actor;
    let _ = <Resolved<Actor> as MutRttiCastSource>::raw_mut_source_ptr
        as fn(&mut Resolved<Actor>) -> *mut Actor;
}
