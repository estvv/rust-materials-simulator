//! Main configuration structures.
//!
//! Defines the top-level configuration types that aggregate all other settings.

use serde::Deserialize;

use crate::config::options::Options;
use crate::core::cell::Cell;
use crate::core::registry::MaterialRegistry;
use crate::render::ui::menu::Menu;

/// The complete application configuration.
///
/// Aggregates all configuration loaded from various TOML files.
pub struct Config {
    /// Registry of all available materials.
    pub materials: MaterialRegistry,
    /// UI menu button configuration.
    pub menu: Menu,
    /// Simulation and rendering options.
    pub options: Options,
}
