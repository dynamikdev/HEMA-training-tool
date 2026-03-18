//! Shared constants for the HEMA Training Tool.

use bevy::color::Color;

/// Width of the configuration side panel in pixels.
pub const PANEL_WIDTH: f32 = 300.0;

/// Radius of the circle where target numbers are positioned.
pub const CIRCLE_RADIUS: f32 = 300.0;

/// The target numbers used in the training sequence, arranged in their display order.
pub const LABELS: [u8; 8] = [7, 1, 5, 3, 8, 4, 6, 2];

/// Color for active (highlighted) targets.
pub const HIGHLIGHT_COLOR: Color = Color::srgb(5.0, 0.0, 0.0);

/// Default color for inactive targets.
pub const TARGET_COLOR: Color = Color::WHITE;
