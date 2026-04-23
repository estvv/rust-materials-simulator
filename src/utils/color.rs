//! Color manipulation utilities.
//!
//! Provides functions for parsing colors and performing color operations
//! like lightening, darkening, and interpolation.

use raylib::prelude::*;

/// Parses a hex color string into a raylib Color.
///
/// Accepts strings in the format "#RRGGBB" or "RRGGBB".
/// Returns black if the string is invalid.
pub fn parse_hex(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        Color::new(r, g, b, 255)
    } else {
        Color::BLACK
    }
}

/// Lightens a color by adding the specified amount to each RGB component.
///
/// Values are clamped at 255 to prevent overflow.
pub fn lighten(color: Color, amount: u8) -> Color {
    Color::new(
        (color.r as u16 + amount as u16).min(255) as u8,
        (color.g as u16 + amount as u16).min(255) as u8,
        (color.b as u16 + amount as u16).min(255) as u8,
        color.a,
    )
}

/// Darkens a color by subtracting the specified amount from each RGB component.
///
/// Values are clamped at 0 to prevent underflow.
pub fn darken(color: Color, amount: u8) -> Color {
    Color::new(
        color.r.saturating_sub(amount),
        color.g.saturating_sub(amount),
        color.b.saturating_sub(amount),
        color.a,
    )
}

/// Returns a new color with the specified alpha value.
pub fn with_alpha(color: Color, alpha: u8) -> Color {
    Color::new(color.r, color.g, color.b, alpha)
}

/// Linearly interpolates between two colors.
///
/// The parameter `t` should be between 0.0 and 1.0, where 0.0 returns `a`
/// and 1.0 returns `b`.
pub fn lerp(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::new(
        (a.r as f32 + (b.r as f32 - a.r as f32) * t) as u8,
        (a.g as f32 + (b.g as f32 - a.g as f32) * t) as u8,
        (a.b as f32 + (b.b as f32 - a.b as f32) * t) as u8,
        (a.a as f32 + (b.a as f32 - a.a as f32) * t) as u8,
    )
}
