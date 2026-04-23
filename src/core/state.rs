//! Application state management.
//!
//! This module defines the central state that tracks all user interactions,
//! simulation state, and world data during runtime.

use crate::core::world::World;

/// The central application state that tracks all runtime data.
///
/// This struct holds the simulation world, UI state (selected material, hover state),
/// and simulation control state (paused/running).
pub struct AppState {
    /// The currently selected material ID for placing cells.
    pub selected_material: u32,
    /// Whether the simulation is currently paused.
    pub paused: bool,
    /// Current mouse position in screen coordinates.
    pub mouse_pos: (i32, i32),
    /// The material ID currently being hovered over in the sidebar, if any.
    pub hovered_material: Option<u32>,
    /// The action button currently being hovered over, if any.
    pub hovered_action: Option<String>,
    /// The simulation world containing all cells.
    pub world: World,
    /// Accumulated time for fixed-timestep simulation updates.
    pub update_accumulator: f32,
    /// Fixed timestep duration in seconds (how often simulation updates).
    pub tick_duration: f32,
}

impl AppState {
    /// Creates a new application state with the specified default material and grid dimensions.
    ///
    /// The simulation starts in a paused state with no cells in the world.
    /// Defaults to 60 ticks per second for simulation updates.
    pub fn new(default_material: u32, grid_width: u32, grid_height: u32) -> Self {
        AppState {
            selected_material: default_material,
            paused: false,
            mouse_pos: (0, 0),
            hovered_material: None,
            hovered_action: None,
            world: World::new(grid_width, grid_height),
            update_accumulator: 0.0,
            tick_duration: 1.0 / 60.0,
        }
    }

    /// Sets the currently selected material for placing cells.
    pub fn select_material(&mut self, material_id: u32) {
        self.selected_material = material_id;
        log::info!("Selected material: {}", material_id);
    }

    /// Clears all cells from the world grid.
    pub fn clear_grid(&mut self) {
        self.world.clear();
        log::warn!("Cleared the grid");
    }

    /// Resizes the world grid to the specified dimensions.
    pub fn resize_world(&mut self, width: u32, height: u32) {
        self.world.resize(width, height);
    }

    /// Toggles between paused and running simulation states.
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        log::info!("Toggle pause callback: {}", self.paused);
    }

    /// Handles a click on the grid, placing the selected material at the given position.
    pub fn on_grid_click(&mut self, grid_x: u32, grid_y: u32) {
        self.world.set_cell(grid_x, grid_y, self.selected_material);
        log::info!(
            "Set cell ({}, {}) to material {}",
            grid_x,
            grid_y,
            self.selected_material
        );
    }

    /// Accumulates frame time and returns the number of simulation ticks to run.
    ///
    /// This implements a fixed-timestep loop where the simulation updates
    /// at a constant rate (default 60 ticks/second) regardless of frame rate.
    pub fn accumulate_ticks(&mut self, frame_time: f32) -> u32 {
        if self.paused {
            return 0;
        }

        self.update_accumulator += frame_time;

        let mut ticks = 0;
        while self.update_accumulator >= self.tick_duration {
            self.update_accumulator -= self.tick_duration;
            ticks += 1;
        }

        ticks
    }
}
