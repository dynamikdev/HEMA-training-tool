//! Marker components for the HEMA Training Tool.

use bevy::prelude::*;

/// Marker for the target numbers, storing their index (0-7).
#[derive(Component)]
pub struct NumberIndex(pub u8);

/// Marker for the button that starts/stops the training sequence.
#[derive(Component)]
pub struct SequenceControlButton;

/// Marker for the button that toggles between Random and Ordered sequence modes.
#[derive(Component)]
pub struct SequenceModeButton;

/// Marker for the button that toggles between Constant and Accelerate rhythm modes.
#[derive(Component)]
pub struct RhythmModeButton;

/// Marker for the text displaying the current rhythm duration.
#[derive(Component)]
pub struct RhythmText;
