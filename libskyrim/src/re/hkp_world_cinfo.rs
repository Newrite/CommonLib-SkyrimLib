#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_hkpWorldCinfo;
use crate::offsets::offsets_vtable::VTABLE_hkpWorldCinfo;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::hkpWorldCinfo::SolverType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpWorldCinfoSolverType {
    Invalid = 0,
    Iters2Soft = 1,
    Iters2Medium = 2,
    Iters2Hard = 3,
    Iters4Soft = 4,
    Iters4Medium = 5,
    Iters4Hard = 6,
    Iters8Soft = 7,
    Iters8Medium = 8,
    Iters8Hard = 9,
    Total = 10,
}

/// C++ `RE::hkpWorldCinfo::SimulationType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpWorldCinfoSimulationType {
    Invalid = 0,
    Discrete = 1,
    Continuous = 2,
    Multithreaded = 3,
}

/// C++ `RE::hkpWorldCinfo::ContactPointGeneration`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpWorldCinfoContactPointGeneration {
    AcceptAlways = 0,
    RejectDubious = 1,
    RejectMany = 2,
}

/// C++ `RE::hkpWorldCinfo::BroadPhaseBorderBehaviour`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpWorldCinfoBroadPhaseBorderBehaviour {
    Assert = 0,
    FixEntity = 1,
    RemoveEntity = 2,
    DoNothing = 3,
}

/// C++ `RE::hkpWorldCinfo::TreeUpdateType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpWorldCinfoTreeUpdateType {
    Active = 0,
    All = 1,
}

crate::core_util::abstract_type! {
    pub type hkpWorldCinfo;
}

impl RttiType for hkpWorldCinfo {
    const RTTI: VariantID = RTTI_hkpWorldCinfo;
}

impl hkpWorldCinfo {
    pub const RTTI: VariantID = RTTI_hkpWorldCinfo;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpWorldCinfo;
}

// TODO: SOURCE - keep `hkpWorldCinfo` opaque for now. `hkpWorld` only needs
// the nested enum layers from `hkpWorldCinfo.h`, and the concrete field layout
// around `hkRefPtr` members is not yet expressed honestly enough in Rust.
