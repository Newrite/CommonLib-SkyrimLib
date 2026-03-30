use crate::re::collision_layers::ColLayer;

/// C++ `RE::BIPED_PART`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BipedPart {
    Other = 0,
    Head = 1,
    Body = 2,
    Spine1 = 3,
    Spine2 = 4,
    LeftUpperArm = 5,
    LeftForearm = 6,
    LeftHand = 7,
    LeftThigh = 8,
    LeftCalf = 9,
    LeftFoot = 10,
    RightUpperArm = 11,
    RightForearm = 12,
    RightHand = 13,
    RightThigh = 14,
    RightCalf = 15,
    RightFoot = 16,
    Tail = 17,
    Shield = 18,
    Quiver = 19,
    Weapon = 20,
    PonyTail = 21,
    Wing = 22,
    Pack = 23,
    Chain = 24,
    AddonHead = 25,
    AddonChest = 26,
    AddonLeg = 27,
    AddonArm = 28,
}

core_util::impl_enumset_type!(BipedPart => u32);

/// C++ `RE::CFilter::Flags`
pub struct CFilterFlags;

impl CFilterFlags {
    pub const NONE: u32 = 0;
    pub const NO_COLLISION: u32 = 1 << 14;
    pub const LINKED_GROUP: u32 = 1 << 15;
    pub const PART_MASK: u32 = 0x1F;
    pub const LAYER_MASK: u32 = 0x7F;
}

/// C++ `RE::CFilter`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CFilter {
    pub filter: u32, // 00
}

const _: () = assert!(core::mem::size_of::<CFilter>() == 0x4);
const _: () = assert!(core::mem::offset_of!(CFilter, filter) == 0x0);

impl CFilter {
    #[inline(always)]
    pub fn get_collision_layer(&self) -> ColLayer {
        unsafe { core::mem::transmute((self.filter & CFilterFlags::LAYER_MASK) as i32) }
    }

    #[inline(always)]
    pub fn get_biped_part(&self) -> BipedPart {
        unsafe { core::mem::transmute((self.filter >> 8) & CFilterFlags::PART_MASK) }
    }

    #[inline(always)]
    pub const fn q_no_collision(&self) -> bool {
        ((self.filter >> 14) & 1) != 0
    }

    #[inline(always)]
    pub const fn q_linked_group(&self) -> bool {
        ((self.filter >> 15) & 1) != 0
    }

    #[inline(always)]
    pub const fn get_system_group(&self) -> u32 {
        self.filter >> 16
    }

    #[inline(always)]
    pub fn set_collision_layer(&mut self, layer: ColLayer) {
        self.filter &= !CFilterFlags::LAYER_MASK;
        self.filter |= (layer as u32) & CFilterFlags::LAYER_MASK;
    }

    #[inline(always)]
    pub fn set_biped_part(&mut self, part: BipedPart) {
        self.filter &= !(CFilterFlags::PART_MASK << 8);
        self.filter |= ((part as u32) & CFilterFlags::PART_MASK) << 8;
    }

    #[inline(always)]
    pub fn set_no_collision(&mut self, set: bool) {
        if set {
            self.filter |= CFilterFlags::NO_COLLISION;
        } else {
            self.filter &= !CFilterFlags::NO_COLLISION;
        }
    }

    #[inline(always)]
    pub fn set_linked_group(&mut self, set: bool) {
        if set {
            self.filter |= CFilterFlags::LINKED_GROUP;
        } else {
            self.filter &= !CFilterFlags::LINKED_GROUP;
        }
    }

    #[inline(always)]
    pub fn set_system_group(&mut self, group: u32) {
        self.filter &= 0x0000_FFFF;
        self.filter |= group << 16;
    }
}
