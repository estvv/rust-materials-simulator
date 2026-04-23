//! Granular material behavior (sand, dust, etc.)
//!
//! Granular materials fall straight down when possible, and spread
//! diagonally when blocked. Density-based displacement is handled by world.rs.

use super::{BehaviorContext, MoveResult};
use std::time::{SystemTime, UNIX_EPOCH};

/// Find the next position for a granular particle.
///
/// Priority:
/// 1. Fall straight down (if empty)
/// 2. Fall diagonally (left or right, chosen randomly)
pub fn find_position(ctx: &BehaviorContext) -> MoveResult {
    let x = ctx.x;
    let y = ctx.y;

    // Check if at bottom
    if y + 1 >= ctx.height {
        return None;
    }

    // Try to fall straight down
    if is_empty(x, y + 1, ctx) {
        return Some((x, y + 1));
    }

    // Try to fall diagonally
    let can_left = x > 0;
    let can_right = x + 1 < ctx.width;

    let mut options: Vec<(u32, u32)> = Vec::new();

    if can_left && is_empty(x - 1, y + 1, ctx) {
        options.push((x - 1, y + 1));
    }
    if can_right && is_empty(x + 1, y + 1, ctx) {
        options.push((x + 1, y + 1));
    }

    if !options.is_empty() {
        Some(options[rand_index(options.len())])
    } else {
        None
    }
}

/// Check if a position is empty (can move into).
fn is_empty(x: u32, y: u32, ctx: &BehaviorContext) -> bool {
    if x >= ctx.width || y >= ctx.height {
        return false;
    }

    let idx = (y * ctx.width + x) as usize;

    // Empty if both original and new cells are 0
    ctx.cells[idx] == 0 && ctx.new_cells[idx] == 0
}

fn rand_index(len: usize) -> usize {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize;
    nanos % len
}
