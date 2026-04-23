//! Simulation configuration.
//!
//! Settings controlling the simulation grid and update behavior.

use serde::Deserialize;

/// Simulation options for grid and timing settings.
#[derive(Deserialize, Debug, Clone)]
pub struct SimulationOptions {
    /// Width of the simulation grid in cells.
    pub grid_width: u32,
    /// Height of the simulation grid in cells.
    pub grid_height: u32,
    /// Size of each cell in pixels.
    pub cell_size: u32,
    /// Maximum frames per second for the simulation.
    pub fps_limit: u32,
}
