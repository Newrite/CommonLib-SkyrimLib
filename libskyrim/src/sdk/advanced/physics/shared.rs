use core::cmp::Ordering;
use core::ptr;

use crate::re::{
    BSReadLockGuard, CFilter, NiAVObject, NiPoint3, TESHavokUtilities, TESObjectCELL,
    TESObjectREFR, bhkPickData, bhkWorld, hkVector4, hkpWorldRayCastOutput,
};

use super::geometry::point_on_segment;
use super::types::RaycastHit;

/// Runs one closure while holding the cell Havok world read lock.
///
/// Returns [`None`] when the cell has no physics world.
pub(super) fn with_cell_world_read<R>(
    cell: &TESObjectCELL,
    f: impl FnOnce(*mut bhkWorld) -> R,
) -> Option<R> {
    let world = cell.get_bhk_world();
    if world.is_null() {
        return None;
    }

    let _lock = unsafe { BSReadLockGuard::from_ptr(ptr::addr_of_mut!((*world).world_lock)) };
    Some(f(world))
}

/// Builds the low-level `bhkPickData` packet used by raycast helpers.
#[inline(always)]
pub(super) fn build_pick_data(from: NiPoint3, to: NiPoint3, filter: CFilter) -> bhkPickData {
    let world_scale = bhkWorld::get_world_scale();
    let mut pick_data = bhkPickData::default();
    pick_data.ray_input.from = hkVector4::from(from * world_scale);
    pick_data.ray_input.to = hkVector4::from(to * world_scale);
    pick_data.ray_input.filter_info = filter;
    pick_data.ray = hkVector4::from((to - from) * world_scale);
    pick_data
}

/// Converts one Havok raycast output into the SDK-facing [`RaycastHit`].
pub(super) fn raycast_hit_from_output(
    from: NiPoint3,
    to: NiPoint3,
    output: &hkpWorldRayCastOutput,
) -> Option<RaycastHit> {
    if !output.has_hit() {
        return None;
    }

    let root_collidable = output.root_collidable;
    let collision_layer = if root_collidable.is_null() {
        None
    } else {
        unsafe { (*root_collidable).try_get_collision_layer() }
    };

    let (reference, object): (*mut TESObjectREFR, *mut NiAVObject) = if root_collidable.is_null() {
        (ptr::null_mut(), ptr::null_mut())
    } else {
        let collidable = unsafe { &*root_collidable };
        (
            TESHavokUtilities::find_collidable_ref(collidable),
            TESHavokUtilities::find_collidable_object(collidable),
        )
    };

    let hit_fraction = output.base.base.hit_fraction;
    Some(RaycastHit {
        hit_fraction,
        hit_point: point_on_segment(from, to, hit_fraction),
        normal: ni_point3_from_hk(&output.base.base.normal),
        root_collidable,
        collision_layer,
        reference,
        object,
    })
}

/// Comparison helper that sorts hits by increasing hit fraction.
#[inline(always)]
pub(super) fn compare_hit_fraction(a: &RaycastHit, b: &RaycastHit) -> Ordering {
    a.hit_fraction
        .partial_cmp(&b.hit_fraction)
        .unwrap_or(Ordering::Equal)
}

/// Converts one Havok vector into a gameplay-space [`NiPoint3`].
#[inline(always)]
fn ni_point3_from_hk(vector: &hkVector4) -> NiPoint3 {
    NiPoint3::new(vector.quad[0], vector.quad[1], vector.quad[2])
}
