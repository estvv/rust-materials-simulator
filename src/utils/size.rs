//! Size type for dimension definitions.
//!
//! Simple 2D size structure used in configuration files.

use serde::Deserialize;

/// A 2D size with width and height.
#[derive(Deserialize, Debug, Clone)]
pub struct Size {
    /// The width dimension.
    pub width: i32,
    /// The height dimension.
    pub height: i32,
}
