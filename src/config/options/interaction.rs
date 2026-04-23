//! User interaction configuration.
//!
//! Settings controlling how the user interacts with the simulation.

use serde::Deserialize;

/// Interaction options for user input handling.
#[derive(Deserialize, Debug, Clone)]
pub struct InteractionOptions {
    /// Size of the brush for placing materials (currently unused).
    pub brush_size: u32,
    /// Default material ID selected on startup.
    pub default_material: u32,
}
