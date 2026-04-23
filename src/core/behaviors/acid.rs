use super::{BehaviorContext, MoveResult};

pub fn find_position(ctx: &BehaviorContext) -> MoveResult {
    let x = ctx.x;
    let y = ctx.y;
    let can_down = y + 1 < ctx.height;

    if can_down && can_move_to(x, y + 1, ctx) {
        return Some((x, y + 1));
    }

    let can_left = x > 0;
    let can_right = x + 1 < ctx.width;

    let mut below_options: Vec<(u32, u32)> = Vec::new();

    if can_down && can_left && can_move_to(x - 1, y + 1, ctx) {
        below_options.push((x - 1, y + 1));
    }
    if can_down && can_right && can_move_to(x + 1, y + 1, ctx) {
        below_options.push((x + 1, y + 1));
    }

    if !below_options.is_empty() {
        let idx = pseudo_random(ctx.tick, x, y, 0) % below_options.len();
        return Some(below_options[idx]);
    }

    let mut side_options: Vec<(u32, u32)> = Vec::new();

    if can_left && can_move_to(x - 1, y, ctx) {
        side_options.push((x - 1, y));
    }
    if can_right && can_move_to(x + 1, y, ctx) {
        side_options.push((x + 1, y));
    }

    if !side_options.is_empty() {
        let settle_chance = pseudo_random(ctx.tick, x, y, 1) % 4;

        if settle_chance == 0 {
            None
        } else {
            let idx = pseudo_random(ctx.tick, x, y, 2) % side_options.len();
            Some(side_options[idx])
        }
    } else {
        None
    }
}

fn can_move_to(x: u32, y: u32, ctx: &BehaviorContext) -> bool {
    if x >= ctx.width || y >= ctx.height {
        return false;
    }

    let target_index = (y * ctx.width + x) as usize;

    let target_original = ctx.cells[target_index];
    let target_new = ctx.new_cells[target_index];

    if target_original == 0 && target_new == 0 {
        return true;
    }

    let target_id = if target_new != 0 {
        target_new
    } else if target_original != 0 {
        target_original
    } else {
        return true;
    };

    if target_id == ctx.material_id {
        return false;
    }

    let moving_density = match ctx.materials.get_density(ctx.material_id) {
        Some(d) => d,
        None => return false,
    };

    let target_density = match ctx.materials.get_density(target_id) {
        Some(d) => d,
        None => return false,
    };

    moving_density > target_density
}

fn pseudo_random(tick: u32, x: u32, y: u32, seed: u32) -> usize {
    let mut n = tick
        .wrapping_add(x.wrapping_mul(374761393))
        .wrapping_add(y.wrapping_mul(668265263))
        .wrapping_add(seed.wrapping_mul(1274126177));
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    n = (n ^ (n >> 16)).wrapping_mul(1274126177);
    n as usize
}
