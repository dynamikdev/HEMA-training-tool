//! Global constant definitions for the HEMA Training Tool.
//!
//! This module contains shared configuration values that define the visual
//! layout, default colors, and the logical ordering of training targets.

use bevy::color::Color;

/// The fixed width (in pixels) of the interactive settings panel on the right.
///
/// This value is used by layout systems to calculate the remaining available
/// space for the training circle.
pub const PANEL_WIDTH: f32 = 300.0;

/// The base radius (in pixels) of the circle where target numbers are positioned.
///
/// While the layout is now dynamic, this constant may still serve as a reference
/// value for coordinate calculations.
pub const CIRCLE_RADIUS: f32 = 300.0;

/// The human-readable labels assigned to the 8 target positions.
///
/// These numbers are arranged in a specific order starting from the top position
/// (Index 0) and proceeding clockwise. This layout typically corresponds to
/// standard HEMA target diagrams (e.g., Meyer's Square).
pub const LABELS: [u8; 8] = [7, 1, 5, 3, 8, 4, 6, 2];

/// The default color used for targets when they are not being highlighted.
///
/// A neutral white is used to ensure high contrast against the dark background.
pub const TARGET_COLOR: Color = Color::WHITE;

/// The color used to highlight the currently active target number.
///
/// This red color has high-intensity components to trigger the Bloom glow
/// effect, making the active target clearly visible.
pub const HIGHLIGHT_COLOR: Color = Color::linear_rgb(20.0, 0.0, 0.0);
