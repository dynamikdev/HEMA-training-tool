use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

use super::settings::spawn_settings_panel;
use super::target::spawn_target_circle;
use crate::constants::PANEL_WIDTH;

/// Initial setup for the UI, including the camera, the settings panel, and the target numbers.
pub fn setup(mut commands: Commands) {
    spawn_camera(&mut commands);

    // UI Root Node: Full screen container.
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexEnd, // Push children to the right side.
            ..default()
        })
        .with_children(|parent| {
            spawn_settings_panel(parent);
        });

    spawn_target_circle(&mut commands);
}

/// Spawns the 2D camera with Bloom effect.
fn spawn_camera(commands: &mut Commands) {
    commands.spawn((
        Camera2d::default(),
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        // Offset the camera so (0,0) is centered in the space remaining after the panel.
        Transform::from_xyz(PANEL_WIDTH / 2.0, 0.0, 0.0),
    ));
}
