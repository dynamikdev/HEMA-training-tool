use bevy::prelude::*;
use std::f32::consts::PI;

use crate::components::NumberIndex;
use crate::constants::{CIRCLE_RADIUS, LABELS};

/// Spawns the circular layout of target numbers.
pub fn spawn_target_circle(commands: &mut Commands) {
    let text_font = TextFont {
        font_size: 70.0,
        ..default()
    };
    let text_color = TextColor(Color::WHITE);
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
