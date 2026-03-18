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
        .insert_resource(RhythmState {
            duration: 1.0,
            mode: RhythmMode::Constant,
            accelerate_counter: 0,
        })
        .insert_resource(SessionStats::default());
}

fn main() {
    let mut app = App::new();
    initialize_app(&mut app);
    app.run();
}
