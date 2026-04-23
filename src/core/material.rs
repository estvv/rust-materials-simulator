//! Material definition for the simulation.
//!
//! Materials define the types of particles that can exist in the simulation,
//! including their visual appearance and behavioral properties.

use serde::Deserialize;

use crate::core::behavior::Behavior;

/// Defines a material type in the simulation.
///
/// Materials are loaded from configuration files and registered in the material registry.
/// Each material has a unique ID, display name, color, and behavior type.
#[derive(Deserialize, Debug, Clone)]
pub struct Material {
    /// Unique identifier for this material type.
    pub id: u32,
    /// Human-readable name displayed in the UI.
    pub name: String,
    /// Hex color string (e.g., "#FF5500") for rendering this material.
    pub color: String,
    /// The behavioral type that determines how this material moves.
    pub behavior: Behavior,
    /// The density of this material.
    pub density: f32,
}
