//! Button definition for UI actions.
//!
//! Defines the structure for configurable action buttons in the UI.

use serde::Deserialize;

/// Represents a clickable button in the UI.
///
/// Buttons have an ID, display label, action identifier, and background color.
/// Loaded from configuration files.
#[derive(Deserialize, Debug, Clone)]
pub struct Button {
    /// Unique identifier for this button.
    pub id: String,
    /// Display text shown on the button.
    pub label: String,
    /// Action identifier executed when clicked (e.g., "toggle_simulation").
    pub action: String,
    /// Hex color string for the button background (e.g., "#3498db").
    pub color: String,
}
