#![allow(non_camel_case_types)]

/// C++ `RE::hkpShapeType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpShapeType {
    Invalid = 0,
    Sphere = 1,
    Cylinder = 2,
    Triangle = 3,
    Box = 4,
    Capsule = 5,
    ConvexVertices = 6,
    Collection = 7,
    BVTree = 8,
    List = 9,
    MOPP = 10,
    ConvexTranslate = 11,
    ConvexTransform = 12,
    SampledHeightField = 13,
    ExtendedMesh = 14,
    Transform = 15,
    CompressedMesh = 16,
    Compound = 17,
    TotalSPU = 18,
    Convex = 19,
    MOPPEmbedded = 20,
    ConvexPiece = 21,
    MultiSphere = 22,
    ConvexList = 23,
    TriangleCollection = 24,
    MultiRay = 25,
    HeightField = 26,
    SphereRep = 27,
    BV = 28,
    Plane = 29,
    PhantomCallback = 30,
    User0 = 31,
    User1 = 32,
    User2 = 33,
    Total = 34,
    All = -1,
}

impl Default for hkpShapeType {
    #[inline(always)]
    fn default() -> Self {
        Self::Invalid
    }
}

impl hkpShapeType {
    pub const FIRST_TYPE: Self = Self::Sphere;
}
