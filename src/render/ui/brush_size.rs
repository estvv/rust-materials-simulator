//! Brush size selection UI.
//!
//! Draws buttons for selecting brush size (1, 4, 7, 10).

use raylib::prelude::*;

use crate::core::state::AppState;
use crate::utils::layout::{BUTTON_GAP, FONT_SIZE_BUTTON, PADDING};

pub const BRUSH_SIZES: [u32; 4] = [1, 4, 7, 10];

const BUTTON_HEIGHT: i32 = 32;
const COLOR_BG: Color = Color::new(235, 235, 240, 255);
const COLOR_SELECTED: Color = Color::new(80, 80, 90, 255);
const COLOR_HOVER: Color = Color::new(200, 200, 210, 255);
const COLOR_TEXT: Color = Color::new(50, 50, 60, 255);

/// Draws brush size selection buttons in a row.
pub fn draw(
    d: &mut RaylibDrawHandle,
    sidebar_x: i32,
    sidebar_width: i32,
    y_start: i32,
    state: &AppState,
) {
    let button_count = BRUSH_SIZES.len() as i32;
    let total_gap = (button_count - 1) * BUTTON_GAP;
    let button_width = (sidebar_width - PADDING * 2 - total_gap) / button_count;
    let x = sidebar_x + PADDING;
    let y = y_start;

    for (i, &size) in BRUSH_SIZES.iter().enumerate() {
        let col = i as i32;
        let btn_x = x + col * (button_width + BUTTON_GAP);
        let btn_y = y;

        let is_selected = state.brush_size == size;
        let is_hovered = is_hovered(state.mouse_pos, btn_x, btn_y, button_width, BUTTON_HEIGHT);

        let color = if is_selected {
            COLOR_SELECTED
        } else if is_hovered {
            COLOR_HOVER
        } else {
            COLOR_BG
        };

        d.draw_rectangle(btn_x, btn_y, button_width, BUTTON_HEIGHT, color);

        if is_selected {
            d.draw_rectangle_lines_ex(
                Rectangle::new(
                    btn_x as f32,
                    btn_y as f32,
                    button_width as f32,
                    BUTTON_HEIGHT as f32,
                ),
                2.0,
                COLOR_SELECTED,
            );
        } else if is_hovered {
            d.draw_rectangle_lines_ex(
                Rectangle::new(
                    btn_x as f32,
                    btn_y as f32,
                    button_width as f32,
                    BUTTON_HEIGHT as f32,
                ),
                1.0,
                Color::new(150, 150, 160, 255),
            );
        }

        let text = format!("{}", size);
        let text_width = d.measure_text(&text, FONT_SIZE_BUTTON);
        let text_x = btn_x + (button_width - text_width as i32) / 2;
        let text_y = btn_y + (BUTTON_HEIGHT - FONT_SIZE_BUTTON) / 2;

        let text_color = if is_selected {
            Color::new(255, 255, 255, 255)
        } else {
            COLOR_TEXT
        };

        d.draw_text(&text, text_x, text_y, FONT_SIZE_BUTTON, text_color);
    }
}

/// Returns the brush size if the mouse is hovering over a button.
pub fn get_hovered(
    mouse_x: i32,
    mouse_y: i32,
    sidebar_x: i32,
    sidebar_width: i32,
    y_start: i32,
) -> Option<u32> {
    let button_count = BRUSH_SIZES.len() as i32;
    let total_gap = (button_count - 1) * BUTTON_GAP;
    let button_width = (sidebar_width - PADDING * 2 - total_gap) / button_count;
    let x = sidebar_x + PADDING;

    for (i, &size) in BRUSH_SIZES.iter().enumerate() {
        let col = i as i32;
        let btn_x = x + col * (button_width + BUTTON_GAP);
        let btn_y = y_start;

        if mouse_x >= btn_x
            && mouse_x < btn_x + button_width
            && mouse_y >= btn_y
            && mouse_y < btn_y + BUTTON_HEIGHT
        {
            return Some(size);
        }
    }

    None
}

fn is_hovered(mouse_pos: (i32, i32), x: i32, y: i32, width: i32, height: i32) -> bool {
    mouse_pos.0 >= x && mouse_pos.0 < x + width && mouse_pos.1 >= y && mouse_pos.1 < y + height
}
