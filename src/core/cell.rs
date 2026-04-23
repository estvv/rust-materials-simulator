//! Cell definition for grid positions.
//!
//! A cell represents a single position on the simulation grid with its associated material.

use serde::Deserialize;

/// Represents a single cell in the simulation grid.
///
/// Each cell has a position (x, y) and references a material by its ID.
/// Used primarily for loading initial cell configurations from TOML files.
#[derive(Deserialize, Debug, Clone)]
pub struct Cell {
    /// The x-coordinate (column) of the cell in the grid.
    pub x: u32,
    /// The y-coordinate (row) of the cell in the grid.
    pub y: u32,
    /// The ID of the material occupying this cell.
    pub material_id: u32,
}
