//! Gameplay-oriented helpers built on top of the RE gameplay surface.
//!
//! This module is organized around plugin workflows rather than raw engine
//! class families. Use it when you want higher-level gameplay helpers such as:
//!
//! - actor/player/camera/combat queries
//! - input gestures and directional intent
//! - inventory/equipment and magic inspection
//! - quest/objective/alias/stage state
//! - projectile launch, targeting, and steering
//! - world/pathing/spatial scans
//!
//! The current surface is intentionally query-first with targeted mutation
//! helpers where repeated plugin patterns are already clear. When a workflow is
//! still too plugin-specific or too data-driven, the SDK prefers to stop at
//! honest building blocks instead of pretending to provide one universal
//! gameplay framework.

pub mod actors;
pub mod camera;
pub mod combat;
pub mod input;
pub mod inventory;
pub mod magic;
pub mod movement;
pub mod navmesh;
pub mod pathing;
pub mod player;
pub mod projectiles;
pub mod quests;
pub mod spatial;
pub mod world;
