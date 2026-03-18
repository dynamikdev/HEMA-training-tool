//! UI implementation for the HEMA Training Tool, including the configuration panel and circular target layout.

pub mod settings;
pub mod setup;
pub mod systems;
pub mod target;

use bevy::prelude::*;
use bevy_ui_widgets::SliderPlugin;

use setup::setup;
use systems::*;

/// Plugin that initializes and manages the training tool's user interface.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SliderPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    sequence_control_button_system,
                    mode_toggle_system,
                    rhythm_mode_toggle_system,
                    style_slider_system,
                    manual_slider_interaction,
                    update_rhythm_from_slider,
                    update_circle_layout,
                ),
            );
    }
}
