use crate::re::{BSGeometry, NiAVObject, NiNode, NiObject, NiPointLight, bhkNiCollisionObject};

/// C++ `RE::BSVisit::BSVisitControl`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSVisitControl {
    Continue = 0,
    Stop = 1,
}

#[inline]
fn visit_children(
    node: *mut NiNode,
    mut visit: impl FnMut(*mut NiAVObject) -> BSVisitControl,
) -> BSVisitControl {
    if node.is_null() {
        return BSVisitControl::Continue;
    }

    let children = unsafe { (*node).get_children() };
    if children.data.is_null() || children.len() == 0 {
        return BSVisitControl::Continue;
    }

    let slice = unsafe { core::slice::from_raw_parts(children.data, children.len() as usize) };
    for child in slice {
        let result = visit(child.get());
        if matches!(result, BSVisitControl::Stop) {
            return result;
        }
    }

    BSVisitControl::Continue
}

pub fn traverse_scenegraph_collision(
    object: *mut NiAVObject,
    func: &mut impl FnMut(*mut bhkNiCollisionObject) -> BSVisitControl,
) -> BSVisitControl {
    if object.is_null() {
        return BSVisitControl::Continue;
    }

    let collision = unsafe {
        (*object)
            .collision_object
            .get()
            .cast::<bhkNiCollisionObject>()
    };
    if !collision.is_null() {
        let result = func(collision);
        if matches!(result, BSVisitControl::Stop) {
            return result;
        }
    }

    let node = unsafe { (*object.cast::<NiObject>()).as_node() };
    visit_children(node, |child| traverse_scenegraph_collision(child, func))
}

pub fn traverse_scenegraph_geometries(
    object: *mut NiAVObject,
    func: &mut impl FnMut(*mut BSGeometry) -> BSVisitControl,
) -> BSVisitControl {
    if object.is_null() {
        return BSVisitControl::Continue;
    }

    let geom = unsafe { (*object.cast::<NiObject>()).as_geometry() };
    if !geom.is_null() {
        return func(geom);
    }

    let node = unsafe { (*object.cast::<NiObject>()).as_node() };
    visit_children(node, |child| traverse_scenegraph_geometries(child, func))
}

pub fn traverse_scenegraph_lights(
    object: *mut NiAVObject,
    func: &mut impl FnMut(*mut NiPointLight) -> BSVisitControl,
) -> BSVisitControl {
    if object.is_null() {
        return BSVisitControl::Continue;
    }

    let point_light = unsafe {
        crate::re::ni_rtti::netimmerse_cast::<NiPointLight>(
            object.cast::<NiObject>(),
            NiPointLight::NI_RTTI.address() as *const crate::re::NiRTTI,
        )
    };
    if !point_light.is_null() {
        return func(point_light);
    }

    let node = unsafe { (*object.cast::<NiObject>()).as_node() };
    visit_children(node, |child| traverse_scenegraph_lights(child, func))
}

pub fn traverse_scenegraph_objects(
    object: *mut NiAVObject,
    func: &mut impl FnMut(*mut NiAVObject) -> BSVisitControl,
) -> BSVisitControl {
    if object.is_null() {
        return BSVisitControl::Continue;
    }

    let result = func(object);
    if matches!(result, BSVisitControl::Stop) {
        return result;
    }

    let node = unsafe { (*object.cast::<NiObject>()).as_node() };
    visit_children(node, |child| traverse_scenegraph_objects(child, func))
}
