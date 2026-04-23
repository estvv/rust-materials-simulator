//! Position type for coordinate definitions.
//!
//! Simple 2D position structure used in configuration files.

use serde::Deserialize;

/// A 2D position with integer coordinates.
#[derive(Deserialize, Debug, Clone)]
pub struct Position {
    /// The x-coordinate.
    pub x: i32,
    /// The y-coordinate.
    pub y: i32,
}
