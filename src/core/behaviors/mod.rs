//! Behavior module for material movement rules.
//!
//! This module defines the behavior trait and implementations for different
//! material types. Each behavior determines how particles move and interact.

pub mod granular;
pub mod liquid;
pub mod static_behavior;

use crate::core::behavior::Behavior;
use crate::core::registry::MaterialRegistry;

pub type MoveResult = Option<(u32, u32)>;

pub struct BehaviorContext<'a> {
    pub x: u32,
    pub y: u32,
    pub material_id: u32,
    pub tick: u32,
    pub width: u32,
    pub height: u32,
    pub cells: &'a [u32],
    pub new_cells: &'a [u32],
    pub materials: &'a MaterialRegistry,
}

pub trait MovementBehavior {
    fn find_position(&self, ctx: &BehaviorContext) -> MoveResult;
}

/// Dispatch movement behavior based on material type.
pub fn dispatch(behavior: &Behavior, ctx: &BehaviorContext) -> MoveResult {
    match behavior {
        Behavior::Granular => granular::find_position(ctx),
        Behavior::Liquid => liquid::find_position(ctx),
        Behavior::Static | Behavior::Solid | Behavior::Burning => {
            static_behavior::find_position(ctx)
        }
    }
}
