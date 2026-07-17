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

/// The four concentric sequences of Meyer's Square.
///
/// Organized from outer to inner, with each sequence containing a 4-strike order.
/// Coordinates are normalized from -1.0 to 1.0, with (0,0) at the center.
pub const MEYER_SEQUENCES: [crate::resources::MeyerSequence; 4] = [
    // Outer Sequence
    crate::resources::MeyerSequence {
        nodes: [
            crate::resources::MeyerNode { x: -1.0, y: 1.0, technique: crate::resources::TechniqueType::Cut },
            crate::resources::MeyerNode { x: 1.0, y: -1.0, technique: crate::resources::TechniqueType::Cut },
            crate::resources::MeyerNode { x: -1.0, y: -1.0, technique: crate::resources::TechniqueType::Cut },
            crate::resources::MeyerNode { x: 1.0, y: 1.0, technique: crate::resources::TechniqueType::Cut },
        ],
    },
    // Outer-Mid Sequence
    crate::resources::MeyerSequence {
        nodes: [
            crate::resources::MeyerNode { x: -0.66, y: 0.66, technique: crate::resources::TechniqueType::Thrust },
            crate::resources::MeyerNode { x: 0.66, y: -0.66, technique: crate::resources::TechniqueType::Thrust },
            crate::resources::MeyerNode { x: -0.66, y: -0.66, technique: crate::resources::TechniqueType::Thrust },
            crate::resources::MeyerNode { x: 0.66, y: 0.66, technique: crate::resources::TechniqueType::Thrust },
        ],
    },
    // Inner-Mid Sequence
    crate::resources::MeyerSequence {
        nodes: [
            crate::resources::MeyerNode { x: -0.33, y: 0.33, technique: crate::resources::TechniqueType::Parry },
            crate::resources::MeyerNode { x: 0.33, y: -0.33, technique: crate::resources::TechniqueType::Parry },
            crate::resources::MeyerNode { x: -0.33, y: -0.33, technique: crate::resources::TechniqueType::Parry },
            crate::resources::MeyerNode { x: 0.33, y: 0.33, technique: crate::resources::TechniqueType::Parry },
        ],
    },
    // Inner Sequence
    crate::resources::MeyerSequence {
        nodes: [
            crate::resources::MeyerNode { x: -0.1, y: 0.1, technique: crate::resources::TechniqueType::Cut },
            crate::resources::MeyerNode { x: 0.1, y: -0.1, technique: crate::resources::TechniqueType::Cut },
            crate::resources::MeyerNode { x: -0.1, y: -0.1, technique: crate::resources::TechniqueType::Cut },
            crate::resources::MeyerNode { x: 0.1, y: 0.1, technique: crate::resources::TechniqueType::Cut },
        ],
    },
];

// --- Kinetic Brutalism Color Palette ---

/// Background: The primary void.
pub const BACKGROUND_COLOR: Color = Color::srgb(0.075, 0.075, 0.075); // #131313

/// Surface Container Low: The Telemetry Rail (Sidebar) base.
pub const SURFACE_LOW: Color = Color::srgb(0.118, 0.118, 0.118); // #1e1e1e

/// Neutral Text / Inactive Signal: Reserved for technical data, labels, and inactive targets.
pub const NEUTRAL_TEXT: Color = Color::srgb(0.886, 0.886, 0.886); // #e2e2e2

/// Primary Active Signal: High-intensity HDR value for #ff5540.
///
/// This color is used for active combat targets and critical states. It must be treated as a light source.
pub const PRIMARY_EMISSIVE: Color = Color::linear_rgb(20.0, 3.0, 1.0); // Intense glowing red/orange

/// Ghost Border: 20% opacity neutral border for functional boundaries.
pub const GHOST_BORDER: Color = Color::srgba(0.886, 0.886, 0.886, 0.2); // #e2e2e2 at 20%

// --- Curriculum Manifest ---
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Statically defines the available curriculum grades and their corresponding documents.
///
/// This structure mirrors the `assets/Grades Escrime` folder to allow the application
/// to dynamically populate UI dropdowns or lists without needing to read the file system
/// directly at runtime.
///
/// Format: `HashMap<GradeName, Vec<(DocumentName, PageCount)>>`
pub static CURRICULUM_MANIFEST: Lazy<HashMap<&'static str, Vec<(&'static str, usize)>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // 14 pages (01 to 13)
    m.insert("Niveau 1.1", vec![("Passage de Grade 1.1", 13)]);
    // 11 pages (01 to 10)
    m.insert("Niveau 1.2", vec![("Passage de grade 1.2", 10)]);
    m
});
