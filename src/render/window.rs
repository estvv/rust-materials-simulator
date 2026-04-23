//! Window initialization and main render loop.
//!
//! This module handles creating the raylib window and running the main event loop
//! that processes input, updates simulation state, and renders the UI.

use raylib::prelude::*;

use crate::config::Config;
use crate::core::state::AppState;
use crate::render::ui;
use crate::utils::layout::{calculate_sidebar_width, BUTTON_HEIGHT, GRID_GAP, PADDING};

const COLOR_BACKGROUND: Color = Color::new(248, 250, 252, 255);

/// Initializes the raylib window with default settings.
///
/// Creates a 1920x1080 window with the specified title and sets the log level to error only.
pub fn init_window(title: &str) -> (RaylibHandle, RaylibThread) {
    raylib::init()
        .size(1920, 1080)
        .title(title)
        .log_level(TraceLogLevel::LOG_ERROR)
        .build()
}

/// Runs the main application loop.
///
/// This function handles:
/// - Mouse input tracking and hover state updates
/// - Button click handling for UI actions
/// - Grid click handling for cell placement
/// - Simulation updates with fixed timestep (independent of frame rate)
/// - Rendering of the grid and sidebar UI
/// - Window resize handling to adapt the world grid
pub fn run(rl: &mut RaylibHandle, thread: &RaylibThread, config: &Config, mut state: AppState) {
    let cell_size = config.options.simulation.cell_size;
    let buttons = &config.menu.buttons;
    let materials = &config.materials;

    let mut last_screen_width = 0;
    let mut last_screen_height = 0;
    let mut last_frame_time = rl.get_time();

    while !rl.window_should_close() {
        let current_time = rl.get_time();
        let frame_time = (current_time - last_frame_time) as f32;
        last_frame_time = current_time;

        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();
        let sidebar_width = calculate_sidebar_width(screen_width);

        if screen_width != last_screen_width || screen_height != last_screen_height {
            let available_width = screen_width - sidebar_width - GRID_GAP - GRID_GAP;
            let available_height = screen_height - GRID_GAP - GRID_GAP;
            if available_width > 0 && available_height > 0 {
                let grid_width = (available_width as u32) / cell_size;
                let grid_height = (available_height as u32) / cell_size;
                state.resize_world(grid_width, grid_height);
            }
            last_screen_width = screen_width;
            last_screen_height = screen_height;
        }

        let mouse_pos = rl.get_mouse_position();
        state.mouse_pos = (mouse_pos.x as i32, mouse_pos.y as i32);

        update_hover_state(&mut state, screen_width, sidebar_width, materials, buttons);

        let mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);

        if mouse_pressed {
            handle_button_click(&mut state);
        }

        if mouse_down {
            let sidebar_x = screen_width - sidebar_width;
            handle_grid_click(&mut state, sidebar_x, cell_size);
        }

        // Fixed-timestep simulation updates
        let ticks = state.accumulate_ticks(frame_time);
        for _ in 0..ticks {
            state.world.update(materials);
        }

        if state.hovered_material.is_some() || state.hovered_action.is_some() {
            rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_POINTING_HAND);
        } else if is_over_grid(
            state.mouse_pos.0,
            state.mouse_pos.1,
            screen_width - sidebar_width,
        ) {
            rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_CROSSHAIR);
        } else {
            rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
        }

        {
            let mut d = rl.begin_drawing(thread);

            draw_background(&mut d);

            let sidebar_x = d.get_screen_width() - sidebar_width;

            ui::grid::draw(&mut d, sidebar_x, &state, cell_size, materials);
            ui::sidebar::draw(
                &mut d,
                sidebar_x,
                sidebar_width,
                &state,
                materials,
                buttons,
                &config.options.rendering,
            );
        }
    }
}

/// Clears the background with the default background color.
fn draw_background(d: &mut RaylibDrawHandle) {
    d.clear_background(COLOR_BACKGROUND);
}

/// Checks if the mouse cursor is over the grid area.
fn is_over_grid(mouse_x: i32, mouse_y: i32, sidebar_x: i32) -> bool {
    mouse_x < sidebar_x - GRID_GAP && mouse_x >= GRID_GAP && mouse_y >= GRID_GAP
}

/// Updates the hover state based on mouse position.
///
/// Checks if the mouse is hovering over action buttons or material buttons
/// and updates the state accordingly.
fn update_hover_state(
    state: &mut AppState,
    screen_width: i32,
    sidebar_width: i32,
    materials: &crate::core::registry::MaterialRegistry,
    buttons: &[crate::render::ui::button::Button],
) {
    let mouse_x = state.mouse_pos.0;
    let mouse_y = state.mouse_pos.1;
    let sidebar_x = screen_width - sidebar_width;

    let y_start = PADDING;

    if let Some(action) =
        ui::action_buttons::is_hovered(mouse_x, mouse_y, sidebar_x, sidebar_width, y_start, buttons)
    {
        state.hovered_action = Some(action.to_string());
        state.hovered_material = None;
        return;
    }

    let materials_y = PADDING + BUTTON_HEIGHT + 15 + 15;

    if let Some(material_id) = ui::materials::get_hovered(
        mouse_x,
        mouse_y,
        sidebar_x,
        sidebar_width,
        materials_y,
        materials,
    ) {
        state.hovered_material = Some(material_id);
        state.hovered_action = None;
        return;
    }

    state.hovered_material = None;
    state.hovered_action = None;
}

/// Processes a button click based on the current hover state.
///
/// Executes the corresponding action for action buttons or selects
/// a material for material buttons.
fn handle_button_click(state: &mut AppState) {
    if let Some(ref action) = state.hovered_action.clone() {
        match action.as_str() {
            "toggle_simulation" => state.toggle_pause(),
            "clear_world" => state.clear_grid(),
            _ => log::info!("Unknown action: {}", action),
        }
        return;
    }

    if let Some(material_id) = state.hovered_material {
        state.select_material(material_id);
    }
}

/// Handles mouse clicks on the grid for placing cells.
///
/// Converts screen coordinates to grid coordinates and places
/// the currently selected material at that position.
fn handle_grid_click(state: &mut AppState, sidebar_x: i32, cell_size: u32) {
    if state.hovered_action.is_some()
        || !is_over_grid(state.mouse_pos.0, state.mouse_pos.1, sidebar_x)
    {
        return;
    }

    let cell_size = cell_size as i32;
    let grid_x = (state.mouse_pos.0 - GRID_GAP) / cell_size;
    let grid_y = (state.mouse_pos.1 - GRID_GAP) / cell_size;

    if grid_x >= 0 && grid_y >= 0 {
        let grid_x = grid_x as u32;
        let grid_y = grid_y as u32;
        if grid_x < state.world.width && grid_y < state.world.height {
            state.on_grid_click(grid_x, grid_y);
        }
    }
}
