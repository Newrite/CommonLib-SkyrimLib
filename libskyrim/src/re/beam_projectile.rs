#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_BeamProjectile;
use crate::offsets::offsets_vtable::VTABLE_BeamProjectile;
use crate::re::{
    BSEventNotifyControl, BSProceduralGeomEvent, BSTEventSink, BSTEventSource,
    BeamProjectileImpactEvent, FormCastable, FormType, Projectile,
};
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::BeamProjectile::RecordFlags`
pub struct BeamProjectileRecordFlags;

/// C++ `RE::BeamProjectile::BEAM_RUNTIME_DATA`
#[repr(C)]
pub struct BeamProjectileRuntimeData {
    pub unk238: u64,
}

const _: () = assert!(core::mem::size_of::<BeamProjectileRuntimeData>() == 0x08);
const _: () = assert!(core::mem::offset_of!(BeamProjectileRuntimeData, unk238) == 0x00);

/// C++ `RE::BeamProjectile`
#[repr(C)]
pub struct BeamProjectile {
    pub base: Projectile, // 00
}

const _: () = assert!(core::mem::size_of::<BeamProjectile>() == 0x80);
const _: () = assert!(core::mem::offset_of!(BeamProjectile, base) == 0x00);

core_util::inherit!(BeamProjectile : Projectile, base);

impl RttiType for BeamProjectile {
    const RTTI: VariantID = RTTI_BeamProjectile;
}

impl FormCastable for BeamProjectile {
    const TARGET_FORM_TYPE: FormType = FormType::ProjectileBeam;
}

impl BeamProjectile {
    pub const RTTI: VariantID = RTTI_BeamProjectile;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BeamProjectile;
    pub const FORMTYPE: FormType = FormType::ProjectileBeam;
    pub const BS_PROCEDURAL_GEOM_EVENT_SINK_OFFSET: VariantOffset =
        VariantOffset::new(0x1D8, 0x1E0, 0x1D8);
    pub const BEAM_PROJECTILE_IMPACT_EVENT_SOURCE_OFFSET: VariantOffset =
        VariantOffset::new(0x1E0, 0x1E8, 0x1E0);
    // TODO: `BeamProjectile.h` is internally inconsistent here: the vendored header keeps
    // `RUNTIME_DATA_ACCESSOR_VERSIONED_EX(..., 0x98, 0xA0)` but also comments the named member as
    // `unk238 // 238, 240`, and the moved `BSTEventSink`/`BSTEventSource` bases also force the
    // tail to start after `0x238/0x240`. Keep the concrete layout-comment/base-arithmetic offset
    // until a source-backed verify pass confirms whether the accessor macro or the field comments
    // are wrong upstream.
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x238, 0x240, 0x238);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x240, 0x248, 0x240);

    crate::runtime_cast_accessor! {
        pub fn as_bs_procedural_geom_event_sink() -> BSTEventSink<BSProceduralGeomEvent> {
            offset: Self::BS_PROCEDURAL_GEOM_EVENT_SINK_OFFSET
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_bs_procedural_geom_event_sink_mut() -> BSTEventSink<BSProceduralGeomEvent> {
            offset: Self::BS_PROCEDURAL_GEOM_EVENT_SINK_OFFSET
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_beam_projectile_impact_event_source() -> BSTEventSource<BeamProjectileImpactEvent> {
            offset: Self::BEAM_PROJECTILE_IMPACT_EVENT_SOURCE_OFFSET
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_beam_projectile_impact_event_source_mut() -> BSTEventSource<BeamProjectileImpactEvent> {
            offset: Self::BEAM_PROJECTILE_IMPACT_EVENT_SOURCE_OFFSET
        }
    }

    crate::runtime_data_accessor! {
        pub fn get_beam_runtime_data() -> BeamProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_beam_runtime_data_mut() -> BeamProjectileRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    #[inline(always)]
    pub fn process_procedural_geom_event(
        &mut self,
        event: *const BSProceduralGeomEvent,
        event_source: *mut BSTEventSource<BSProceduralGeomEvent>,
    ) -> BSEventNotifyControl {
        unsafe {
            self.as_bs_procedural_geom_event_sink_mut()
                .process_event(event, event_source)
        }
    }
}
