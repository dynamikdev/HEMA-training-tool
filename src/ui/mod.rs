//! User interface module for the HEMA Training Tool.
//!
//! This module defines the `UiPlugin`, which handles the creation and management
//! of the visual elements of the application, including the training target layout
//! and the configuration sidebar.

pub mod curriculum;
pub mod settings;
pub mod setup;
#[cfg(test)]
mod setup_test;
#[cfg(test)]
mod curriculum_test;
pub mod systems;
#[cfg(test)]
mod systems_test;
pub mod target;

use bevy::prelude::*;
use bevy_ui_widgets::SliderPlugin;

use setup::setup;
use systems::*;
use curriculum::{
    curriculum_keyboard_navigation, load_curriculum_pages, toggle_curriculum_visibility,
    update_curriculum_image, ensure_curriculum_aspect_ratio,
};

/// Plugin that initializes and manages the training tool's user interface.
///
/// This plugin adds the `SliderPlugin` for configuration controls and schedules
/// the initialization and update systems for the target and settings panel.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SliderPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    sequence_control_button_system,
                    curriculum_toggle_system,
                    curriculum_grade_system,
                    curriculum_document_system,
                    curriculum_page_button_system,
                    sync_curriculum_ui_labels,
                    mode_toggle_system,
                    rhythm_mode_toggle_system,
                    style_slider_system,
                    update_rhythm_from_slider,
                    update_circle_layout,
                    render_glowing_arrow,
                    sync_target_visuals,
                    toggle_curriculum_visibility,
                    curriculum_keyboard_navigation,
                    load_curriculum_pages,
                    update_curriculum_image,
                    ensure_curriculum_aspect_ratio,
                ),
            );
    }
}
