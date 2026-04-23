//! Configuration file loaders.
//!
//! Functions for loading various configuration files from TOML format.

use log::error;
use serde::Deserialize;
use std::{fs, process::exit};

use crate::config::config::{Config};
use crate::config::options::Options;
use crate::core::material::Material;
use crate::core::registry::MaterialRegistry;
use crate::render::ui::menu::Menu;

/// Loads the materials registry from a TOML file.
///
/// Parses the materials configuration and registers all materials.
/// Exits with code 1 if the file cannot be read or parsed.
pub fn load_materials(filepath: &str) -> MaterialRegistry {
    log::info!("Parsing materials from: {}", filepath);

    let content = fs::read_to_string(filepath).unwrap_or_else(|_| {
        error!("Could not read materials config: {}", filepath);
        exit(1);
    });

    #[derive(Deserialize)]
    struct MaterialsFile {
        material: Vec<Material>,
    }

    let config: MaterialsFile = toml::from_str(&content).unwrap_or_else(|e| {
        error!("TOML parse error in {}: {}", filepath, e);
        exit(1);
    });

    let mut materials = MaterialRegistry::new();
    for mat in config.material {
        log::debug!(
            "  Parsed material: {} (id={}, behavior={})",
            mat.name,
            mat.id,
            mat.behavior
        );
        materials.register(mat);
    }

    log::info!("Loaded {} materials", materials.len());

    materials
}

/// Loads the menu configuration from a TOML file.
///
/// Parses the menu buttons configuration.
/// Exits with code 1 if the file cannot be read or parsed.
pub fn load_menu(filepath: &str) -> Menu {
    log::info!("Parsing menu from: {}", filepath);

    let content = fs::read_to_string(filepath).unwrap_or_else(|_| {
        error!("Could not read menu config: {}", filepath);
        exit(1);
    });

    let config: Menu = toml::from_str(&content).unwrap_or_else(|e| {
        error!("TOML parse error in {}: {}", filepath, e);
        exit(1);
    });

    for btn in &config.buttons {
        log::debug!("  Parsed button: {} (action={})", btn.label, btn.action);
    }

    log::info!("Loaded {} menu buttons", config.buttons.len());
    config
}

/// Loads the options configuration from a TOML file.
///
/// Parses simulation, rendering, and interaction options.
/// Exits with code 1 if the file cannot be read or parsed.
pub fn load_options(filepath: &str) -> Options {
    log::info!("Parsing options from: {}", filepath);

    let content = fs::read_to_string(filepath).unwrap_or_else(|_| {
        error!("Could not read options config: {}", filepath);
        exit(1);
    });

    let config: Options = toml::from_str(&content).unwrap_or_else(|e| {
        error!("TOML parse error in {}: {}", filepath, e);
        exit(1);
    });

    log::debug!(
        "  Grid: {}x{}, cell_size: {}",
        config.simulation.grid_width,
        config.simulation.grid_height,
        config.simulation.cell_size
    );
    log::debug!(
        "  FPS limit: {}, show_grid: {}, show_fps: {}",
        config.simulation.fps_limit,
        config.rendering.show_grid,
        config.rendering.show_fps
    );
    log::debug!(
        "  Brush size: {}, default material: {}",
        config.interaction.brush_size,
        config.interaction.default_material
    );

    log::info!("Loaded options config");
    config
}

/// Loads all configuration files and returns a complete Config struct.
///
/// This is the main entry point for configuration loading, combining
/// materials, menu, options, and start configuration.
pub fn load_all(
    materials_path: &str,
    menu_path: &str,
    options_path: &str
) -> Config {
    Config {
        materials: load_materials(materials_path),
        menu: load_menu(menu_path),
        options: load_options(options_path)
    }
}
