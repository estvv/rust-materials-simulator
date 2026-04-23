//! Dynamic Materials - A particle simulation application.
//!
//! This application provides a grid-based simulation where different materials
//! interact with each other. Materials can have behaviors like granular (sand-like)
//! or liquid, and they update their positions accordingly.

mod config;
mod core;
mod render;
mod utils;

use core::state::AppState;

/// Application entry point.
///
/// Initializes logging, loads configuration files, creates the application state,
/// initializes the rendering window, and starts the main event loop.
fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cfg = config::load_all(
        "config/materials.toml",
        "config/menu.toml",
        "config/options.toml"
    );

    let state = AppState::new(
        cfg.options.interaction.default_material,
        cfg.options.simulation.grid_width,
        cfg.options.simulation.grid_height,
    );

    let (mut rl, thread) = render::window::init_window("Dynamic Materials");

    render::window::run(&mut rl, &thread, &cfg, state);
}
