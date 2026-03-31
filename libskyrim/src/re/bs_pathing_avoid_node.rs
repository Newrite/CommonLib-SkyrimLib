use crate::core_util::EnumSet;
use crate::re::NiPoint3;

/// C++ `RE::BSPathingAvoidNode::AvoidNodeType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSPathingAvoidNodeType {
    Sphere = 0,
    Cylinder = 1,
    SphereActor = 2,
    SphereTarget = 3,
    SphereThreat = 4,
    SphereObstacle = 5,
}

core_util::impl_enumset_type!(BSPathingAvoidNodeType => u32);

/// C++ `RE::BSPathingAvoidNode`
#[repr(C)]
pub struct BSPathingAvoidNode {
    pub point1: NiPoint3,                                      // 00
    pub point2: NiPoint3,                                      // 0C
    pub radius: f32,                                           // 18
    pub cost: f32,                                             // 1C
    pub avoid_node_type: EnumSet<BSPathingAvoidNodeType, u32>, // 20
}

const _: () = assert!(core::mem::size_of::<BSPathingAvoidNode>() == 0x24);
const _: () = assert!(core::mem::offset_of!(BSPathingAvoidNode, point1) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSPathingAvoidNode, point2) == 0x0C);
const _: () = assert!(core::mem::offset_of!(BSPathingAvoidNode, radius) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSPathingAvoidNode, cost) == 0x1C);
const _: () = assert!(core::mem::offset_of!(BSPathingAvoidNode, avoid_node_type) == 0x20);
