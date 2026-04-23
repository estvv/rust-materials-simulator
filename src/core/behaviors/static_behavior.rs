//! Static material behavior (walls, stone, etc.)
//!
//! Static materials do not move and cannot be displaced.

use super::{BehaviorContext, MoveResult};

pub fn find_position(_ctx: &BehaviorContext) -> MoveResult {
    None
}
