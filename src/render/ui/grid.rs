//! Grid rendering module.
//!
//! Handles drawing the simulation grid, cells, and brush preview overlay.

use raylib::prelude::*;

use crate::core::registry::MaterialRegistry;
use crate::core::state::AppState;
use crate::utils::color::parse_hex;
use crate::utils::layout::GRID_GAP;

pub const COLOR_GRID_BG: Color = Color::new(250, 250, 252, 255);
pub const COLOR_GRID_BORDER: Color = Color::new(200, 200, 210, 255);
pub const COLOR_BRUSH_PREVIEW: Color = Color::new(100, 150, 255, 80);
pub const COLOR_BRUSH_BORDER: Color = Color::new(100, 150, 255, 200);

/// Draws the simulation grid including background, border, cells, and brush preview.
///
/// The grid fills the entire available space from the left edge to the sidebar.
pub fn draw(
    d: &mut RaylibDrawHandle,
    sidebar_x: i32,
    state: &AppState,
    cell_size: u32,
    materials: &MaterialRegistry,
) {
    let grid_width = sidebar_x - GRID_GAP - GRID_GAP;
    let grid_height = d.get_screen_height() - GRID_GAP - GRID_GAP;

    d.draw_rectangle(GRID_GAP, GRID_GAP, grid_width, grid_height, COLOR_GRID_BG);

    d.draw_rectangle_lines_ex(
        Rectangle::new(
            GRID_GAP as f32,
            GRID_GAP as f32,
            grid_width as f32,
            grid_height as f32,
        ),
        2.0,
        COLOR_GRID_BORDER,
    );

    draw_cells(d, state, cell_size, materials);
    draw_brush_preview(d, sidebar_x, state, cell_size);
}

/// Renders all cells in the world with their material colors.
fn draw_cells(
    d: &mut RaylibDrawHandle,
    state: &AppState,
    cell_size: u32,
    materials: &MaterialRegistry,
) {
    for (x, y, material_id) in state.world.cells() {
        if let Some(mat) = materials.get(material_id) {
            let screen_x = GRID_GAP + (x * cell_size) as i32;
            let screen_y = GRID_GAP + (y * cell_size) as i32;

            d.draw_rectangle(
                screen_x,
                screen_y,
                cell_size as i32,
                cell_size as i32,
                parse_hex(&mat.color),
            );
        }
    }
}

/// Draws a semi-transparent preview showing where the brush will place cells.
fn draw_brush_preview(d: &mut RaylibDrawHandle, sidebar_x: i32, state: &AppState, cell_size: u32) {
    if state.mouse_pos.0 >= sidebar_x - GRID_GAP {
        return;
    }

    let cell_size = cell_size as i32;
    let grid_x = (state.mouse_pos.0 - GRID_GAP) / cell_size;
    let grid_y = (state.mouse_pos.1 - GRID_GAP) / cell_size;

    if grid_x >= 0
        && grid_y >= 0
        && grid_x < state.world.width as i32
        && grid_y < state.world.height as i32
    {
        let preview_x = GRID_GAP + grid_x * cell_size;
        let preview_y = GRID_GAP + grid_y * cell_size;

        d.draw_rectangle(
            preview_x,
            preview_y,
            cell_size,
            cell_size,
            COLOR_BRUSH_PREVIEW,
        );

        d.draw_rectangle_lines_ex(
            Rectangle::new(
                preview_x as f32,
                preview_y as f32,
                cell_size as f32,
                cell_size as f32,
            ),
            1.0,
            COLOR_BRUSH_BORDER,
        );
    }
}

/// Checks if the given mouse coordinates are over the grid area.
pub fn is_over_grid(mouse_x: i32, mouse_y: i32, sidebar_x: i32) -> bool {
    mouse_x < sidebar_x - GRID_GAP && mouse_x >= GRID_GAP && mouse_y >= GRID_GAP
}
