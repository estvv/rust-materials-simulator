//! Action button rendering and hover detection.
//!
//! Draws the control buttons (play/pause, clear) and handles mouse hover detection.

use raylib::prelude::*;

use crate::core::state::AppState;
use crate::render::ui::button::Button;
use crate::utils::color::parse_hex;
use crate::utils::layout::{BUTTON_GAP, BUTTON_HEIGHT, FONT_SIZE_BUTTON, PADDING};

pub const COLOR_TEXT: Color = Color::new(50, 50, 60, 255);

/// Draws all action buttons and returns the Y position for the next UI element.
pub fn draw(
    d: &mut RaylibDrawHandle,
    sidebar_x: i32,
    sidebar_width: i32,
    y_start: i32,
    state: &AppState,
    buttons: &[Button],
) -> i32 {
    let button_width = (sidebar_width - PADDING * 2 - BUTTON_GAP) / 2;
    let x = sidebar_x + PADDING;

    for (i, button) in buttons.iter().enumerate() {
        let btn_x = x + (i as i32 * (button_width + BUTTON_GAP));
        let is_hovered = state.hovered_action.as_deref() == Some(&button.action);

        let base_color = parse_hex(&button.color);

        let color = if is_hovered {
            lighten_color(base_color, 30)
        } else {
            base_color
        };

        d.draw_rectangle(btn_x, y_start, button_width, BUTTON_HEIGHT, color);

        if is_hovered {
            d.draw_rectangle_lines_ex(
                Rectangle::new(
                    btn_x as f32,
                    y_start as f32,
                    button_width as f32,
                    BUTTON_HEIGHT as f32,
                ),
                2.0,
                Color::new(50, 50, 60, 255),
            );
        }

        draw_centered_text(
            d,
            &button.label,
            btn_x,
            y_start,
            button_width,
            BUTTON_HEIGHT,
            is_hovered,
        );
    }

    y_start + BUTTON_HEIGHT + 15
}

/// Returns the action string of the button currently hovered by the mouse, if any.
pub fn is_hovered(
    mouse_x: i32,
    mouse_y: i32,
    sidebar_x: i32,
    sidebar_width: i32,
    y_start: i32,
    buttons: &[Button],
) -> Option<&str> {
    let button_width = (sidebar_width - PADDING * 2 - BUTTON_GAP) / 2;
    let x = sidebar_x + PADDING;

    for (i, button) in buttons.iter().enumerate() {
        let btn_x = x + (i as i32 * (button_width + BUTTON_GAP));
        let rect = Rectangle::new(
            btn_x as f32,
            y_start as f32,
            button_width as f32,
            BUTTON_HEIGHT as f32,
        );

        if rect.check_collision_point_rec(Vector2::new(mouse_x as f32, mouse_y as f32)) {
            return Some(Box::leak(button.action.clone().into_boxed_str()));
        }
    }

    None
}

/// Draws centered text within a button rectangle.
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

    let text_color = if is_hovered {
        Color::new(30, 30, 40, 255)
    } else {
        COLOR_TEXT
    };

    d.draw_text(text, text_x, text_y, FONT_SIZE_BUTTON, text_color);
}

/// Lightens a color by the specified amount.
fn lighten_color(color: Color, amount: u8) -> Color {
    Color::new(
        (color.r as u16 + amount as u16).min(255) as u8,
        (color.g as u16 + amount as u16).min(255) as u8,
        (color.b as u16 + amount as u16).min(255) as u8,
        color.a,
    )
}
