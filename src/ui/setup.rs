//! Orchestrates the initial creation and layout of the HEMA training tool's user interface.
//!
//! This module defines the high-level setup routine that initializes the 2D camera,
//! the settings sidebar, and the target display circle.

use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::render::view::Hdr;
use bevy::prelude::*;

use super::settings::spawn_settings_panel;
use super::target::spawn_target_circle;

/// The primary entry point for UI initialization.
///
/// This system is typically called during the `Startup` schedule. It initializes:
/// 1. The 2D rendering pipeline (camera).
/// 2. The main layout container (root node).
/// 3. The interactive settings panel.
/// 4. The circular training target interface.
pub fn setup(mut commands: Commands) {
    spawn_camera(&mut commands);

    // UI Root Node: Full screen container.
    // We use FlexEnd to push the settings panel to the right side of the screen,
    // leaving the central area clear for the target circle.
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexEnd,
            ..default()
        })
        .with_children(|parent| {
            spawn_settings_panel(parent);
        });

    spawn_target_circle(&mut commands);
}

/// Configures and spawns the main 2D camera.
///
/// We enable Bloom and TonyMcMapface tonemapping to provide a high-quality,
/// "glowing" aesthetic for the highlighted targets, improving visibility and
/// visual feedback during training.
fn spawn_camera(commands: &mut Commands) {
    commands.spawn((
        Camera2d::default(),
        Hdr,
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: 0.4,
            ..default()
        },
    ));
}
