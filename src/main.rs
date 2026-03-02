//! HEMA Training Tool
//!
//! A Bevy-powered application designed to help HEMA (Historical European Martial Arts) practitioners
//! train their reaction time and target recognition by highlighting numbers in a circular layout.

mod components;
mod constants;
mod logic;
mod resources;
mod ui;

use bevy::prelude::*;
use bevy_ui_widgets::slider_self_update;

use logic::TrainingPlugin;
use resources::*;
use ui::UiPlugin;

/// Application entry point. Sets up the Bevy app, resources, and plugins.
fn main() {
    App::new()
        // Add default Bevy plugins (window, input, rendering, etc.)
        .add_plugins(DefaultPlugins)
        // Initialize global resources with default training settings.
        .insert_resource(HighlightTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(CurrentNumber(1))
        .insert_resource(SequenceState {
            running: false,
            mode: SequenceMode::Random,
            current_ordered_value: 0,
        })
        .insert_resource(RhythmState {
            duration: 1.0,
            mode: RhythmMode::Constant,
            accelerate_counter: 0,
        })
        // Add custom game logic and UI plugins.
        .add_plugins(TrainingPlugin)
        .add_plugins(UiPlugin)
        // Self-update the SliderValue component upon dragging (external widget logic).
        .add_observer(slider_self_update)
        .run();
}
