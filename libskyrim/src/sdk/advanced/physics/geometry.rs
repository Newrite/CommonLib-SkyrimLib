use crate::re::NiPoint3;

#[inline(always)]
pub fn point_on_segment(from: NiPoint3, to: NiPoint3, fraction: f32) -> NiPoint3 {
    from + (to - from) * fraction
}

#[inline(always)]
pub fn backoff_point(from: NiPoint3, to: NiPoint3, fraction: f32, distance: f32) -> NiPoint3 {
    let delta = to - from;
    let length = delta.length();
    if length <= f32::EPSILON {
        return point_on_segment(from, to, fraction);
    }

    let backed_off_fraction = (fraction - (distance / length)).clamp(0.0, 1.0);
    point_on_segment(from, to, backed_off_fraction)
}

#[inline(always)]
pub fn offset_point_along_normal(point: NiPoint3, normal: NiPoint3, distance: f32) -> NiPoint3 {
    let mut direction = normal;
    direction.unitize();
    point + direction * distance
}
