//! Options configuration module.
//!
//! Contains all configuration options for simulation, rendering, and user interaction.

pub mod interaction;
pub mod rendering;
pub mod simulation;

pub use interaction::InteractionOptions;
pub use rendering::RenderingOptions;
use serde::Deserialize;
pub use simulation::SimulationOptions;

/// Combined options configuration loaded from `config/options.toml`.
#[derive(Deserialize, Debug, Clone)]
pub struct Options {
    /// Grid and simulation settings.
    pub simulation: SimulationOptions,
    /// Visual rendering settings.
    pub rendering: RenderingOptions,
    /// User interaction settings.
    pub interaction: InteractionOptions,
}
