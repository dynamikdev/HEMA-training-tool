//! Entry point for the HEMA Training Tool.
//!
//! This crate provides a Bevy-based application for practicing Meyer's Square (Carré Meyer)
//! and other HEMA cutting patterns. It features a customizable target layout,
//! sequence modes (Random/Ordered), and adjustable rhythms (Constant/Accelerating).

use bevy::prelude::*;

#[cfg(test)]
mod main_test;

mod components;
#[cfg(test)]
mod components_test;
mod constants;
mod logic;
#[cfg(test)]
mod logic_test;
mod resources;
#[cfg(test)]
mod resources_test;
mod ui;

use logic::TrainingPlugin;
use resources::*;
use ui::UiPlugin;

/// Initializes and configures the HEMA Training Tool application.
///
/// This function sets up the default plugins, application-specific plugins (`UiPlugin`, `TrainingPlugin`),
/// and initializes the required resources for state management, including timers,
/// sequence tracking, and rhythm configuration.
fn initialize_app(app: &mut App) {
    #[cfg(not(test))]
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "HEMA Training Target Tool".into(),
            ..default()
        }),
        ..default()
    }));

    app.add_plugins(UiPlugin)
        .add_plugins(TrainingPlugin)
        .insert_resource(HighlightTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(CurrentNumber(0))
        .insert_resource(SequenceState {
            running: false,
            mode: SequenceMode::Random,
            current_ordered_value: 0,
        })
        .insert_resource(ActiveWorkflow::default())
        .insert_resource(MeyerTrainingResource::default())
        .insert_resource(RhythmState {
            duration: 1.0,
            mode: RhythmMode::Constant,
            accelerate_counter: 0,
        })
        .insert_resource(ArrowTarget::default())
        .insert_resource(ArrowAnimationState::default())
        .insert_resource(Typography::default())
        .insert_resource(CurriculumState::default());
}

/// The main entry point of the application.
///
/// It creates a new Bevy `App`, initializes it via `initialize_app`, and starts the event loop.
fn main() {
    let mut app = App::new();
    initialize_app(&mut app);
    app.run();
}
