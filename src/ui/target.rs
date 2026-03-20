//! Target visualization for the HEMA Training Tool.
//!
//! This module handles the creation and initial positioning of the training targets
//! arranged in a circular pattern, representing the cutting directions of Meyer's Square.

use bevy::prelude::*;
use std::f32::consts::PI;

use crate::components::NumberIndex;
use crate::constants::{CIRCLE_RADIUS, LABELS, NEUTRAL_TEXT};
use crate::resources::Typography;

/// Spawns the circular layout of target numbers in the 2D world.
///
/// Each target is represented by a `Text2d` entity with a `NumberIndex` component,
/// allowing systems to identify and highlight specific targets during the sequence.
/// The initial positions are calculated based on a circle with `CIRCLE_RADIUS`.
pub fn spawn_target_circle(commands: &mut Commands, typography: &Typography) {
    let text_font = TextFont {
        font: typography.space_grotesk.clone(),
        font_size: 70.0,
        ..default()
    };
    let text_color = TextColor(NEUTRAL_TEXT);
    for i in 0..=7 {
        // Calculation centered at (0,0), later refined by update_circle_layout.
        let angle = PI / 2.0 - (i as f32) * (PI / 4.0);
        let x = angle.cos() * CIRCLE_RADIUS;
        let y = angle.sin() * CIRCLE_RADIUS;

        commands.spawn((
            Text2d::new(format!("{}", LABELS[i as usize])),
            text_font.clone(),
            text_color,
            Transform::from_xyz(x, y, 0.0),
            NumberIndex(i),
        ));
    }
}
