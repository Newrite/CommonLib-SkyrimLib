use alloc::string::String;
use alloc::vec::Vec;

use core_util::EnumSet;

use crate::re::{
    ACTOR_LOS_LOCATION, Actor, ActorHandle, BGSExplosion, BGSMaterialType, BGSProjectile,
    BGSProjectileFlags, BGSProjectileType, CastingSource, ColLayer, EffectSetting, ImpactResult,
    MagicItem, NiPoint3, ObjectRefHandle, Projectile, ProjectileHandle, TESAmmo, TESObjectREFR,
    TESObjectWEAP,
};
use crate::sdk::core::{GamePtr, Resolved};

use super::shared::{
    anticipated_position_from_velocity, handle_to_ptr, push_unique_handle, resolve_managed_handle,
};
use super::targeting::view_cone_threshold_dot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectileKind {
    Arrow,
    Barrier,
    Beam,
    Cone,
    Flame,
    Grenade,
    Missile,
}

impl ProjectileKind {
    #[inline(always)]
    pub fn from_types(types: EnumSet<BGSProjectileType, u16>) -> Option<Self> {
        if types.all(BGSProjectileType::Arrow) {
            Some(Self::Arrow)
        } else if types.all(BGSProjectileType::Barrier) {
            Some(Self::Barrier)
        } else if types.all(BGSProjectileType::Beam) {
            Some(Self::Beam)
        } else if types.all(BGSProjectileType::Cone) {
            Some(Self::Cone)
        } else if types.all(BGSProjectileType::Flamethrower) {
            Some(Self::Flame)
        } else if types.all(BGSProjectileType::Grenade) {
            Some(Self::Grenade)
        } else if types.all(BGSProjectileType::Missile) {
            Some(Self::Missile)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn from_base(projectile: &BGSProjectile) -> Option<Self> {
        Self::from_types(projectile.data.types)
    }
}

#[derive(Debug, Clone)]
pub struct ProjectileManagerSnapshot {
    pub unlimited: Vec<ProjectileHandle>,
    pub limited: Vec<ProjectileHandle>,
    pub pending: Vec<ProjectileHandle>,
}

impl ProjectileManagerSnapshot {
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.unlimited.is_empty() && self.limited.is_empty() && self.pending.is_empty()
    }

    #[inline(always)]
    pub fn total_entries(&self) -> usize {
        self.unlimited.len() + self.limited.len() + self.pending.len()
    }

    pub fn managed_handles(&self) -> Vec<ProjectileHandle> {
        let mut handles = Vec::with_capacity(self.total_entries());
        for handle in self.unlimited.iter().copied() {
            push_unique_handle(&mut handles, handle);
        }
        for handle in self.limited.iter().copied() {
            push_unique_handle(&mut handles, handle);
        }
        for handle in self.pending.iter().copied() {
            push_unique_handle(&mut handles, handle);
        }
        handles
    }

    pub fn managed_projectiles(&self) -> Vec<Resolved<Projectile>> {
        self.managed_handles()
            .into_iter()
            .filter_map(resolve_managed_handle)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ProjectileBaseSnapshot {
    pub projectile: GamePtr<BGSProjectile>,
    pub form_id: u32,
    pub editor_id: String,
    pub display_name: String,
    pub kind: Option<ProjectileKind>,
    pub flags: EnumSet<BGSProjectileFlags, u16>,
    pub types: EnumSet<BGSProjectileType, u16>,
    pub gravity: f32,
    pub speed: f32,
    pub range: f32,
    pub tracer_chance: f32,
    pub explosion_proximity: f32,
    pub explosion_timer: f32,
    pub explosion_type: GamePtr<BGSExplosion>,
    pub cone_spread: f32,
    pub collision_radius: f32,
    pub lifetime: f32,
    pub relaunch_interval: f32,
    pub default_weapon_source: GamePtr<TESObjectWEAP>,
}

impl ProjectileBaseSnapshot {
    #[inline(always)]
    pub fn is_hitscan(&self) -> bool {
        self.flags.all(BGSProjectileFlags::HitScan)
    }

    #[inline(always)]
    pub fn has_explosion(&self) -> bool {
        self.flags.all(BGSProjectileFlags::Explosion) || self.explosion_type.is_some()
    }

    #[inline(always)]
    pub fn is_arrow(&self) -> bool {
        self.kind == Some(ProjectileKind::Arrow)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectileTargetDisposition {
    AggressiveToCaster,
    HostileToCaster,
    AnyActor,
}

#[derive(Debug, Clone, Copy)]
pub struct ProjectileTargetSearchOptions {
    pub radius: f32,
    pub disposition: ProjectileTargetDisposition,
    pub require_line_of_sight: bool,
    pub maximum_view_cone_degrees: Option<f32>,
    pub search_origin: Option<NiPoint3>,
    pub source_los_location: ACTOR_LOS_LOCATION,
    pub target_los_location: ACTOR_LOS_LOCATION,
}

impl ProjectileTargetSearchOptions {
    #[inline(always)]
    pub const fn new(radius: f32) -> Self {
        Self {
            radius,
            disposition: ProjectileTargetDisposition::HostileToCaster,
            require_line_of_sight: false,
            maximum_view_cone_degrees: None,
            search_origin: None,
            source_los_location: ACTOR_LOS_LOCATION::Head,
            target_los_location: ACTOR_LOS_LOCATION::Torso,
        }
    }

    #[inline(always)]
    pub const fn with_disposition(mut self, disposition: ProjectileTargetDisposition) -> Self {
        self.disposition = disposition;
        self
    }

    #[inline(always)]
    pub const fn with_line_of_sight(mut self, require_line_of_sight: bool) -> Self {
        self.require_line_of_sight = require_line_of_sight;
        self
    }

    #[inline(always)]
    pub const fn with_view_cone_degrees(mut self, maximum_view_cone_degrees: f32) -> Self {
        self.maximum_view_cone_degrees = Some(maximum_view_cone_degrees);
        self
    }

    #[inline(always)]
    pub const fn without_view_cone(mut self) -> Self {
        self.maximum_view_cone_degrees = None;
        self
    }

    #[inline(always)]
    pub const fn with_search_origin(mut self, search_origin: NiPoint3) -> Self {
        self.search_origin = Some(search_origin);
        self
    }

    #[inline(always)]
    pub const fn with_los_locations(
        mut self,
        source_los_location: ACTOR_LOS_LOCATION,
        target_los_location: ACTOR_LOS_LOCATION,
    ) -> Self {
        self.source_los_location = source_los_location;
        self.target_los_location = target_los_location;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProjectileTargetSnapshot {
    pub actor: GamePtr<Actor>,
    pub handle: ActorHandle,
    pub position: NiPoint3,
    pub velocity: NiPoint3,
    pub speed: f32,
    pub distance: f32,
    pub squared_distance: f32,
    pub hostile_to_caster: bool,
    pub aggressive_to_caster: bool,
    pub line_of_sight_clear: Option<bool>,
    pub view_alignment_dot: Option<f32>,
}

impl ProjectileTargetSnapshot {
    #[inline(always)]
    pub fn resolved_actor(self) -> Option<Resolved<Actor>> {
        if !self.handle.has_value() {
            None
        } else {
            Resolved::from_handle(self.handle)
        }
    }

    #[inline(always)]
    pub fn reference(self) -> GamePtr<TESObjectREFR> {
        unsafe { GamePtr::from_raw(self.actor.as_ptr().cast()) }
    }

    #[inline(always)]
    pub fn is_moving(self) -> bool {
        self.speed > f32::EPSILON
    }

    #[inline(always)]
    pub fn anticipated_position(self, delta_seconds: f32) -> Option<NiPoint3> {
        anticipated_position_from_velocity(self.position, self.velocity, delta_seconds)
    }

    #[inline(always)]
    pub fn within_view_cone(self, maximum_view_cone_degrees: f32) -> bool {
        let Some(dot) = self.view_alignment_dot else {
            return false;
        };

        dot >= view_cone_threshold_dot(maximum_view_cone_degrees)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProjectileInterceptSnapshot {
    pub origin: NiPoint3,
    pub target_position: NiPoint3,
    pub target_velocity: NiPoint3,
    pub initial_prediction_seconds: f32,
    pub travel_time_seconds: f32,
    pub projectile_speed: f32,
    pub intercept_point: NiPoint3,
    pub direction: NiPoint3,
}

impl ProjectileInterceptSnapshot {
    #[inline(always)]
    pub fn total_lead_seconds(self) -> f32 {
        self.initial_prediction_seconds + self.travel_time_seconds
    }

    #[inline(always)]
    pub fn travel_distance(self) -> f32 {
        self.origin.get_distance(self.intercept_point)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectileRetargetStrategy {
    Nearest,
    ViewAligned,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectileSteeringBehavior {
    ConstantTurnRateRadians { radians_per_second: f32 },
    ConstantAcceleration { acceleration: f32 },
}

impl ProjectileSteeringBehavior {
    #[inline(always)]
    pub const fn constant_turn_rate_radians(radians_per_second: f32) -> Self {
        Self::ConstantTurnRateRadians { radians_per_second }
    }

    #[inline(always)]
    pub const fn constant_acceleration(acceleration: f32) -> Self {
        Self::ConstantAcceleration { acceleration }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProjectileImpactSnapshot {
    pub collidee_handle: ObjectRefHandle,
    pub material: GamePtr<BGSMaterialType>,
    pub collided_layer: EnumSet<ColLayer, i32>,
    pub impact_result: ImpactResult,
    pub desired_target_loc: NiPoint3,
    pub negative_velocity: NiPoint3,
}

impl ProjectileImpactSnapshot {
    #[inline(always)]
    pub fn collidee(&self) -> GamePtr<TESObjectREFR> {
        handle_to_ptr(self.collidee_handle)
    }
}

#[derive(Debug, Clone)]
pub struct ProjectileSnapshot {
    pub projectile: GamePtr<Projectile>,
    pub handle: ProjectileHandle,
    pub kind: Option<ProjectileKind>,
    pub base: GamePtr<BGSProjectile>,
    pub base_snapshot: Option<ProjectileBaseSnapshot>,
    pub shooter_handle: ObjectRefHandle,
    pub desired_target_handle: ObjectRefHandle,
    pub spell: GamePtr<MagicItem>,
    pub av_effect: GamePtr<EffectSetting>,
    pub explosion: GamePtr<BGSExplosion>,
    pub weapon_source: GamePtr<TESObjectWEAP>,
    pub ammo_source: GamePtr<TESAmmo>,
    pub casting_source: CastingSource,
    pub position: NiPoint3,
    pub angle_x: f32,
    pub angle_z: f32,
    pub velocity: NiPoint3,
    pub linear_velocity: NiPoint3,
    pub power: f32,
    pub speed: f32,
    pub speed_mult: f32,
    pub range: f32,
    pub living_time: f32,
    pub weapon_damage: f32,
    pub transparency: f32,
    pub explosion_timer: f32,
    pub distance_moved: f32,
    pub scale: f32,
    pub flags: EnumSet<crate::re::ProjectileFlags, u32>,
    pub impact_count: u32,
}

impl ProjectileSnapshot {
    #[inline(always)]
    pub fn shooter(&self) -> GamePtr<TESObjectREFR> {
        handle_to_ptr(self.shooter_handle)
    }

    #[inline(always)]
    pub fn desired_target(&self) -> GamePtr<TESObjectREFR> {
        handle_to_ptr(self.desired_target_handle)
    }

    #[inline(always)]
    pub fn shooter_actor(&self) -> GamePtr<Actor> {
        self.shooter().try_cast::<Actor>()
    }

    #[inline(always)]
    pub fn desired_target_actor(&self) -> GamePtr<Actor> {
        self.desired_target().try_cast::<Actor>()
    }
}
