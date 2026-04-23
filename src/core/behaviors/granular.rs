use super::{BehaviorContext, MoveResult};

pub fn find_position(ctx: &BehaviorContext) -> MoveResult {
    let x = ctx.x;
    let y = ctx.y;

    if y + 1 >= ctx.height {
        return None;
    }

    if is_empty(x, y + 1, ctx) {
        return Some((x, y + 1));
    }

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
        let idx = pseudo_random(ctx.tick, x, y) % options.len();
        Some(options[idx])
    } else {
        None
    }
}

fn is_empty(x: u32, y: u32, ctx: &BehaviorContext) -> bool {
    if x >= ctx.width || y >= ctx.height {
        return false;
    }

    let idx = (y * ctx.width + x) as usize;

    ctx.cells[idx] == 0 && ctx.new_cells[idx] == 0
}

fn pseudo_random(tick: u32, x: u32, y: u32) -> usize {
    let mut n = tick
        .wrapping_add(x.wrapping_mul(374761393))
        .wrapping_add(y.wrapping_mul(668265263));
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    n = (n ^ (n >> 16)).wrapping_mul(1274126177);
    n as usize
}
