//! Menu configuration loaded from TOML.
//!
//! Defines the structure for menu button configuration files.

use serde::Deserialize;

use crate::render::ui::button::Button;

/// A menu containing a list of action buttons.
///
/// Loaded from `config/menu.toml`.
#[derive(Deserialize, Debug, Clone)]
pub struct Menu {
    /// The list of buttons in this menu.
    pub buttons: Vec<Button>,
}
