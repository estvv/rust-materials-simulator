//! Sidebar UI rendering.
//!
//! Draws the sidebar panel containing action buttons and material selection.

use raylib::prelude::*;

use crate::config::options::rendering::RenderingOptions;
use crate::core::registry::MaterialRegistry;
use crate::core::state::AppState;
use crate::render::ui::action_buttons;
use crate::render::ui::button::Button;
use crate::render::ui::materials;
use crate::utils::layout::PADDING;

pub const COLOR_SIDEBAR: Color = Color::new(245, 245, 250, 255);
pub const COLOR_SEPARATOR: Color = Color::new(220, 220, 230, 255);

/// Draws the complete sidebar including action buttons, materials list, and FPS display.
pub fn draw(
    d: &mut RaylibDrawHandle,
    sidebar_x: i32,
    sidebar_width: i32,
    state: &AppState,
    materials: &MaterialRegistry,
    buttons: &[Button],
    rendering: &RenderingOptions,
) {
    d.draw_rectangle(
        sidebar_x,
        0,
        sidebar_width,
        d.get_screen_height(),
        COLOR_SIDEBAR,
    );

    let mut y = PADDING;

    y = action_buttons::draw(d, sidebar_x, sidebar_width, y, state, buttons);

    y = draw_separator(d, sidebar_x, sidebar_width, y);

    materials::draw(d, sidebar_x, sidebar_width, y, state, materials);

    if rendering.show_fps {
        let screen_height = d.get_screen_height();
        let fps_text = format!("{} FPS", d.get_fps());
        let text_width = d.measure_text(&fps_text, 20);
        let fps_y = screen_height - 35;

        draw_separator(d, sidebar_x, sidebar_width, fps_y - 15);

        let fps_x = sidebar_x + (sidebar_width - text_width) / 2;
        d.draw_text(&fps_text, fps_x, fps_y, 20, Color::new(50, 50, 60, 200));
    }
}

/// Draws a horizontal separator line and returns the new Y position.
pub fn draw_separator(d: &mut RaylibDrawHandle, sidebar_x: i32, sidebar_width: i32, y: i32) -> i32 {
    d.draw_line(
        sidebar_x + PADDING,
        y,
        sidebar_x + sidebar_width - PADDING,
        y,
        COLOR_SEPARATOR,
    );
    y + 15
}
