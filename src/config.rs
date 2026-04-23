//! Configuration loading and management.
//!
//! This module handles loading all configuration from TOML files including
//! materials, menu buttons, simulation options, and initial state.

pub mod config;
pub mod loader;
pub mod options;

pub use config::Config;
pub use loader::load_all;
