#![allow(non_camel_case_types)]

use crate::re::collision_layers::ColLayer;

/// C++ `RE::BIPED_PART`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BIPED_PART {
    kOther = 0,
    kHead = 1,
    kBody = 2,
    kSpine1 = 3,
    kSpine2 = 4,
    kLUpperArm = 5,
    kLForearm = 6,
    kLHand = 7,
    kLThigh = 8,
    kLCalf = 9,
    kLFoot = 10,
    kRUpperArm = 11,
    kRForearm = 12,
    kRHand = 13,
    kRThigh = 14,
    kRCalf = 15,
    kRFoot = 16,
    kTail = 17,
    kShield = 18,
    kQuiver = 19,
    kWeapon = 20,
    kPonyTail = 21,
    kWing = 22,
    kPack = 23,
    kChain = 24,
    kAddonHead = 25,
    kAddonChest = 26,
    kAddonLeg = 27,
    kAddonArm = 28,
}

core_util::impl_enumset_type!(BIPED_PART => u32);

/// C++ `RE::CFilter::Flags`
pub struct CFilterFlags;

impl CFilterFlags {
    pub const kNone: u32 = 0;
    pub const kNoCollision: u32 = 1 << 14;
    pub const kLinkedGroup: u32 = 1 << 15;
    pub const kPartMask: u32 = 0x1F;
    pub const kLayerMask: u32 = 0x7F;
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
        unsafe { core::mem::transmute((self.filter & CFilterFlags::kLayerMask) as i32) }
    }

    #[inline(always)]
    pub fn get_biped_part(&self) -> BIPED_PART {
        unsafe { core::mem::transmute((self.filter >> 8) & CFilterFlags::kPartMask) }
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
        self.filter &= !CFilterFlags::kLayerMask;
        self.filter |= (layer as u32) & CFilterFlags::kLayerMask;
    }

    #[inline(always)]
    pub fn set_biped_part(&mut self, part: BIPED_PART) {
        self.filter &= !(CFilterFlags::kPartMask << 8);
        self.filter |= ((part as u32) & CFilterFlags::kPartMask) << 8;
    }

    #[inline(always)]
    pub fn set_no_collision(&mut self, set: bool) {
        if set {
            self.filter |= CFilterFlags::kNoCollision;
        } else {
            self.filter &= !CFilterFlags::kNoCollision;
        }
    }

    #[inline(always)]
    pub fn set_linked_group(&mut self, set: bool) {
        if set {
            self.filter |= CFilterFlags::kLinkedGroup;
        } else {
            self.filter &= !CFilterFlags::kLinkedGroup;
        }
    }

    #[inline(always)]
    pub fn set_system_group(&mut self, group: u32) {
        self.filter &= 0x0000_FFFF;
        self.filter |= group << 16;
    }
}
