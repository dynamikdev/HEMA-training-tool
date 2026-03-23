//! Orchestrates the initial creation and layout of the HEMA training tool's user interface.
//!
//! This module defines the high-level setup routine that initializes the 2D camera,
//! the settings sidebar, and the target display circle.

use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::render::view::Hdr;
use bevy::prelude::*;

use crate::constants::BACKGROUND_COLOR;
use crate::resources::Typography;
use super::settings::spawn_settings_panel;
use super::target::spawn_target_circle;

/// The primary entry point for UI initialization.
///
/// This system is typically called during the `Startup` schedule. It initializes:
/// 1. The 2D rendering pipeline (camera).
/// 2. The main layout container (root node).
/// 3. The interactive settings panel.
/// 4. The circular training target interface.
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut typography: ResMut<Typography>,
) {
    // Load Kinetic Brutalism fonts.
    typography.space_grotesk = asset_server.load("fonts/SpaceGrotesk-Bold.ttf");
    typography.work_sans = asset_server.load("fonts/WorkSans-Medium.ttf");

    spawn_camera(&mut commands);

    // Active Canvas: 2D World Background Layer.
    // We spawn a large sprite or a clear background to ensure the 2D world
    // is visible and not occluded by the UI root.
    commands.spawn((
        Name::new("Active Canvas Background"),
        Sprite {
            color: BACKGROUND_COLOR,
            custom_size: Some(Vec2::new(10000.0, 10000.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0), // Far back in the 2D world
    ));

    // UI Root Node: Full screen container.
    // We use Transparent background for the root to allow 2D world to show through.
    commands
        .spawn((
            Name::new("UI Root"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            spawn_settings_panel(parent, &typography);
        });

    spawn_target_circle(&mut commands, &typography);
}

/// Configures and spawns the main 2D camera.
///
/// We enable Bloom and TonyMcMapface tonemapping to provide a high-quality,
/// "glowing" aesthetic for the highlighted targets, improving visibility and
/// visual feedback during training.
fn spawn_camera(commands: &mut Commands) {
    commands.spawn((
        Camera2d,
        Hdr,
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: 0.4,
            ..default()
        },
    ));
}
