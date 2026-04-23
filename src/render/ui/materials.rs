//! Material selection button rendering.
//!
//! Draws the grid of material selection buttons and handles hover detection.

use raylib::prelude::*;

use crate::core::registry::MaterialRegistry;
use crate::core::state::AppState;
use crate::utils::color::{lighten, parse_hex};
use crate::utils::layout::{BUTTON_GAP, FONT_SIZE_BUTTON, MATERIAL_BUTTON_HEIGHT, PADDING};

pub const COLOR_TEXT: Color = Color::new(50, 50, 60, 255);
pub const COLOR_SELECTED: Color = Color::new(80, 80, 90, 255);
pub const COLOR_HOVER_BORDER: Color = Color::new(80, 80, 90, 255);

/// Draws all material selection buttons in a two-column grid layout.
pub fn draw(
    d: &mut RaylibDrawHandle,
    sidebar_x: i32,
    sidebar_width: i32,
    y_start: i32,
    state: &AppState,
    materials: &MaterialRegistry,
) {
    let button_width = (sidebar_width - PADDING * 2 - BUTTON_GAP) / 2;
    let x = sidebar_x + PADDING;
    let y = y_start;

    let items: Vec<_> = materials.all().collect();

    for (i, mat) in items.iter().enumerate() {
        let color = parse_hex(&mat.color);
        let col = i % 2;
        let row = i / 2;

        let btn_x = x + (col as i32 * (button_width + BUTTON_GAP));
        let btn_y = y + (row as i32 * (MATERIAL_BUTTON_HEIGHT + BUTTON_GAP));

        let is_hovered = state.hovered_material == Some(mat.id);
        let is_selected = state.selected_material == mat.id;

        let final_color = if is_hovered {
            lighten(color, 40)
        } else if is_selected {
            lighten(color, 20)
        } else {
            color
        };

        d.draw_rectangle(
            btn_x,
            btn_y,
            button_width,
            MATERIAL_BUTTON_HEIGHT,
            final_color,
        );

        if is_selected {
            d.draw_rectangle_lines_ex(
                Rectangle::new(
                    btn_x as f32,
                    btn_y as f32,
                    button_width as f32,
                    MATERIAL_BUTTON_HEIGHT as f32,
                ),
                3.0,
                COLOR_SELECTED,
            );
        } else if is_hovered {
            d.draw_rectangle_lines_ex(
                Rectangle::new(
                    btn_x as f32,
                    btn_y as f32,
                    button_width as f32,
                    MATERIAL_BUTTON_HEIGHT as f32,
                ),
                2.0,
                COLOR_HOVER_BORDER,
            );
        }

        draw_centered_text(
            d,
            &mat.name,
            btn_x,
            btn_y,
            button_width,
            MATERIAL_BUTTON_HEIGHT,
            is_hovered,
        );
    }
}

/// Returns the material ID of the button currently hovered by the mouse, if any.
pub fn get_hovered(
    mouse_x: i32,
    mouse_y: i32,
    sidebar_x: i32,
    sidebar_width: i32,
    y_start: i32,
    materials: &MaterialRegistry,
) -> Option<u32> {
    let button_width = (sidebar_width - PADDING * 2 - BUTTON_GAP) / 2;
    let x = sidebar_x + PADDING;

    let items: Vec<_> = materials.all().collect();

    for (i, mat) in items.iter().enumerate() {
        let col = i % 2;
        let row = i / 2;

        let btn_x = x + (col as i32 * (button_width + BUTTON_GAP));
        let btn_y = y_start + (row as i32 * (MATERIAL_BUTTON_HEIGHT + BUTTON_GAP));

        let rect = Rectangle::new(
            btn_x as f32,
            btn_y as f32,
            button_width as f32,
            MATERIAL_BUTTON_HEIGHT as f32,
        );

        if rect.check_collision_point_rec(Vector2::new(mouse_x as f32, mouse_y as f32)) {
            return Some(mat.id);
        }
    }

    None
}

/// Draws centered text within a material button.
fn draw_centered_text(
    d: &mut RaylibDrawHandle,
    text: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    is_hovered: bool,
) {
    let text_width = d.measure_text(text, FONT_SIZE_BUTTON);
    let text_x = x + (width - text_width as i32) / 2;
    let text_y = y + (height - FONT_SIZE_BUTTON) / 2;

    if is_hovered {
        d.draw_text(
            text,
            text_x,
            text_y,
            FONT_SIZE_BUTTON,
            Color::new(30, 30, 40, 255),
        );
    } else {
        d.draw_text(text, text_x, text_y, FONT_SIZE_BUTTON, COLOR_TEXT);
    }
}
