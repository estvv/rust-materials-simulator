//! Layout constants and calculations.
//!
//! Provides constants and functions for UI layout positioning and sizing.

/// Gap between the grid and window edges in pixels.
pub const GRID_GAP: i32 = 20;
/// Standard padding for UI elements in pixels.
pub const PADDING: i32 = 15;
/// Gap between buttons in pixels.
pub const BUTTON_GAP: i32 = 10;

/// Font size for section titles in pixels.
pub const FONT_SIZE_TITLE: i32 = 20;
/// Font size for button text in pixels.
pub const FONT_SIZE_BUTTON: i32 = 20;

/// Height of action buttons in pixels.
pub const BUTTON_HEIGHT: i32 = 32;
/// Height of material selection buttons in pixels.
pub const MATERIAL_BUTTON_HEIGHT: i32 = 36;

/// Calculates the sidebar width based on screen width.
///
/// Returns a value between 200 and 300 pixels, approximately 1/5 of screen width.
pub fn calculate_sidebar_width(screen_width: i32) -> i32 {
    let calculated = screen_width / 5;
    if calculated < 200 {
        200
    } else if calculated > 300 {
        300
    } else {
        calculated
    }
}

/// Calculates the width of a button based on sidebar width.
///
/// Accounts for padding and gap between two buttons.
pub fn calculate_button_width(sidebar_width: i32) -> i32 {
    (sidebar_width - PADDING * 2 - BUTTON_GAP) / 2
}

/// Converts grid coordinates to screen pixel coordinates.
///
/// Returns (screen_x, screen_y) for the top-left corner of the cell.
pub fn grid_to_screen(grid_x: i32, grid_y: i32, cell_size: i32) -> (i32, i32) {
    (GRID_GAP + grid_x * cell_size, GRID_GAP + grid_y * cell_size)
}

/// Converts screen pixel coordinates to grid coordinates.
///
/// Returns (grid_x, grid_y) for the cell under the screen position.
pub fn screen_to_grid(screen_x: i32, screen_y: i32, cell_size: i32) -> (i32, i32) {
    (
        (screen_x - GRID_GAP) / cell_size,
        (screen_y - GRID_GAP) / cell_size,
    )
}
