//! Translation of `RE::BGSBodyPartDefs.h`.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum LimbEnum {
    None = -1,
    Torso = 0,
    Head = 1,
    Eye = 2,
    LookAt = 3,
    FlyGrab = 4,
    LeftArm = 5,
    LeftHand = 6,
    RightArm = 7,
    RightHand = 8,
    LeftLeg = 9,
    LeftFoot = 10,
    RightLeg = 11,
    RightFoot = 12,
    Tail = 13,
}
