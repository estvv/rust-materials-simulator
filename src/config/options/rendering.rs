//! Rendering configuration.
//!
//! Settings controlling visual aspects of the simulation.

use serde::Deserialize;

/// Rendering options for display settings.
#[derive(Deserialize, Debug, Clone)]
pub struct RenderingOptions {
    /// Background color in hex format (e.g., "#FFFFFF").
    pub background_color: String,
    /// Whether to show grid lines (currently unused).
    pub show_grid: bool,
    /// Whether to display FPS counter in the sidebar.
    pub show_fps: bool,
}
